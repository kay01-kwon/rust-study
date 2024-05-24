use std::io;

fn main() {
    if_test1();
}


fn if_test1()
{
    loop{
        let mut num_: String = String::new();

        io::stdin()
        .read_line(&mut num_)
        .expect("Failed to read line.");
    
        let num_:i32 = match num_.trim().parse()
        {
            Ok(num) => num,
            Err(_)=>continue,
        };

        if num_ < 8
        {
            println!("Condition was true");
        }
        else
        {
            println!("Condition was false");
            break;
        }
    }

}
