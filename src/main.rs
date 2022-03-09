use std::io;

fn main() {
    println!("Welcome to the Rust Temperature Converter");
    
    loop {
    println!("To convert from Fharenheit enter 'F'. To convert from Celsius enter 'C'.");

    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Invalid input, try again.");
    
        let unit: char = match unit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    println!("Enter the temperature you wish to convert.");

    let mut temp = String::new();
    
    io::stdin()
        .read_line(&mut temp)
        .expect("Invalid input, try again.");

        let temp: u32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    let new_temp = if unit == 'F' {
        (temp - 32) * 5 / 9
    } else {
        temp * 9 / 5 + 32
    };

    println!("The resulting conversion from {}{} is {}",temp,unit,new_temp);
    break;
}
}
