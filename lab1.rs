
fn is_prime(n:u32)->bool{
    if n==1 {return false}
    if n==3 {return true}
    if n%2==0 || n%3==0{return false}
    let mut i=5;
    while i*i<=n{
        if n%2==0 || n%(i+2)==0{return false}
        i+=1;
    }
    true
} 
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
fn is_coprime(a: u32, b: u32) -> bool {
    gcd(a, b) == 1
}
fn sing_99_bottles() {
    for i in (1..=99).rev() {
        let current_bottle = if i == 1 { "bottle" } else { "bottles" };
        let next_i = i - 1;
        let next_bottle = if next_i == 1 { "bottle" } else { "bottles" };

        println!("{0} {1} of beer on the wall, {0} {1} of beer.", i, current_bottle);
        
        if next_i == 0 {
            println!("Take one down and pass it around, no more bottles of beer on the wall.\n");
        } else {
            println!("Take one down and pass it around, {} {} of beer on the wall.\n", next_i, next_bottle);
        }
    }
    
    println!("No more bottles of beer on the wall, no more bottles of beer.");
    println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
}
fn main() {
    for n in 0..=100 {
        if is_prime(n) {
            print!("{} ", n);
        }
    }
    println!("\n");
    let mut coprime_count = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            if is_coprime(i, j) {
                coprime_count += 1;
            }
        }
    }
    println!("S au gasit {} de perechi coprime.\n", coprime_count);
    println!("99 Bottles of Beer:");
    sing_99_bottles();
}