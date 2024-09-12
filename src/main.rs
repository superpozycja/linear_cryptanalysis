use linear_cryptanalysis as la;

fn main() {
    println!("Hello, world!");
    let ct = la::encrypt(b"ab", b"xy");
    println!("ciphertext = {:#04x}{:02x}", ct[0], ct[1])
}
