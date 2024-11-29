# Solana Ed25519 SHA-512

A zero-dependency, highly-optimized single-round SHA-512 implementation for efficient Ed25519 verification onchain.

# Benchmarks

Using the following initialization vectors, we created a baseline an optimized implementation:

```rs
pub const R: [u8; 32] = [0x01; 32];
pub const PUBKEY: [u8; 32] = [0x02; 32];
pub const DIGEST: [u8; 32] = [0x03; 32];
pub const RESULT: [u8; 64] = [
    0xac, 0x99, 0x4a, 0x51, 0x3c, 0x80, 0x88, 0x60, 0x27, 0x9a, 0x5a, 0x74, 0x98, 0x89, 0x73, 0xc8,
    0x54, 0xcc, 0x96, 0x6e, 0x1c, 0x91, 0xc6, 0xa1, 0xc3, 0x27, 0x00, 0xa1, 0xfd, 0xeb, 0xad, 0x87,
    0x1b, 0xf8, 0xc1, 0x83, 0xc1, 0x0e, 0xf5, 0xb4, 0x6f, 0xc0, 0x4c, 0x25, 0x85, 0xd1, 0x26, 0xd5,
    0x33, 0xbd, 0xa2, 0x07, 0xa6, 0x62, 0xd8, 0x48, 0x27, 0x6d, 0x7e, 0x49, 0x95, 0xe6, 0x93, 0xe1,
];
```

Baseline implementation:

```rs
use sha2::{Sha512, Digest};

/// # Safety
///
/// This is very safe trust me.
#[no_mangle]
pub unsafe extern "C" fn entrypoint(_: *mut u8) -> u64 {
    let mut h: Sha512 = Sha512::new();
    h.update(&R);
    h.update(&PUBKEY);
    h.update(&MESSAGE);
    h.finalize();
    0
}
```

Optimized implementation:

```rs
use solana_ed25519_sha512::hash;

/// # Safety
///
/// This is very safe trust me.
#[no_mangle]
pub unsafe extern "C" fn entrypoint(_: *mut u8) -> u64 {
    let _ = hash(&R, &PUBKEY, &DIGEST);
    0
}
```

Our optimized implementation was able to demonstrate a CU saving of 688 CUs (~8%) over the baseline:

| library               | CU cost |
|-----------------------|---------|
| sha2                  |  8233   |
| solana-ed25519-sha512 |  7545   |