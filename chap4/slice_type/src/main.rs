fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("{}",word);
    // But, the string is already deleted
    // You cannot access to the String variable s
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