unsafe extern "C" {
    fn asm_add(a: i64, b: i64) -> i64;
}

fn main() {
    let a: i64 = 7;
    let b: i64 = 35;
    let result = unsafe { asm_add(a, b) };
    println!("{a} + {b} = {result}");
}
