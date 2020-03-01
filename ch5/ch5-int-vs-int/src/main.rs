fn main() {
    // int_vs_int()
    // frankentype();
    // overflow();
}

fn int_vs_int() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}

fn frankentype() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe { std::mem::transmute(a) };
    println!("{:032b}", frankentype);
}

fn overflow() {
    let mut i: u16 = 0;
    print!("{}..", i);

    loop {
        i += 1000;
        print!("{}..", i);
        if i %  10000 == 0 {
            print!("\n");
        }
    }
}


