/* this can be side channeled i think */
static S_BOX: [u8; 16] = 
    [0xe, 0x4, 0xd, 0x1, 0x2, 0xf, 0xb, 0x8,
     0x3, 0xa, 0x6, 0xc, 0x5, 0x9, 0x0, 0x7];

#[inline(always)]
fn copy_bit(a: &mut u8, an: u8, b: u8, bn: u8) {
    *a = *a & !(1 << an) | (((b >> bn) & 1) << an);
}

fn sub_byte(x: u8) -> u8 {
    let mut res: u8 = 0;
    res += S_BOX[(x & 0x0f) as usize];
    let x = x >> 4;
    res += S_BOX[(x & 0x0f) as usize] << 4;

    res
}

fn sub(x: &mut [u8; 2]) {
    x[0] = sub_byte(x[0]);
    x[1] = sub_byte(x[1]);
}

fn xor(x: &mut [u8; 2], y: &[u8; 2]) {
    x[0] ^= y[0];
    x[1] ^= y[1];
}

fn per(x: &mut [u8; 2]) {
    let x2 = x.clone();

    copy_bit(&mut x[0], 7, x2[0], 7);
    copy_bit(&mut x[0], 6, x2[0], 3);
    copy_bit(&mut x[0], 5, x2[1], 7);
    copy_bit(&mut x[0], 4, x2[1], 3);

    copy_bit(&mut x[0], 3, x2[0], 6);
    copy_bit(&mut x[0], 2, x2[0], 2);
    copy_bit(&mut x[0], 1, x2[1], 6);
    copy_bit(&mut x[0], 0, x2[1], 2);
    
    copy_bit(&mut x[1], 7, x2[0], 5);
    copy_bit(&mut x[1], 6, x2[0], 1);
    copy_bit(&mut x[1], 5, x2[1], 5);
    copy_bit(&mut x[1], 4, x2[1], 1);

    copy_bit(&mut x[1], 3, x2[0], 4);
    copy_bit(&mut x[1], 2, x2[0], 0);
    copy_bit(&mut x[1], 1, x2[1], 4);
    copy_bit(&mut x[1], 0, x2[1], 0);
}

fn round(x: &mut [u8; 2], r_k: &[u8; 2]) {
    xor(x, r_k);
    sub(x);
    per(x);
}

fn sched(key: &[u8; 4]) -> [u8; 10] {
    let mut s: [u8; 10] = [0; 10];

    for i in (0..10).step_by(2) {
        if i % 4 == 0 {
            s[i] = key[i/4];
            s[i+1] = key[i/4+1];
        } else {
            s[i] = key[i/4] & 0xf;
            s[i] <<= 4;
            s[i] += (key[i/4+1] & 0xf0) >> 4;

            s[i+1] = key[i/4+1] & 0xf;
            s[i+1] <<= 4;
            s[i+1] += (key[i/4+2] & 0xf0) >> 4;

        }
    }
    s
}

pub fn print_block(t: &[u8; 2]) {
    println!("{:#04x}{:02x}", t[0], t[1]);
}

pub fn print_block_b(t: &[u8; 2]) {
    println!("{:#010b}{:08b}", t[0], t[1]);
}

pub fn encrypt(pt: &[u8; 2], key: &[u8; 4]) -> [u8; 2] {
    let mut ct = pt.clone();
    let round_key = sched(key);
    for i in (0..6).step_by(2) {
        round(&mut ct, round_key[i..i+2].try_into().unwrap());
    }
    xor(&mut ct, round_key[6..8].try_into().unwrap());
    sub(&mut ct);
    xor(&mut ct, round_key[8..10].try_into().unwrap());

    ct
}
