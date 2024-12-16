///  Message word schedule permutations for each round of both BLAKE2b and BLAKE2s are defined by SIGMA.
const BLAKE2B_SIGMA: [[u8; 16]; 12] = [
    [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf],
    [0xe, 0xa, 0x4, 0x8, 0x9, 0xf, 0xd, 0x6, 0x1, 0xc, 0x0, 0x2, 0xb, 0x7, 0x5, 0x3],
    [0xb, 0x8, 0xc, 0x0, 0x5, 0x2, 0xf, 0xd, 0xa, 0xe, 0x3, 0x6, 0x7, 0x1, 0x9, 0x4],
    [0x7, 0x9, 0x3, 0x1, 0xd, 0xc, 0xb, 0xe, 0x2, 0x6, 0x5, 0xa, 0x4, 0x0, 0xf, 0x8],
    [0x9, 0x0, 0x5, 0x7, 0x2, 0x4, 0xa, 0xf, 0xe, 0x1, 0xb, 0xc, 0x6, 0x8, 0x3, 0xd],
    [0x2, 0xc, 0x6, 0xa, 0x0, 0xb, 0x8, 0x3, 0x4, 0xd, 0x7, 0x5, 0xf, 0xe, 0x1, 0x9],
    [0xc, 0x5, 0x1, 0xf, 0xe, 0xd, 0x4, 0xa, 0x0, 0x7, 0x6, 0x3, 0x9, 0x2, 0x8, 0xb],
    [0xd, 0xb, 0x7, 0xe, 0xc, 0x1, 0x3, 0x9, 0x5, 0x0, 0xf, 0x4, 0x8, 0x6, 0x2, 0xa],
    [0x6, 0xf, 0xe, 0x9, 0xb, 0x3, 0x0, 0x8, 0xc, 0x2, 0xd, 0x7, 0x1, 0x4, 0xa, 0x5],
    [0xa, 0x2, 0x8, 0x4, 0x7, 0x6, 0x1, 0x5, 0xf, 0xb, 0x9, 0xe, 0x3, 0xc, 0xd, 0x0],
    [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf],
    [0xe, 0xa, 0x4, 0x8, 0x9, 0xf, 0xd, 0x6, 0x1, 0xc, 0x0, 0x2, 0xb, 0x7, 0x5, 0x3],
];

/// The initialization vector constant.
/// IV[i] = floor(2**w * frac(sqrt(prime(i+1)))), where prime(i) is the i:th prime number (2, 3, 5, 7, 11, 13, 17, 19).
const BLAKE2B_IV: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

/// Block bytes.
const BLAKE2B_BB: usize = 128;
/// Hash bytes.
const BLAKE2B_NN: usize = 64;

/// G rotation constants.
const BLAKE2B_R1: u32 = 32;
const BLAKE2B_R2: u32 = 24;
const BLAKE2B_R3: u32 = 16;
const BLAKE2B_R4: u32 = 63;

/// Interpretation of bytes as words. On little endian platforms, rust will automatically optimize this function.
fn interp_hb2w(b: &[u8; BLAKE2B_NN]) -> [u64; BLAKE2B_NN / 8] {
    let mut w = [0; BLAKE2B_NN / 8];
    let mut u = [0; 8];
    for i in 0..w.len() {
        u.copy_from_slice(&b[i * 8..i * 8 + 8]);
        w[i] = u64::from_le_bytes(u)
    }
    w
}

/// Interpretation of words as bytes. On little endian platforms, rust will automatically optimize this function.
fn interp_hw2b(w: &[u64; BLAKE2B_NN / 8]) -> [u8; BLAKE2B_NN] {
    let mut b = [0; BLAKE2B_NN];
    for i in 0..w.len() {
        b[i * 8..i * 8 + 8].copy_from_slice(&w[i].to_le_bytes());
    }
    b
}

/// Interpretation of bytes as words. On little endian platforms, rust will automatically optimize this function.
fn interp_bb2w(b: &[u8; BLAKE2B_BB]) -> [u64; BLAKE2B_BB / 8] {
    let mut w = [0; BLAKE2B_BB / 8];
    let mut u = [0; 8];
    for i in 0..w.len() {
        u.copy_from_slice(&b[i * 8..i * 8 + 8]);
        w[i] = u64::from_le_bytes(u)
    }
    w
}

/// The G primitive function mixes two input words, "x" and "y", into four words indexed by "a", "b", "c", and "d" in
/// the working vector v[0..15].
fn mixing(v: &mut [u64; 16], a: usize, b: usize, c: usize, d: usize, x: u64, y: u64) {
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
    v[d] = (v[d] ^ v[a]).rotate_right(BLAKE2B_R1);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(BLAKE2B_R2);
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
    v[d] = (v[d] ^ v[a]).rotate_right(BLAKE2B_R3);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(BLAKE2B_R4);
}

/// Compression function F takes as an argument the state vector "h", message block vector "m" (last block is padded
/// with zeros to full block size, if required), 2w-bit offset counter "t", and final block indicator flag "f".  Local
/// vector v[0..15] is used in processing. F returns a new state vector. The number of rounds, "r", is 12 for BLAKE2b
/// and 10 for BLAKE2s. Rounds are numbered from 0 to r - 1.
fn reduce(h: &mut [u64; 8], m: &[u64; 16], t: &[u64; 2], f: &[u64; 2]) {
    let mut v = [0x00; 16];
    v[0x00..0x08].copy_from_slice(h);
    v[0x08..0x10].copy_from_slice(&BLAKE2B_IV);
    v[0x0c] ^= t[0];
    v[0x0d] ^= t[1];
    v[0x0e] ^= f[0];
    v[0x0f] ^= f[1];
    for s in BLAKE2B_SIGMA {
        mixing(&mut v, 0x0, 0x4, 0x8, 0xc, m[s[0x0] as usize], m[s[0x1] as usize]);
        mixing(&mut v, 0x1, 0x5, 0x9, 0xd, m[s[0x2] as usize], m[s[0x3] as usize]);
        mixing(&mut v, 0x2, 0x6, 0xa, 0xe, m[s[0x4] as usize], m[s[0x5] as usize]);
        mixing(&mut v, 0x3, 0x7, 0xb, 0xf, m[s[0x6] as usize], m[s[0x7] as usize]);
        mixing(&mut v, 0x0, 0x5, 0xa, 0xf, m[s[0x8] as usize], m[s[0x9] as usize]);
        mixing(&mut v, 0x1, 0x6, 0xb, 0xc, m[s[0xa] as usize], m[s[0xb] as usize]);
        mixing(&mut v, 0x2, 0x7, 0x8, 0xd, m[s[0xc] as usize], m[s[0xd] as usize]);
        mixing(&mut v, 0x3, 0x4, 0x9, 0xe, m[s[0xe] as usize], m[s[0xf] as usize]);
    }
    for i in 0..8 {
        h[i] = h[i] ^ v[i] ^ v[i + 8]
    }
}

/// Add n to message byte offset.
fn incoff(t: &mut [u64; 2], n: u64) {
    t[0] = t[0].wrapping_add(n);
    t[1] = t[1].wrapping_add((t[0] < n) as u64);
}

/// BLAKE2b parameter block structure.
pub struct Param2b {
    buf: [u8; 64],
    key: [u8; 64],
}

impl Param2b {
    /// Set digest byte length. An integer in [1, 64] for BLAKE2b, in [1, 32] for BLAKE2s.
    pub fn digest(&mut self, n: u8) {
        assert!(1 <= n && n <= 64);
        self.buf[0x00] = n;
    }

    /// Set key. Key length in [0, 64] for BLAKE2b, in [0, 32] for BLAKE2s.
    pub fn key(&mut self, n: &[u8]) {
        assert!(n.len() <= 64);
        self.buf[0x01] = n.len() as u8;
        self.key[..n.len()].copy_from_slice(n);
    }

    /// Set salt. An arbitrary string of 16 bytes for BLAKE2b, and 8 bytes for BLAKE2s.
    pub fn salt(&mut self, n: &[u8]) {
        assert!(n.len() <= 16);
        self.buf[0x20..0x20 + n.len()].copy_from_slice(n);
    }

    /// Set personalization. An arbitrary string of 16 bytes for BLAKE2b, and 8 bytes for BLAKE2s.
    pub fn person(&mut self, n: &[u8]) {
        assert!(n.len() <= 16);
        self.buf[0x30..0x30 + n.len()].copy_from_slice(n);
    }
}

/// A context for computing the BLAKE2b checksum.
pub struct Blake2b {
    /// Internal state of the hash.
    h: [u64; 8],
    /// Message byte offset at the end of the current block.
    t: [u64; 2],
    /// Flag indicating the last block.
    f: [u64; 2],
    /// Buffer.
    b: [u8; BLAKE2B_BB],
    /// Buffer length.
    l: usize,
    /// Parameter block.
    p: Param2b,
}

impl Blake2b {
    /// Update this hash object's state with the provided data.
    pub fn update(&mut self, data: &[u8]) {
        let mut dlen = data.len();
        if dlen <= BLAKE2B_BB - self.l {
            self.b[self.l..self.l + dlen].copy_from_slice(data);
            self.l += dlen;
            return;
        }
        let mut doff = 0;
        if self.l != 0 {
            self.b[self.l..].copy_from_slice(&data[..BLAKE2B_BB - self.l]);
            incoff(&mut self.t, BLAKE2B_BB as u64);
            reduce(&mut self.h, &interp_bb2w(&self.b), &self.t, &self.f);
            doff += BLAKE2B_BB - self.l;
            dlen -= BLAKE2B_BB - self.l;
        }
        for _ in 0..(dlen - 1) / BLAKE2B_BB {
            self.b.copy_from_slice(&data[doff..doff + BLAKE2B_BB]);
            incoff(&mut self.t, BLAKE2B_BB as u64);
            reduce(&mut self.h, &interp_bb2w(&self.b), &self.t, &self.f);
            doff += BLAKE2B_BB;
            dlen -= BLAKE2B_BB;
        }
        self.b[..dlen].copy_from_slice(&data[doff..]);
        self.l = dlen;
    }

    /// Return the digest value.
    pub fn digest(&mut self, d: &mut [u8]) {
        self.b[self.l..].fill(0);
        self.f[0] = u64::MAX;
        incoff(&mut self.t, self.l as u64);
        reduce(&mut self.h, &interp_bb2w(&self.b), &self.t, &self.f);
        let br = interp_hw2b(&self.h);
        d.copy_from_slice(&br[..self.p.buf[0] as usize]);
    }
}

/// Create the parameter block of BLAKE2b. All general parameters are supported.
pub fn blake2b_params() -> Param2b {
    let mut r = Param2b { buf: [0; 64], key: [0; 64] };
    r.buf[0x02] = 0x01;
    r.buf[0x03] = 0x01;
    r
}

/// Core hasher state of BLAKE2b.
pub fn blake2b(param2b: Param2b) -> Blake2b {
    let mut r = Blake2b { h: [0; 8], t: [0; 2], f: [0; 2], b: [0; 128], l: 0, p: param2b };
    let w = interp_hb2w(&r.p.buf);
    for i in 0..8 {
        r.h[i] ^= BLAKE2B_IV[i] ^ w[i]
    }
    if r.p.buf[1] != 0 {
        let mut b = [0; BLAKE2B_BB];
        b[..r.p.key.len()].copy_from_slice(&r.p.key);
        r.update(&b);
    }
    r
}
