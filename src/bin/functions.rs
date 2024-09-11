
fn main() {
    println!("Hello, world!");

    another_function(5, 'h');


    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");


    let z = plus_one(5);
    println!("The value of z is {z}")
}

fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}