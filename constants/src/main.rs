// must give it a type - no fixed memory address
// const cannot be muteable
const FOO: u8 = 42;

// another way - fixed memory address
static BAR: u8 = 123;

// mutable constants !?
static mut FIZZ: i32 = 345;

fn main() {
    println!("{}", FOO);
    println!("{}", BAR);
    // must be unsafe to use FIZZ because FIZZ is not thread
    // safe since anything at anytime could be reading
    // and writting from it
    unsafe {
        println!("{}", FIZZ);
        FIZZ = 678; // eep
        println!("{}", FIZZ);
    }
}
