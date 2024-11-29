use core::mem::MaybeUninit;

#[inline(always)]
const fn rotr(x: u64, n: u32) -> u64 {
    (x >> n) | (x << (64 - n))
}

#[inline(always)]
const fn gamma0(x: u64) -> u64 {
    rotr(x, 1) ^ rotr(x, 8) ^ (x >> 7)
}

#[inline(always)]
const fn gamma1(x: u64) -> u64 {
    rotr(x, 19) ^ rotr(x, 61) ^ (x >> 6)
}

#[inline(always)]
const fn ch(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (!x & z)
}

#[inline(always)]
const fn maj(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[inline(always)]
const fn sigma0(x: u64) -> u64 {
    rotr(x, 28) ^ rotr(x, 34) ^ rotr(x, 39)
}

#[inline(always)]
const fn sigma1(x: u64) -> u64 {
    rotr(x, 14) ^ rotr(x, 18) ^ rotr(x, 41)
}

#[inline(always)]
pub fn hash(r: &[u8; 32], pubkey: &[u8; 32], digest: &[u8; 32]) -> [u8; 64] {
    let mut words = MaybeUninit::<[u64; 80]>::uninit();
    let w = unsafe { words.assume_init_mut() };
    w[0] = u64::from_be_bytes([r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7]]);
    w[1] = u64::from_be_bytes([r[8], r[9], r[10], r[11], r[12], r[13], r[14], r[15]]);
    w[2] = u64::from_be_bytes([r[16], r[17], r[18], r[19], r[20], r[21], r[22], r[23]]);
    w[3] = u64::from_be_bytes([r[24], r[25], r[26], r[27], r[28], r[29], r[30], r[31]]);
    w[4] = u64::from_be_bytes([
        pubkey[0], pubkey[1], pubkey[2], pubkey[3], pubkey[4], pubkey[5], pubkey[6], pubkey[7],
    ]);
    w[5] = u64::from_be_bytes([
        pubkey[8], pubkey[9], pubkey[10], pubkey[11], pubkey[12], pubkey[13], pubkey[14],
        pubkey[15],
    ]);
    w[6] = u64::from_be_bytes([
        pubkey[16], pubkey[17], pubkey[18], pubkey[19], pubkey[20], pubkey[21], pubkey[22],
        pubkey[23],
    ]);
    w[7] = u64::from_be_bytes([
        pubkey[24], pubkey[25], pubkey[26], pubkey[27], pubkey[28], pubkey[29], pubkey[30],
        pubkey[31],
    ]);
    w[8] = u64::from_be_bytes([
        digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7],
    ]);
    w[9] = u64::from_be_bytes([
        digest[8], digest[9], digest[10], digest[11], digest[12], digest[13], digest[14],
        digest[15],
    ]);
    w[10] = u64::from_be_bytes([
        digest[16], digest[17], digest[18], digest[19], digest[20], digest[21], digest[22],
        digest[23],
    ]);
    w[11] = u64::from_be_bytes([
        digest[24], digest[25], digest[26], digest[27], digest[28], digest[29], digest[30],
        digest[31],
    ]);
    w[12] = 0x8000000000000000; // Padding bit
    w[13] = 0x0000000000000000; // Explicity set this
    w[14] = 0x0000000000000000; // And this
    w[15] = 0x0000000000000300;
    for i in 16..80 {
        w[i] = w[i - 16]
            .wrapping_add(gamma0(w[i - 15]))
            .wrapping_add(w[i - 7])
            .wrapping_add(gamma1(w[i - 2]));
    }

    // Round 0
    let mut t1 = 0x5be0cd19137e2179u64
        .wrapping_add(sigma1(0x510e527fade682d1u64))
        .wrapping_add(ch(
            0x510e527fade682d1u64,
            0x9b05688c2b3e6c1fu64,
            0x1f83d9abfb41bd6bu64,
        ))
        .wrapping_add(0x428A2F98D728AE22)
        .wrapping_add(w[0]);
    let mut t2 = 0x4334C1BEA164F555;
    let mut h = 0x1f83d9abfb41bd6bu64;
    let mut g = 0x9b05688c2b3e6c1fu64;
    let mut f = 0x510e527fade682d1u64;
    let mut e = 0xa54ff53a5f1d36f1u64.wrapping_add(t1);
    let mut d = 0x3c6ef372fe94f82bu64;
    let mut c = 0xbb67ae8584caa73bu64;
    let mut b = 0x6a09e667f3bcc908u64;
    let mut a = t1.wrapping_add(t2);

    // Round 1
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x7137449123EF65CD)
        .wrapping_add(w[1]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 2
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xB5C0FBCFEC4D3B2F)
        .wrapping_add(w[2]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 3
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xE9B5DBA58189DBBC)
        .wrapping_add(w[3]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 4
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x3956C25BF348B538)
        .wrapping_add(w[4]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 5
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x59F111F1B605D019)
        .wrapping_add(w[5]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 6
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x923F82A4AF194F9B)
        .wrapping_add(w[6]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 7
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xAB1C5ED5DA6D8118)
        .wrapping_add(w[7]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 8
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xD807AA98A3030242)
        .wrapping_add(w[8]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 9
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x12835B0145706FBE)
        .wrapping_add(w[9]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 10
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x243185BE4EE4B28C)
        .wrapping_add(w[10]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 11
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x550C7DC3D5FFB4E2)
        .wrapping_add(w[11]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 12
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x72BE5D74F27B896F)
        .wrapping_add(w[12]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 13
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x80DEB1FE3B1696B1)
        .wrapping_add(w[13]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 14
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x9BDC06A725C71235)
        .wrapping_add(w[14]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 15
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xC19BF174CF692694)
        .wrapping_add(w[15]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 16
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xE49B69C19EF14AD2)
        .wrapping_add(w[16]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 17
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xEFBE4786384F25E3)
        .wrapping_add(w[17]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 18
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x0FC19DC68B8CD5B5)
        .wrapping_add(w[18]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 19
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x240CA1CC77AC9C65)
        .wrapping_add(w[19]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 20
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x2DE92C6F592B0275)
        .wrapping_add(w[20]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 21
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x4A7484AA6EA6E483)
        .wrapping_add(w[21]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 22
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x5CB0A9DCBD41FBD4)
        .wrapping_add(w[22]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 23
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x76F988DA831153B5)
        .wrapping_add(w[23]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 24
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x983E5152EE66DFAB)
        .wrapping_add(w[24]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 25
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xA831C66D2DB43210)
        .wrapping_add(w[25]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 26
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xB00327C898FB213F)
        .wrapping_add(w[26]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 27
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xBF597FC7BEEF0EE4)
        .wrapping_add(w[27]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 28
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xC6E00BF33DA88FC2)
        .wrapping_add(w[28]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 29
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xD5A79147930AA725)
        .wrapping_add(w[29]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 30
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x06CA6351E003826F)
        .wrapping_add(w[30]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 31
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x142929670A0E6E70)
        .wrapping_add(w[31]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 32
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x27B70A8546D22FFC)
        .wrapping_add(w[32]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 33
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x2E1B21385C26C926)
        .wrapping_add(w[33]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 34
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x4D2C6DFC5AC42AED)
        .wrapping_add(w[34]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 35
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x53380D139D95B3DF)
        .wrapping_add(w[35]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 36
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x650A73548BAF63DE)
        .wrapping_add(w[36]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 37
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x766A0ABB3C77B2A8)
        .wrapping_add(w[37]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 38
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x81C2C92E47EDAEE6)
        .wrapping_add(w[38]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 39
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x92722C851482353B)
        .wrapping_add(w[39]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 40
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xA2BFE8A14CF10364)
        .wrapping_add(w[40]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 41
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xA81A664BBC423001)
        .wrapping_add(w[41]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 42
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xC24B8B70D0F89791)
        .wrapping_add(w[42]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 43
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xC76C51A30654BE30)
        .wrapping_add(w[43]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 44
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xD192E819D6EF5218)
        .wrapping_add(w[44]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 45
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xD69906245565A910)
        .wrapping_add(w[45]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 46
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xF40E35855771202A)
        .wrapping_add(w[46]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 47
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x106AA07032BBD1B8)
        .wrapping_add(w[47]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 48
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x19A4C116B8D2D0C8)
        .wrapping_add(w[48]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 49
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x1E376C085141AB53)
        .wrapping_add(w[49]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 50
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x2748774CDF8EEB99)
        .wrapping_add(w[50]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 51
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x34B0BCB5E19B48A8)
        .wrapping_add(w[51]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 52
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x391C0CB3C5C95A63)
        .wrapping_add(w[52]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 53
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x4ED8AA4AE3418ACB)
        .wrapping_add(w[53]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 54
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x5B9CCA4F7763E373)
        .wrapping_add(w[54]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 55
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x682E6FF3D6B2B8A3)
        .wrapping_add(w[55]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 56
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x748F82EE5DEFB2FC)
        .wrapping_add(w[56]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 57
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x78A5636F43172F60)
        .wrapping_add(w[57]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 58
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x84C87814A1F0AB72)
        .wrapping_add(w[58]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 59
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x8CC702081A6439EC)
        .wrapping_add(w[59]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 60
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x90BEFFFA23631E28)
        .wrapping_add(w[60]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 61
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xA4506CEBDE82BDE9)
        .wrapping_add(w[61]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 62
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xBEF9A3F7B2C67915)
        .wrapping_add(w[62]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 63
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xC67178F2E372532B)
        .wrapping_add(w[63]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 64
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xCA273ECEEA26619C)
        .wrapping_add(w[64]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 65
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xD186B8C721C0C207)
        .wrapping_add(w[65]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 66
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xEADA7DD6CDE0EB1E)
        .wrapping_add(w[66]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 67
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0xF57D4F7FEE6ED178)
        .wrapping_add(w[67]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 68
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x06F067AA72176FBA)
        .wrapping_add(w[68]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 69
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x0A637DC5A2C898A6)
        .wrapping_add(w[69]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 70
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x113F9804BEF90DAE)
        .wrapping_add(w[70]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 71
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x1B710B35131C471B)
        .wrapping_add(w[71]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 72
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x28DB77F523047D84)
        .wrapping_add(w[72]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 73
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x32CAAB7B40C72493)
        .wrapping_add(w[73]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 74
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x3C9EBE0A15C9BEBC)
        .wrapping_add(w[74]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 75
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x431D67C49C100D4C)
        .wrapping_add(w[75]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 76
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x4CC5D4BECB3E42B6)
        .wrapping_add(w[76]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 77
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x597F299CFC657E2A)
        .wrapping_add(w[77]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 78
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x5FCB6FAB3AD6FAEC)
        .wrapping_add(w[78]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Round 79
    t1 = h
        .wrapping_add(sigma1(e))
        .wrapping_add(ch(e, f, g))
        .wrapping_add(0x6C44198C4A475817)
        .wrapping_add(w[79]);
    t2 = sigma0(a).wrapping_add(maj(a, b, c));
    h = g;
    g = f;
    f = e;
    e = d.wrapping_add(t1);
    d = c;
    c = b;
    b = a;
    a = t1.wrapping_add(t2);

    // Final state values
    let state = [
        0x6a09e667f3bcc908u64.wrapping_add(a).to_be_bytes(),
        0xbb67ae8584caa73bu64.wrapping_add(b).to_be_bytes(),
        0x3c6ef372fe94f82bu64.wrapping_add(c).to_be_bytes(),
        0xa54ff53a5f1d36f1u64.wrapping_add(d).to_be_bytes(),
        0x510e527fade682d1u64.wrapping_add(e).to_be_bytes(),
        0x9b05688c2b3e6c1fu64.wrapping_add(f).to_be_bytes(),
        0x1f83d9abfb41bd6bu64.wrapping_add(g).to_be_bytes(),
        0x5be0cd19137e2179u64.wrapping_add(h).to_be_bytes(),
    ];

    // Convert to bytes
    let mut result = [0u8; 64];
    for (i, &word) in state.iter().enumerate() {
        result[i * 8..(i + 1) * 8].copy_from_slice(&word);
    }

    result
}
