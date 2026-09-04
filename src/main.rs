// src/main.rs
mod memory;

mod aot {
    include!(concat!(env!("OUT_DIR"), "/baked_native.rs"));
}

fn main() {
    let t0 = std::time::Instant::now();
    let tables = aot::run_baked();
    let elapsed = t0.elapsed();

    for (id, t) in tables.iter().enumerate() {
        let (mut nz, mut ck) = (0u64, 0i64);
        for (i, v) in t.array.iter().enumerate() {
            if *v != 0 { nz += 1; }
            ck = ck.wrapping_add((i as i64 + 1).wrapping_mul(*v)); // position-weighted
        }
        println!("TABLE {id} LEN {} NZ {nz} CHECKSUM {ck}", t.array.len());
    }
    println!("STATS {}", aot::STATS);
    println!("TIME {:?}", elapsed);
}
