#!/usr/bin/env python3
import random
import copy

class PhiaLuaGenerator:
    def __init__(self, seed):
        self.r = random.Random(seed)
        self.lines = []
        self.indent_level = 0
        self.var_id = 0

        # Scoped memory tracking
        self.scalars = {'int': [], 'float': [], 'str': [], 'bool': []}
        # Maps table name -> element type (e.g., 't0': 'int')
        self.tables = {}

        # Keep track of top-level variables so we can print them at the end
        self.sink_vars = []

    def emit(self, line):
        self.lines.append("  " * self.indent_level + line)

    def fresh_var(self):
        v = f"v{self.var_id}"
        self.var_id += 1
        return v

    def push_scope(self):
        return copy.deepcopy(self.scalars), copy.deepcopy(self.tables)

    def pop_scope(self, state):
        self.scalars, self.tables = state

    # --- Literals ---
    def gen_literal(self, typ):
        if typ == 'int':
            return str(self.r.randint(-20, 20))
        elif typ == 'float':
            return str(round(self.r.uniform(-20.0, 20.0), 2))
        elif typ == 'str':
            word = self.r.choice(["alpha", "beta", "gamma", "delta", "echo"])
            return f'"{word}"'
        elif typ == 'bool':
            return self.r.choice(["true", "false"])

    def get_value(self, typ):
        """Returns either a tracked variable of the requested type or a literal."""
        # 50% chance to use an existing variable, if one exists
        if self.r.random() < 0.5 and self.scalars[typ]:
            return self.r.choice(self.scalars[typ])
        return self.gen_literal(typ)

    # --- Expressions ---
    def gen_expr(self, typ, allow_table_read=True):
        """Generates a valid type-safe expression."""
        # 20% chance to read from a table, if allowed and a table exists
        valid_tables = [t for t, t_typ in self.tables.items() if t_typ == typ]
        if allow_table_read and valid_tables and self.r.random() < 0.2:
            t = self.r.choice(valid_tables)
            idx = self.r.randint(0, 5)  # Keep keys dense
            return f"{t}[{idx}]"

        if typ == 'int' or typ == 'float':
            if self.r.random() < 0.3:
                return self.get_value(typ)

            left = self.get_value(typ)
            op = self.r.choice(['+', '-', '*', '//', '%']) if typ == 'int' else self.r.choice(['+', '-', '*', '/', '//', '%'])

            # Prevent div-by-zero by forcing RHS to be a safe literal
            if op in ['/', '//', '%']:
                rhs = str(self.r.choice([2, 3, 4, 5, -2, -3]))
                if typ == 'float': rhs += ".0"
            else:
                rhs = self.get_value(typ)

            return f"({left} {op} {rhs})"

        elif typ == 'str':
            if self.r.random() < 0.5:
                return f"({self.get_value('str')} .. {self.get_value('str')})"
            return self.get_value('str')

        elif typ == 'bool':
            k = self.r.randrange(3)
            if k == 0: # Unary
                return f"(not {self.get_value('bool')})"
            elif k == 1: # Eq/Neq (Any type)
                cmp_typ = self.r.choice(['int', 'float', 'str', 'bool'])
                op = self.r.choice(['==', '~='])
                return f"({self.get_value(cmp_typ)} {op} {self.get_value(cmp_typ)})"
            else: # Relational (Numbers only)
                cmp_typ = self.r.choice(['int', 'float'])
                op = self.r.choice(['<', '>', '<=', '>='])
                return f"({self.get_value(cmp_typ)} {op} {self.get_value(cmp_typ)})"

    # --- Statements ---
    def gen_local_decl(self):
        typ = self.r.choice(['int', 'float', 'str', 'bool'])
        v = self.fresh_var()
        expr = self.gen_expr(typ)
        self.emit(f"local {v} = {expr}")
        self.scalars[typ].append(v)
        if self.indent_level == 0:
            self.sink_vars.append(v)

    def gen_assignment(self):
        # Pick a random initialized type that has variables
        avail = [t for t in ['int', 'float', 'str', 'bool'] if self.scalars[t]]
        if not avail:
            return self.gen_local_decl()

        typ = self.r.choice(avail)
        v = self.r.choice(self.scalars[typ])
        expr = self.gen_expr(typ)
        self.emit(f"{v} = {expr}")

    def gen_table_decl(self):
        # Bools cannot be stored in tables in phia
        typ = self.r.choice(['int', 'float', 'str'])
        t = self.fresh_var()
        self.emit(f"local {t} = {{}}")
        # Monomorphic lock: write index 0 to lock the type immediately
        expr = self.gen_expr(typ, allow_table_read=False)
        self.emit(f"{t}[0] = {expr}")
        self.tables[t] = typ

    def gen_table_write(self):
        if not self.tables:
            return self.gen_table_decl()

        t = self.r.choice(list(self.tables.keys()))
        typ = self.tables[t]
        idx = self.r.randint(0, 5) # Dense integers
        expr = self.gen_expr(typ)
        self.emit(f"{t}[{idx}] = {expr}")

    def gen_if(self):
        cond = self.gen_expr('bool')
        self.emit(f"if {cond} then")
        self.indent_level += 1

        state = self.push_scope()
        for _ in range(self.r.randint(1, 3)): self.gen_statement()
        self.pop_scope(state)
        self.indent_level -= 1

        if self.r.random() < 0.5:
            self.emit("else")
            self.indent_level += 1
            state = self.push_scope()
            for _ in range(self.r.randint(1, 3)): self.gen_statement()
            self.pop_scope(state)
            self.indent_level -= 1

        self.emit("end")

    def gen_while(self):
        """Generates a safe while loop bounded by a deterministic integer."""
        v_iter = self.fresh_var()
        limit = self.r.randint(2, 5)
        self.emit(f"local {v_iter} = 0")
        self.emit(f"while {v_iter} < {limit} do")
        self.indent_level += 1

        state = self.push_scope()
        # Ensure we don't accidentally shadow the iterator
        self.scalars['int'].append(v_iter)
        for _ in range(self.r.randint(1, 3)):
            # Force table writes inside loops to stress arena bounds checking
            if self.r.random() < 0.7: self.gen_table_write()
            else: self.gen_statement()

        self.pop_scope(state)
        # Advance iterator
        self.emit(f"{v_iter} = {v_iter} + 1")
        self.indent_level -= 1
        self.emit("end")

    def gen_statement(self):
        choices = [
            (self.gen_local_decl, 0.3),
            (self.gen_assignment, 0.2),
            (self.gen_table_decl, 0.15),
            (self.gen_table_write, 0.15),
            (self.gen_if, 0.15),
            (self.gen_while, 0.05),
        ]
        func = self.r.choices([c[0] for c in choices], weights=[c[1] for c in choices], k=1)[0]
        func()

    def generate(self):
        # 1. Initialize at least one of every type in root scope
        for t in ['int', 'float', 'str', 'bool']:
            self.emit(f"local root_{t} = {self.gen_literal(t)}")
            self.scalars[t].append(f"root_{t}")
            self.sink_vars.append(f"root_{t}")

        # 2. Initialize a couple of tables
        for _ in range(2): self.gen_table_decl()

        # 3. Generate random combinatorial AST statements
        for _ in range(self.r.randint(10, 20)):
            self.gen_statement()

        # 4. Final print sink (take up to 10 top-level variables)
        sink = ", ".join(self.sink_vars[:10])
        self.emit(f'print("final", {sink})')

        return "\n".join(self.lines) + "\n"

# Quick test if run directly
if __name__ == "__main__":
    gen = PhiaLuaGenerator(seed=42)
    print(gen.generate())
