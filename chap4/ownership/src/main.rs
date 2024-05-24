fn main() {
    let s1 = gives_ownership();

    println!("{}",s1);

    let s2 = String::from("hello");

    println!("{}",s2);

    let s3 = takes_and_gives_ownership(s2);

    println!("{}",s3);

}

fn gives_ownership() -> String
{
    let some_str = String::from("Yours");

    some_str
}

fn takes_and_gives_ownership(a_string:String)->String
{
    a_string
}