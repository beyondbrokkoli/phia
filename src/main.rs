// src/main.rs
mod memory;

//use memory::Value;

mod aot {
    include!(concat!(env!("OUT_DIR"), "/baked_native.rs"));
}

fn main() {
    println!("Booting Phia Native Engine (Inlined Memory Workload)...");

    let start_time = std::time::Instant::now();

    // Bind the returned tables to 'tables'
    let tables = aot::run_baked();

    let elapsed = start_time.elapsed();

    println!("--- EXECUTION FINISHED IN {:?} ---", elapsed);

    println!("\n[ ARENA TABLES ]");
    if tables.is_empty() {
        println!("  (No tables allocated)");
    }

    for (arena_id, table) in tables.iter().enumerate() {
        println!("  Table {}:", arena_id);

        for (i, val) in table.array.iter().take(20).enumerate() {
            if *val != 0 {
                // Changed from Value::nil()
                println!("    [{}] = {}", i, val);
            }
        }
        if table.array.len() > 20 {
            println!("    ... ({} more items)", table.array.len() - 20);
        }
    }
}
