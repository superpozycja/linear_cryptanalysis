use linear_cryptanalysis as la;

fn main() {
    let pt = [0b0010_0110, 0b1011_0111];
    let key = [0b0011_1010, 0b1001_0100, 0b1101_0110, 0b0011_1111];
    let ct = la::encrypt(&pt,  &key);
    la::print_block(&ct);
}
