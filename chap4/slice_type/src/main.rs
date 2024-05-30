fn main() {
    // To get index through reference 
    // is problematic
    let mut s = String::from("hello world");
    let word = first_word_index(&s);
    let (first_index, last_index) = second_word_index(&s);
    // s.clear();

    println!("Space starts at {}",word);
    // But, the string is already deleted
    // You cannot access to the String variable s
    println!("{}, {}",first_index, last_index);


    // String slice
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello: {}",hello);
    println!("world: {}",world);
    

    let first_word = first_word(&s);
    // Mutable reference cannot be borrowed.
    // The below one causes error.
    // s.clear();

    println!("The first word is: {}", first_word);



}

fn first_word_index(s:&String) -> usize
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

fn second_word_index(s: &String) -> (usize, usize) 
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

// Utilize slice to get string
fn first_word(s:&String) -> &str
{
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate()
    {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}