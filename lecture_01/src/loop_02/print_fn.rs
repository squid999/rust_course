pub fn print() {
    let start: u8 = 'A' as u8;

    for ch in char::from(start)..='z' {
        println!("ch: {ch}");
    }
}
