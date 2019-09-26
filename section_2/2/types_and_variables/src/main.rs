use ::std::mem;

fn operators() {
    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a = a + 1;
    a -= 2;

    println!("The remainder of {} / {} = {}", a, 3, a / 3);

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    //bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                   // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10; // >>
    println!("2^10 = {}", two_to_10);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0; //true

    let x = 5;
    let x_is_5 = x == 5; //true
}

fn scope_and_shadowing() {
    let a = 123;

    {
        let b = 456;
        let a = 777;
        println!("inside, b = {}", b);
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
    // println!("b = {}", b);
}

fn fundamental_data_types() {
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

fn main() {
    scope_and_shadowing();
}
