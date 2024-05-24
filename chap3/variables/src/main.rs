fn main() {

    mutability_fn();

    shadowing_fn();

    character_type_fn();

    tuple_type_fn();

    array_type_fn();
}

fn mutability_fn()
{
    println!("1. The mutability property.");
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
}

fn shadowing_fn()
{
    println!("2. Shadowing");
    let x = 5;
    let x = x + 1;

    {
        let x = x*2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

}

fn character_type_fn()
{
    let c = 'z';
    let z:char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("-------------------");
    println!("char 1: {c}");
    println!("char 2: {z}");
    println!("char 3: {heart_eyed_cat}");
}

fn tuple_type_fn()
{
    let tup: (i32, f64, u8) = (500,6.45,1);
    let (x,y,z) = tup;
    println!("-------------------");
    println!("The value of x: {x}");
    println!("The value of y: {y}");
    println!("The value of z: {z}");
    println!("-------------------");
    
    let tup0 = tup.0;
    let tup1 = tup.1;
    let tup2 = tup.2;

    println!("tup.0: {tup0}");
    println!("tup.1: {tup1}");
    println!("tup.2: {tup2}");
}

fn array_type_fn()
{
    let months = ["Jan", "Feb", "Mar", "Apr", 
    "May", "Jun", "Jul", "Aug", "Sep", "Oct",
    "Nov", "Dec"];
    
    println!("-------------------");
    for i in months
    {
        println!("{}",i);
    }
}