fn main() {
    // To get index through reference 
    // is problematic
    let mut s = String::from("hello world");
    let word = first_word(&s);
    let (first_index, last_index) = second_word(&s);
    s.clear();

    println!("Space starts at {}",word);
    // But, the string is already deleted
    // You cannot access to the String variable s
    println!("{}, {}",first_index, last_index);


    // String slice
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn first_word(s:&String) -> usize
{
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate()
    {
        // Return the index of back space
        if item == b' '{
            return i;
        }
    }
    // Otherwise, return the length of the string
    s.len()
}

fn second_word(s: &String) -> (usize, usize) 
{
    let bytes = s.as_bytes();
    let first_index;
    let last_index;

    first_index = 0;
    for(i,&item) in bytes.iter().enumerate()
    {
        // Return the index of back space
        if item == b' '{
            let first_index = i;
        }
    }

    last_index = s.len();

    (first_index, last_index)
}