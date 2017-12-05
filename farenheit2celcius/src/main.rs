use std::io;

fn main() {
    println!("Farenheit to Celcius converter program");

    loop {
        println!("Please input temperature in Farenheit");

        let mut faren_temp = String::new();

        io::stdin().read_line(&mut faren_temp)
            .expect("Failed to read line");

        let faren_temp: f32 = match faren_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a numeric value");
                continue
            },
        };

        println!("You entered: {} degrees Farenheit", faren_temp);
        println!("Celcius degrees: {:.2}", farenheit_to_celcius(faren_temp));

    }
}

fn farenheit_to_celcius(faren_temp: f32) -> f32{
    // T(°C) = (T(°F) - 32) × 5/9
    let celc_temp = (faren_temp - 32.0) * (5.0 / 9.0);
    return celc_temp
}
