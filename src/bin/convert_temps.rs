use std::io;

fn main() {
    let mut unit_type = String::new();
    let mut temp = String::new();

    println!("Press 'F' or 'C' to convert from Farenheit or Celcius");
    io::stdin().read_line(&mut unit_type).expect("Failed to read line.");

    println!("Enter temperature to convert.");
    io::stdin().read_line(&mut temp).expect("Failed to read line.");

    let temp = temp.trim();

    //let temp: i32 = temp.parse().unwrap();

    let temp: f64 = match temp.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number.");
            return;
        },
    }; 

    // println!("{unit_type}");
    if unit_type.trim().eq_ignore_ascii_case("F") {
        convert_f(temp);
    } else if unit_type.trim().eq_ignore_ascii_case("C") {
        convert_c(temp);
    } else {
        println!("Not a correct input.");
    }
}

fn convert_c(x: f64) {
    println!("{}", x * (9.0/5.0) + 32.0);
}

fn convert_f(x: f64) {
    println!("{}", (x - 32.0) * (5.0/9.0));
}
