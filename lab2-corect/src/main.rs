fn add_chars_n(s:&mut String,c:char,n:i32){//->String{//mut s: String
    for _ in 0..n{
        s.push(c);
    }
    //s
}
fn add_chars_n1(mut s: String,c:char,n:i32)->String{//mut s: String
    for _ in 0..n{
        s.push(c);
    }
    s
}
fn add_space(mut s:String,n:i32)->String{
let mut i=0;
while i<n{
    s.push(' ');
    i+=1;
}
s
}
fn add_str(mut s:String, x:String)->String{
    for c in x.chars(){
        s.push(c);
    }
    s
}
fn add_integer(mut s: String, mut n: u32) -> String {
    if n == 0 {
        s.push('0');
        return s;
    }
    let mut temp = String::from("");
    let mut count = 0;
    while n > 0 {
        if count > 0 && count % 3 == 0 {
            temp.push('_');
        }
        let digit = (n % 10) as u8;
        temp.push((digit + b'0') as char);
        n /= 10;
        count += 1;
    }
    for c in temp.chars().rev() {
        s.push(c);
    }
    s
}
fn add_float(mut s: String, f: f32) -> String {
    let int_part = f as u32;
    let mut temp_int = String::from("");
    let mut n = int_part;
    if n == 0 {
        temp_int.push('0');
    }
    while n > 0 {
        temp_int.push(((n % 10) as u8 + b'0') as char);
        n /= 10;
    }
    for c in temp_int.chars().rev() {
        s.push(c);
    }
    s.push('.');
    let frac = f - (int_part as f32);
    let frac_int = (frac * 1000.0 + 0.5) as u32;
    let d1 = frac_int / 100;
    let d2 = (frac_int / 10) % 10;
    let d3 = frac_int % 10;
    s.push((d1 as u8 + b'0') as char);
    s.push((d2 as u8 + b'0') as char);
    s.push((d3 as u8 + b'0') as char);
    s
}
fn main() {
    let mut s = String::from("");

    // Linia 1
    s = add_space(s, 21);
    s = add_str(s, "I".to_string());
    s = add_space(s, 1);
    s = add_str(s, "💚\n".to_string());

    // Linia 2
    s = add_space(s, 21);
    s = add_str(s, "RUST.\n\n".to_string());

    // Linia 3
    s = add_str(s, "Most".to_string());
    s = add_space(s, 12);
    s = add_str(s, "crate".to_string());
    s = add_space(s, 7);
    s = add_integer(s, 306437968);
    s = add_space(s, 11);
    s = add_str(s, "and".to_string());
    s = add_space(s, 7);
    s = add_str(s, "lastest".to_string());
    s = add_space(s, 11);
    s = add_str(s, "is\n".to_string());

    // Linia 4
    s = add_space(s, 8);
    s = add_str(s, "downloaded".to_string());
    s = add_space(s, 6);
    s = add_str(s, "has".to_string());
    s = add_space(s, 13);
    s = add_str(s, "downloads".to_string());
    s = add_space(s, 7);
    s = add_str(s, "the".to_string());
    s = add_space(s, 13);
    s = add_str(s, "version".to_string());
    s = add_space(s, 5);
    s = add_float(s, 2.038);
    s = add_str(s, ".\n".to_string());

    print!("{}", s);
}