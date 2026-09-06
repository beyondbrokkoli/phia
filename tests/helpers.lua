os = require("os")
io = require("io")

-- Paths to the compiled binary and the temporary output files.
-- Storing standard out (OUT) and standard error (ERR/BERR) in files
-- allows us to parse them for assertions after the process finishes.
BIN = "target/release/phia"
OUT = "target/run_out.txt"
ERR = "target/run_err.txt"
BERR = "target/build_err.txt"
EXAMPLES_DIR = "tests/examples"

-- Reads an entire file into a single string.
-- "*a" instructs io.read to read everything from the current position to EOF.
function read_file(path)
    local f = io.open(path, "r")
    if not f then return "" end
    local content = f:read("*a")
    f:close()
    return content
end

-- Checks if a specific string exists exactly as a whole line in the output.
-- Equivalent to bash: grep -qx "target" text
function has_exact_line(text, target)
    -- The pattern "([^\r\n]+)" splits the text into lines.
    -- ^ means "not". \r and \n are carriage return and newline.
    -- + means "one or more".
    -- Together: "Match sequences of characters that are not line breaks."
    for line in text:gmatch("([^\r\n]+)") do
        if line == target then return true end
    end
    return false
end

-- Parses a test file to find expected output defined in comments.
function parse_expects(filepath, prefix)
    -- prefix will be "EXPECT", "EXPECT_BUILD_FAIL", or "EXPECT_PANIC"
    local expects = {}
    local content = read_file(filepath)

    -- Pattern breakdown:
    -- %-       : Matches a literal hyphen (escapes the special '-' character in Lua).
    -- %-%-     : Matches "--" (Lua comment start).
    -- %s*      : Matches zero or more whitespace characters.
    -- prefix:  : Matches prefix holding the literal string.
    -- ( ... )  : Captures the matched sequence inside the parentheses to return it.
    -- [^\r\n]+ : Matches everything up to the end of the line.
    local pattern = "%-%-%s*" .. prefix .. ":%s*([^\r\n]+)"
    for expected_str in content:gmatch(pattern) do
        table.insert(expects, expected_str)
    end
    return expects
end
