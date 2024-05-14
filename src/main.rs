use std::io;
pub use meteo_tools::conversions::temperature::celsius_to_fahrenheit;
pub use meteo_tools::conversions::temperature::fahrenheit_to_celsius;

fn main() {
    println!("temp-convert: Convert temperatures between Fahrenheit and Celsius.\n");

    loop {
        println!("Select conversion mode.\n");
        println!("[1] Fahrenheit -> Celsius\n");
        println!("[2] Celsius -> Farenheit\n");
        println!("[3] Exit\n");

        let mut mode: String = String::new();

        io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");
        
        let mode: u32 = match mode.trim().parse() { // make user choose conversion then enter condition

            Ok(num) => num,
            Err(_) => {
                println!("\nPlease type either 1, 2, or 3!\n"); // tbh this is not necessary should get rid of it later
                continue;
            }
        };

        if mode == 1 {

            println!("\nConverting Fahrenheit -> Celsius\n");

            println!("Enter Fahrenheit temperature value to convert to Celsius:\n");

            // Get fahrenheit temp from user input
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line...");
            
            // Parse string from user input to f64
            let ftemp: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("[Invalid input!] Please enter a floating-point number.");
                    continue;
                }
            };

            println!("\nFahrenheit value entered: {ftemp}°F\n");

            let celsius_temperature = fahrenheit_to_celsius(&ftemp);
            println!("[Converted to Celsius]: Temperature is: {celsius_temperature}°C\n");
        }

        else if mode == 2 {
            println!("\nConverting Celsius -> Farenheit\n");

            println!("Enter Celsius temperature value to convert to Fahrenheit:\n");

            // Get celsius temp from user input
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line...");
            
            // Parse string from user input to f64
            let ctemp: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("[Invalid input!] Please enter a floating-point number.");
                    continue;
                }
            };

            println!("\nCelsius value entered: {ctemp}°C\n");

            let fahrenheit_temperature = celsius_to_fahrenheit(&ctemp);
            println!("[Converted to Fahrenheit]: Temperature is: {fahrenheit_temperature}°F\n");
        }

        else if mode == 3 {
            println!("\nExiting...");
            break;
        }

        else {
            println!("\nPlease choose a valid option (1, 2, or 3)\n");
            continue;
        }
    

        println!("Do you want to perform another conversion? (yes/no)\n");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let answer = answer.trim().to_lowercase();
        if answer != "yes" {
            println!("\nExiting...");
            break;
        }
    }
}
