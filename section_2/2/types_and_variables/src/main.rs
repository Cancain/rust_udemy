use ::std::mem;

fn main() {
    //unsigned
    let a: u8 = 123; //8bits
    println!("a = {}", a);

    // a = 256;

    let mut b: i8 = 0;
    println!("b = {}", b);
    b = 32;
    println!("b = {}", b);

    let mut c = 123456789; //32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, after modification", c);

    let z: isize = 123; //isizeo
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit os",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x';
    println!("d = {}, size = {}", d, mem::size_of_val(&d));

    let e = 2.5; //double-precision, 8 bytes or 64 bits, f64
    println!("e = {}, size = {}", e, mem::size_of_val(&e));

    //booleam
    let g = false;
    println!("g = {}, size = {}", g, mem::size_of_val(&g));

    let f = 4 > 0; //true
}
