// src/fnv.rs — FNV-1a 64. Shared by build.rs (stamp the source content hash
// at build time) and main.rs (verify it at run time). Included into the
// build script via #[path], so both sides hash identically by construction.

pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

pub fn hex(bytes: &[u8]) -> String {
    format!("{:016x}", fnv1a64(bytes))
}
