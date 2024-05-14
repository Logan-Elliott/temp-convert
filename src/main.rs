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
                println!("Please type either 1, 2, or 3!");
                continue;
            }
        };

        if mode == 1 {

            println!("Converting Fahrenheit -> Celsius\n");

            println!("Enter fahrenheit temperature value to convert to celsius:\n");

            // Get fahrenheit temp from user input
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line...");
            
            println!("Fahrenheit value entered: {input}");

            // Parse string from user input to f64
            let ftemp: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("[Invalid input!] Please enter a floating-point number.");
                    continue;
                }
            };

            let celsius_temperature = fahrenheit_to_celsius(&ftemp);
            println!("[Converted to Celsius]: Temperature is: {celsius_temperature}°C\n");
        }

        else if mode == 2 {
            println!("Converting Celsius -> Farenheit\n");

            println!("Enter celsius temperature value to convert to fahrenheit:\n");

            // Get celsius temp from user input
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line...");
            
            println!("Celsius value entered: {input}");

            // Parse string from user input to f64
            let ctemp: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("[Invalid input!] Please enter a floating-point number.");
                    continue;
                }
            };

            let fahrenheit_temperature = celsius_to_fahrenheit(&ctemp);
            println!("[Converted to Fahrenheit]: Temperature is: {fahrenheit_temperature}°F\n");
        }

        else if mode == 3 {
            println!("Exiting...");
            break;
        }

        else {
            println!("Please choose a valid option (1, 2, or 3)\n");
            continue;
        }
    

        println!("Do you want to perform another conversion? (yes/no)");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let answer = answer.trim().to_lowercase();
        if answer != "yes" {
            println!("Exiting...");
            break;
        }
    }
}
