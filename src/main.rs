// src/main.rs
mod memory;
mod fnv;

mod aot {
    include!(concat!(env!("OUT_DIR"), "/baked_native.rs"));
}

mod stamp {
    include!(concat!(env!("OUT_DIR"), "/source_stamp.rs"));
}

fn main() {
    // Staleness guard. Cargo decides whether to rerun build.rs from mtimes;
    // a same-path content swap with an older mtime skips the rebuild and
    // silently runs the PREVIOUS program. Hash the source this binary was
    // built from and compare against the file on disk: a mismatch is never
    // a program error, it is a harness error — fail loud, not wrong.
    // (Deliberately not prefixed "Runtime Error: " so no EXPECT_PANIC pin
    // can ever match it.)
    match std::fs::read(stamp::SOURCE_PATH) {
        Ok(bytes) => {
            if fnv::hex(&bytes) != stamp::SOURCE_HASH {
                panic!(
                    "STALE BINARY: '{}' changed since the last build (content hash mismatch). \
                     Rebuild: PHIA_SOURCE='{}' cargo build --release",
                    stamp::SOURCE_PATH, stamp::SOURCE_PATH
                );
            }
        }
        Err(e) => panic!(
            "STALE BINARY: built source '{}' is unreadable ({}). Rebuild with PHIA_SOURCE set.",
            stamp::SOURCE_PATH, e
        ),
    }

    let t0 = std::time::Instant::now();
    let tables = aot::run_baked();
    let elapsed = t0.elapsed();

    for (id, t) in tables.iter().enumerate() {
        if t.is_string {
            // String tables: same position-weighted checksum formula, with
            // FNV-1a over each element's bytes standing in for the value.
            // Deterministic in content, never in address. NZ counts
            // non-empty elements — the empty string is the pool's zero
            // (absence), exactly like 0 and 0.0.
            let (mut nz, mut ck) = (0u64, 0i64);
            for (i, v) in t.sarray.iter().enumerate() {
                if !v.is_empty() { nz += 1; }
                ck = ck.wrapping_add((i as i64 + 1)
                    .wrapping_mul(fnv::fnv1a64(v.as_bytes()) as i64));
            }
            println!("TABLE {id} LEN {} NZ {nz} CHECKSUM {ck}", t.sarray.len());
        } else if t.is_float {
            // Float tables: bit-pattern checksum (absolutely deterministic,
            // same position-weighted formula as integers) plus a sequential
            // SUM for human-readable pins.
            let (mut nz, mut ck, mut sum) = (0u64, 0i64, 0f64);
            for (i, v) in t.farray.iter().enumerate() {
                if *v != 0.0 { nz += 1; }
                ck = ck.wrapping_add((i as i64 + 1).wrapping_mul(v.to_bits() as i64));
                sum += v;
            }
            println!("TABLE {id} LEN {} NZ {nz} CHECKSUM {ck} SUM {sum}", t.farray.len());
        } else {
            let (mut nz, mut ck) = (0u64, 0i64);
            for (i, v) in t.array.iter().enumerate() {
                if *v != 0 { nz += 1; }
                ck = ck.wrapping_add((i as i64 + 1).wrapping_mul(*v)); // position-weighted
            }
            println!("TABLE {id} LEN {} NZ {nz} CHECKSUM {ck}", t.array.len());
        }
    }
    println!("STATS {}", aot::STATS);
    println!("TIME {:?}", elapsed);
}
