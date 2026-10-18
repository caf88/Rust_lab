fn add_chars_n(s: &mut String, c: char, mut i: i32){
    while i>0 {
        s.push(c);
        i-=1;
    }
}
fn main() {
    let mut s = String::from("ceva");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}