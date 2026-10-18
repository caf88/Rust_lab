fn add_space(s: &mut String, n: usize) {
    for _ in 0..n {
        s.push(' ');
    }
}

fn add_str(s: &mut String, value: &str) {
    s.push_str(value);
}

fn add_integer(s: &mut String, mut n: u64) {
    if n == 0 {
        s.push('0');
        return;
    }

    let mut digits = [0u8; 20];
    let mut len = 0;

    while n > 0 {
        digits[len] = (n % 10) as u8;
        n /= 10;
        len += 1;
    }

    let mut i = len;

    while i>0 {
        s.push((48 + digits[i-1]) as char);

        i-=1;

        if i > 0 && (len-i) % 3 == 0 {
            s.push('_');
        }
        
    }
}

fn add_float(s: &mut String, value: f64) {
    let integer_part = value as u64;
    let fractional_part = ((value - integer_part as f64) * 1000.0).round() as u64;

    add_integer(s, integer_part);
    s.push('.');

    if fractional_part < 100 {
        s.push('0');
    }
    if fractional_part < 10 {
        s.push('0');
    }

    add_integer(s, fractional_part);
}

fn main() {
    let mut result = String::new();

    add_space(&mut result, 40);
    add_str(&mut result, "I 💚\n");

    add_space(&mut result, 40);
    add_str(&mut result, "RUST.\n\n");

    add_space(&mut result, 4);
    add_str(&mut result, "Most");

    add_space(&mut result, 12);
    add_str(&mut result, "crate");

    add_space(&mut result, 6);
    add_integer(&mut result, 306_437_968);

    add_space(&mut result, 11);
    add_str(&mut result, "and");

    add_space(&mut result, 5);
    add_str(&mut result, "lastest");

    add_space(&mut result, 9);
    add_str(&mut result, "is\n");

    add_space(&mut result, 9);
    add_str(&mut result, "downloaded");

    add_space(&mut result, 8);
    add_str(&mut result, "has");

    add_space(&mut result, 13);
    add_str(&mut result, "downloads");

    add_space(&mut result, 5);
    add_str(&mut result, "the");

    add_space(&mut result, 9);
    add_str(&mut result, "version");

    add_space(&mut result, 4);
    add_float(&mut result, 2.038);

    add_str(&mut result, ".");

    println!("{}", result);
}