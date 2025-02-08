fn main() {
    let a = 5;
    let b = 3;
    test(a, b);
}

#[no_mangle]
// #[inline(never)]
fn test(a: u8, b: u8) -> u8 {
    if a > b {
        let c = a + b;
        c
    } else {
        let c = b;
        c
    }
}
