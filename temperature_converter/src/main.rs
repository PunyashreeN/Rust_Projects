use std::io;
fn main(){
    println!("Enter an option:\n 1. Celcius to Fahrenheit\t 2. Fahrenheit to celcius");
    let mut option=String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Could not read line for option");
    let option=option.trim();    
    println!("option:{:?}",option);
   
    if option !="1"  && option !="2"{
        println!("Enter a valid option 1 or 2");
    }
    else{
        println!("Enter the temperature value:");
        let mut temp=String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Couldn't read temperature");
        let temp: f64 =temp.trim().parse().expect("Please enter a valid temperature");
        if option=="1"{
            println!("you choose celcius to fahrenheit:");
            cel_to_fah(temp);
            
        }
        else if option =="2"{
            println!("you choose fahrenheit to celcius");
            fah_to_cel(temp);
        }
        else{
            println!("Could'nt interpret option")
        }
    }
    
}

fn cel_to_fah(temp:f64){
    let fahrenheit=(temp*9.0/5.0)+32.0;    
    println!("Temperature in fahrenheit:{:.2} ",fahrenheit);
}

fn fah_to_cel(temp:f64){
    let celcius=(temp-32.0)*5.0/9.0;
    println!("Temperature in celcius is: {:.2}",celcius);
}

