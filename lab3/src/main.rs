use thiserror::Error;
fn is_prime(n: u16) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut divisor = 3u16;
    while divisor <= n / divisor {
        if n % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}
fn next_prime(x: u16) -> Option<u16> {
    let mut candidate = x;
    loop {
        candidate = candidate.checked_add(1)?;
        if is_prime(candidate) {
            return Some(candidate);
        }
    }
}
fn print_primes_until_none(start: u16) {
    let mut current = start;
    loop {
        match next_prime(current) {
            Some(prime) => {
                println!("prime > {current}: {prime}");
                current = prime;
            }
            None => {
                println!("no larger prime in u16");
                break;
            }
        }
    }
}
fn add_or_panic(a: u32, b: u32) -> u32 {
    a.checked_add(b)
        .unwrap_or_else(|| panic!("u32 addition overflow: {a} + {b}"))
}

fn multiply_or_panic(a: u32, b: u32) -> u32 {
    a.checked_mul(b)
        .unwrap_or_else(|| panic!("u32 multiplication overflow: {a} * {b}"))
}
#[derive(Debug, Error, PartialEq, Eq)]
enum MathError {
    #[error("u32 arithmetic overflow")]
    Overflow,
}

fn add(a: u32, b: u32) -> Result<u32, MathError> {
    a.checked_add(b).ok_or(MathError::Overflow)
}

fn multiply(a: u32, b: u32) -> Result<u32, MathError> {
    a.checked_mul(b).ok_or(MathError::Overflow)
}

fn add_then_multiply(a: u32, b: u32, factor: u32) -> Result<u32, MathError> {
    let sum = add(a, b)?;
    multiply(sum, factor)
}
#[derive(Debug, Error, PartialEq, Eq)]
enum CharError {
    #[error("character is not ASCII")]
    NotAscii,
    #[error("character is not a decimal digit")]
    NotDigit,
    #[error("character is not a hexadecimal digit")]
    NotHexDigit,
    #[error("character is not a letter")]
    NotLetter,
    #[error("character is not printable")]
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<String, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }
    Ok(c.to_uppercase().collect())
}

fn to_lowercase(c: char) -> Result<String, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }
    Ok(c.to_lowercase().collect())
}

fn print_char(c: char) -> Result<(), CharError> {
    if c.is_control() {
        return Err(CharError::NotPrintable);
    }
    println!("{c}");
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    c.to_digit(10).ok_or(CharError::NotDigit)
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    c.to_digit(16).ok_or(CharError::NotHexDigit)
}

fn print_error(error: CharError) {
    println!("Error: {error}");
}
fn first_decimal_digit(text: &str) -> Option<u32> {
    text.chars().find_map(|c| char_to_number(c).ok())
}

fn main() {
    print_primes_until_none(65_500);
    println!("12 + 30 = {}", add_or_panic(12, 30));
    println!("6 * 7 = {}", multiply_or_panic(6, 7));
    let addition_overflow = std::panic::catch_unwind(|| add_or_panic(u32::MAX, 1));
    println!("addition overflow caught: {}", addition_overflow.is_err());
    let multiplication_overflow = std::panic::catch_unwind(|| multiply_or_panic(u32::MAX, 2));
    println!("multiplication overflow caught: {}", multiplication_overflow.is_err());
    println!("10 + 20, then * 2 = {:?}", add_then_multiply(10, 20, 2));
    println!("overflow case = {:?}", add_then_multiply(u32::MAX, 1, 2));
    for result in [to_uppercase('a'), to_uppercase('7')] {
        match result {
            Ok(value) => println!("uppercase: {value}"),
            Err(error) => print_error(error),
        }
    }
    for result in [to_lowercase('Z'), to_lowercase('!')] {
        match result {
            Ok(value) => println!("lowercase: {value}"),
            Err(error) => print_error(error),
        }
    }
    if let Err(error) = print_char('R') {
        print_error(error);
    }
    if let Err(error) = print_char('\n') {
        print_error(error);
    }
    for c in ['7', 'x', 'é'] {
        match char_to_number(c) {
            Ok(value) => println!("decimal value of {c}: {value}"),
            Err(error) => print_error(error),
        }
    }
    for c in ['F', 'G', 'é'] {
        match char_to_number_hex(c) {
            Ok(value) => println!("hex value of {c}: {value}"),
            Err(error) => print_error(error),
        }
    }
    println!("digit in 'room 204'  {:?}", first_decimal_digit("room 204"));
    println!("digit in 'no digits'  {:?}", first_decimal_digit("no digits"));
}
