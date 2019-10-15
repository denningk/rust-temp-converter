use std::io;
        
type TempConversion = fn(f32) -> f32;

fn main() {
    loop {
        println!("Would you like to start with Celsius or Fahrenheit? Enter c or f");

        let mut temp_choice = String::new();

        io::stdin().read_line(&mut temp_choice)
            .expect("Failed to read line");

        let conversion_function: TempConversion;
        let chosen_temp;

        if temp_choice.trim().to_lowercase() == "c" {
            conversion_function = celsius_to_fahrenheit;
            chosen_temp = "Celsius";
        } else if temp_choice.trim().to_lowercase() == "f" {
            conversion_function = fahrenheit_to_celsius;
            chosen_temp = "Fahrenheit";
        } else {
            println!("Please enter c or f");
            continue
        }

        println!("Please enter a temperature in {}", chosen_temp);

        let mut starting_temp = String::new();

        io::stdin().read_line(&mut starting_temp)
            .expect("Failed to read line");

        let starting_temp: f32 = match starting_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let new_temp = conversion_function(starting_temp);
        println!("Temperature in Celsius: {}\nTemperature in Farenheit: {}", starting_temp, new_temp);
    }
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    return (temp * (9.0/5.0)) + 32.0
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    return (temp - 32.0) * 5.0/9.0
}