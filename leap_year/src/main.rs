use std::io;

fn leap_year(year:&u32)->bool{
    if year%4==0{
        if year%100==0{
            if year%400==0{
                return true;
            }
            else {
                return false;
            }
        }
        else {return true;}
    }
    false
}

fn main() {
    println!("Leap Year");
    loop{
        println!("Enter a year to check:\n enter a character to quit");
        let mut year=String::new();
        io::stdin()
            .read_line(&mut year)
            .expect("Cannot read line!");
        let year:u32=match year.trim().parse(){
            Ok(value)=>value,
            Err(_e)=>{//println!("error message:{} ",e);
                    break},
        };
        let result=leap_year(&year);
        match result{
            true=>println!("{} year is a leap year",year),
            false=>println!( "{} year is not a leap year",year),
        };
    }
}
