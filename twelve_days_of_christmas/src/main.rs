fn main() {
    println!("Twelve Days of Christmas");
    twelve_days_of_christmas();
}
fn twelve_days_of_christmas(){
    let days=["first","second","third","fourth","fifth","sixth","seventh","eighth","ninth","tenth","eleventh","twelfth"];
    for (day_no,day) in days.iter().enumerate(){
        println!("On the {} day of christmas my true love sent to me ",day);
         for gift in (1..=day_no+1).rev(){
            if gift==1 && day_no!=0{
                print!( "and ");
            }
             match gift{
                 1=>println!("A partridge in a pear tree. "),
                 2=>println!("Two turtle ducks, "),
                 3=>println!("Three French Hens,  "),
                 4=>println!("Four Calling Birds, "),
                 5=>println!("Five Golden Rings, "),
                 6=>println!("Six Geese a Laying, "),
                 7=>println!("Seven Swans a Swimming, "),
                 8=>println!("Eight Maids a Milking, "),
                 9=>println!("Nine Ladies Dancing, "),
                 10=>println!("Ten Lords a Leaping, "),
                 11=>println!("Eleven Pipers Piping, "),
                 12=>println!("Twelve Drummers Drumming, "),
                 _=>println!(""),
             }
         }
    }
}
