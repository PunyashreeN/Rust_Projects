use std::io;
fn factorial(num:u32)->u32{
    if num==1 || num==0{
        return 1;
    }
    num*factorial(num-1)
}

fn main(){
    println!("Factorial of a number:\n Enter a number to find its factorial");
    let mut num=String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read input number");
    let num:u32=num.trim().parse().expect("Provide a valid number");
    
    println!("factorial of {} is {}",num,factorial(num));

}