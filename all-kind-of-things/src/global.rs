use std::f64::consts;

const C: u8 = 42;
static mut Z: i32 = 1234;

fn global() {
    println!("C: {}", C);
    unsafe {
        println!("Z: {}", Z);
        Z = 4;
        println!("Z: {}", Z);
    }
}
