fn main() {
    another_function(7);
    print_labeled_meas(7,'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("-----------------------");
    println!("The value of y is: {y}");
    println!("-----------------------");

    let x = seven();
    
    println!("The value of x is: {x}");
    println!("-----------------------");

    let x_plus_one = plus_one(x);

    println!("The value of x is: {x_plus_one}");
    println!("-----------------------");
}

fn another_function(x:i32)
{
    println!("-----------------------");
    println!("The value of x is: {x}");
    println!("-----------------------");
}

fn print_labeled_meas(value:i32, unit_label:char)
{
    println!("-----------------------");
    println!("The meas is {value}{unit_label}");
    println!("-----------------------");
}

fn seven() -> i32
{
    7
}

fn plus_one(x: i32) -> i32 
{
    x + 1
}