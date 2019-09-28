fn if_statement() {
    let temp = 5;

    if temp > 30 {
        println!("Its's really hot outside");
    } else if temp < 10 {
        println!("really cold");
    } else {
        println!("temperature is ok");
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("{}", day);

    println!(
        "it is {}",
        if temp > 20 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "ok"
        }
    );

    println!(
        "it is {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "ok"
        }
    );
}

fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }

        println!("x = {}", x);
    }

    let mut y = 1;
    loop {
        //while true
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 {
            break;
        }
    }
}

fn main() {
    while_and_loop();
    // if_statement();
}
