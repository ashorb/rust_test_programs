
fn main() {
    let num = 6;
    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let cond = true;
    let num2 = if cond {5} else {6};
    println!("{num2}");


    
}