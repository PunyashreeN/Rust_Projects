use std::io;
fn main() {
    println!("Enter the number to generate sequence");
    let mut number=String::new();
    io::stdin()
        .read_line(&mut number)
        .expect(" Cannot read number");
    let number: u8=number.trim().parse()
        .expect("Please enter valid number");
    fibonacci(number);
}
fn fibonacci(number:u8){
    let mut a=0;
    let mut b=1;
    let mut c=1;
    for _i in 1..=number{
        c=b;
        b=a+b;
        a=c;
        print!(" {} ",b);
    }
}
