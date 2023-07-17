// 循环打印从’a’~’Z’ 之间的所有字符
pub fn print() {
    let start: u8 = 'Z' as u8;

    for ch in (char::from(start)..='a').rev() {
        println!("ch: {ch}");
    }
}
