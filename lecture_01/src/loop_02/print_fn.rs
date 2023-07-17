pub fn print() {
    let start: u8 = 'A' as u8;

    for ch in char::from(start)..='z' {
        println!("ch: {ch}");
    }
}

pub fn print_pretty() {
    let start: u8 = 'A' as u8;
    let mut c: u32 = 0;
    for ch in char::from(start)..='z' {
        print!("{ch} ");
        c = c + 1;
        if c % 7 == 0 {
            println!();
        }
    }
}
