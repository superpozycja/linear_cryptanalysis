use std::ops::Shl;
use std::ops::Shr;

/* this can be side channeled i think */
static s_box: [u8; 16] = 
    [0xe, 0x4, 0xd, 0x1, 0x2, 0xf, 0xb, 0x8,
     0x3, 0xa, 0x6, 0xc, 0x5, 0x9, 0x0, 0x7];

fn sub_byte(x: u8) -> u8 {
    let mut res: u8 = 0;
    res += s_box[(x & 0x0f) as usize];
    let x = x >> 4;
    res += s_box[(x & 0x0f) as usize] << 4;

    res
}

fn xor(x: &mut [u8; 2], y: &[u8; 2]) {
    x[0] ^= y[0];
    x[1] ^= y[1];
}

fn sub(x: &mut [u8; 2]) {
    x[0] = sub_byte(x[0]);
    x[1] = sub_byte(x[1]);
}

pub fn encrypt(pt: &[u8; 2], key: &[u8; 2]) -> [u8; 2] {
    println!("encrypting");
    let mut ct = pt.clone();
    println!("{:#04x}{:02x}", ct[0], ct[1]);
    xor(&mut ct, key);
    println!("{:#04x}{:02x}", ct[0], ct[1]);
    sub(&mut ct);
    ct
}
