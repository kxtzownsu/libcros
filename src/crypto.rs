pub fn crc32(data: &[u8]) -> u32 {
  let mut crc: u32 = 0xFFFFFFFF;
  for &byte in data {
    let mut b = byte as u32;
    for _ in 0..8 {
      if (crc ^ b) & 1 == 1 {
        crc = (crc >> 1) ^ 0xEDB88320;
      } else {
        crc >>= 1;
      }
      b >>= 1;
    }
  }
  !crc
}

pub fn adler32(data: &[u8]) -> u32 {
  const MOD: u32 = 65521;
  let mut a: u32 = 1;
  let mut b: u32 = 0;
  for &byte in data {
    a = (a + byte as u32) % MOD;
    b = (b + a) % MOD;
  }
  (b << 16) | a
}

pub fn sha1(data: &[u8]) -> [u8; 20] {
  fn rot_left(x: u32, n: u32) -> u32 { (x << n) | (x >> (32 - n)) }
  
  let mut h = [0x67452301u32, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
  let mut msg = data.to_vec();
  let bit_len = (data.len() as u64) * 8;
  
  msg.push(0x80);
  while (msg.len() % 64) != 56 { msg.push(0); }
  msg.extend_from_slice(&bit_len.to_be_bytes());
  
  for chunk in msg.chunks_exact(64) {
    let mut w = [0u32; 80];
    for i in 0..16 { w[i] = u32::from_be_bytes([chunk[i*4], chunk[i*4+1], chunk[i*4+2], chunk[i*4+3]]); }
    for i in 16..80 { w[i] = rot_left(w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16], 1); }
    
    let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
    for i in 0..80 {
      let (f, k) = match i {
        0..=19 => ((b & c) | (!b & d), 0x5A827999),
        20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
        40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
        _ => (b ^ c ^ d, 0xCA62C1D6),
      };
      let temp = rot_left(a, 5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
      e = d; d = c; c = rot_left(b, 30); b = a; a = temp;
    }
    h[0] = h[0].wrapping_add(a); h[1] = h[1].wrapping_add(b);
    h[2] = h[2].wrapping_add(c); h[3] = h[3].wrapping_add(d); h[4] = h[4].wrapping_add(e);
  }
  
  let mut out = [0u8; 20];
  for (i, &v) in h.iter().enumerate() { out[i*4..(i+1)*4].copy_from_slice(&v.to_be_bytes()); }
  out
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
  const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
  ];
  fn rot_right(x: u32, n: u32) -> u32 { (x >> n) | (x << (32 - n)) }
  
  let mut h = [0x6a09e667u32, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
  let mut msg = data.to_vec();
  let bit_len = (data.len() as u64) * 8;
  
  msg.push(0x80);
  while (msg.len() % 64) != 56 { msg.push(0); }
  msg.extend_from_slice(&bit_len.to_be_bytes());
  
  for chunk in msg.chunks_exact(64) {
    let mut w = [0u32; 64];
    for i in 0..16 { w[i] = u32::from_be_bytes([chunk[i*4], chunk[i*4+1], chunk[i*4+2], chunk[i*4+3]]); }
    for i in 16..64 {
      let s0 = rot_right(w[i-15], 7) ^ rot_right(w[i-15], 18) ^ (w[i-15] >> 3);
      let s1 = rot_right(w[i-2], 17) ^ rot_right(w[i-2], 19) ^ (w[i-2] >> 10);
      w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
    }
    
    let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h_var) = (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
    for i in 0..64 {
      let s1 = rot_right(e, 6) ^ rot_right(e, 11) ^ rot_right(e, 25);
      let ch = (e & f) ^ (!e & g);
      let temp1 = h_var.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
      let s0 = rot_right(a, 2) ^ rot_right(a, 13) ^ rot_right(a, 22);
      let maj = (a & b) ^ (a & c) ^ (b & c);
      let temp2 = s0.wrapping_add(maj);
      
      h_var = g; g = f; f = e; e = d.wrapping_add(temp1);
      d = c; c = b; b = a; a = temp1.wrapping_add(temp2);
    }
    h[0] = h[0].wrapping_add(a); h[1] = h[1].wrapping_add(b); h[2] = h[2].wrapping_add(c); h[3] = h[3].wrapping_add(d);
    h[4] = h[4].wrapping_add(e); h[5] = h[5].wrapping_add(f); h[6] = h[6].wrapping_add(g); h[7] = h[7].wrapping_add(h_var);
  }
  
  let mut out = [0u8; 32];
  for (i, &v) in h.iter().enumerate() { out[i*4..(i+1)*4].copy_from_slice(&v.to_be_bytes()); }
  out
}

pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
  const BLOCK_SIZE: usize = 64;
  let mut key_block = [0u8; BLOCK_SIZE];
  
  if key.len() > BLOCK_SIZE {
    key_block[..32].copy_from_slice(&sha256(key));
  } else {
    key_block[..key.len()].copy_from_slice(key);
  }
  
  let mut ipad = [0x36u8; BLOCK_SIZE];
  let mut opad = [0x5cu8; BLOCK_SIZE];
  for i in 0..BLOCK_SIZE {
    ipad[i] ^= key_block[i];
    opad[i] ^= key_block[i];
  }
  
  let mut inner = ipad.to_vec();
  inner.extend_from_slice(data);
  let inner_hash = sha256(&inner);
  
  let mut outer = opad.to_vec();
  outer.extend_from_slice(&inner_hash);
  sha256(&outer)
}

pub fn pbkdf2_hmac_sha256(password: &[u8], salt: &[u8], iterations: u32, output_len: usize) -> Vec<u8> {
  let mut result = Vec::with_capacity(output_len);
  let mut block_num = 1u32;
  
  while result.len() < output_len {
    let mut salt_block = salt.to_vec();
    salt_block.extend_from_slice(&block_num.to_be_bytes());
    
    let mut u = hmac_sha256(password, &salt_block);
    let mut f = u;
    
    for _ in 1..iterations {
      u = hmac_sha256(password, &u);
      for i in 0..32 { f[i] ^= u[i]; }
    }
    
    result.extend_from_slice(&f);
    block_num += 1;
  }
  
  result.truncate(output_len);
  result
}