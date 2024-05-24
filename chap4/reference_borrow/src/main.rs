fn main() {

    let s1 = String::from("hello");
    let (s2, length) = calculate_length(s1);


    println!("***********************************");
    println!("1. By passing just string");
    println!("The length of {} is {}.",s2,length);
    println!("***********************************");

    // You do not need to declare tuple at all
    // when you make use of reference
    let length2 = calculate_length_ptr(&s2);

    println!("***********************************");
    println!("2. By using reference");
    println!("The length of {} is {}.",s2,length2);
    println!("***********************************");

    // Mutable reference
    let mut s = String::from("Hello");

    println!("***********************************");
    println!("3. Mutable reference");
    println!("Before modification...");
    println!("{s}");
    change(&mut s);
    println!("After modification...");
    println!("{s}");
    println!("***********************************");

    // No dangling reference
    let s = no_dangle();
    println!("***********************************");
    println!("4. No dangle() example");
    println!("{}",s);
    println!("***********************************");

}

fn calculate_length(s:String) -> (String, usize)
{
    let l = s.len();
    (s,l)
}

fn calculate_length_ptr(s: &String) -> usize
{
    s.len()
}

fn change(s:&mut String)
{
    s.push_str(", world");
}

fn no_dangle() -> String
{
    let s = String::from("No dangle()");
    s
}