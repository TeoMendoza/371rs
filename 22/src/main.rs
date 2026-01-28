use malloc::*;



fn main() {
    let p0 = malloc(16).unwrap();
    let p1 = malloc(32).unwrap();
    let x = 0x44332211;
    let y = 0x12345678;
    setter(x, p0);
    setter(y, p1);
    let z: i32 = getter(p0);
    let w: i32 = getter(p1);
    assert!(x == z);
    assert!(y == w);
    println!("A+");
    // Advanced topics.

    // Big alloc should fail

    assert!(malloc(2048).is_none());
    println!("A++");

    // Allocs totaling > SIZE should fail

    // We have alloc (16 + 32) * 8 = 384 of 1024
    // Try annother small malloc
    malloc(32).unwrap();
    // And then one too large.
    assert!(malloc(64).is_none());
    println!("A+++");

    // Easiest to test these together:
    //  - Gets to uninitialized memory should fail
    //  - You should write free()
    // No graceful way to autotest these. Left as an exercise to the interested student.
}




