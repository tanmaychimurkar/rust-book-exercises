use std::io;

fn main() {
    // get the following as initial steps:
    // 1) Type of temperature user wants to input
    // 2) Type of temperature user wants to convert into
    // 3) The value of temperature the user wants to convert

    // get type of input temperature
    println!("Enter the type of Temperature you are entering: C or F?");
    let mut temperature_type = String::new();

    io::stdin()
        .read_line(&mut temperature_type)
        .expect("Please enter the type of Temperature you want to input: C or F?");

    // get type to convert to
    println!("Enter the type of Temperature you want to convert to: C or F?");
    let mut convert_type = String::new();

    io::stdin()
        .read_line(&mut convert_type)
        .expect("Please enter the type of Temperature you want to input: C or F?");

    // duh!
    if temperature_type == convert_type {
        println!("The temperature will remain the same in the same units!");
        return;
    };

    // get value of temperature to convert
    println!("Enter the value of the temperature type");
    let mut temperature_value = String::new();

    io::stdin().read_line(&mut temperature_value).expect(
        "There should be a value for the temperature to convert! I cannot yet \
        get the value of the room temperature (I know right, not sentient enough yet!!!!)",
    );

    let temperature_value: f32 = match temperature_value.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("The value must be a number, how can I take a string temperature!!!");
            return;
        }
    };

    println!(
        "You are inputting temperature in {temperature_type}, and want to convert \
        it to {convert_type}. To confirm and proceed, \n please enter (Y), to abort press (N)"
    );
    let mut final_assertion_string = String::new();

    io::stdin()
        .read_line(&mut final_assertion_string)
        .expect("Please confirm with either Y or N to proceed!");

    match final_assertion_string.trim() {
        "N" => println!("Thank you for using temperature converter, have a nice day!"),
        "Y" => {
            if temperature_type.trim() == "F" {
                ftoc(temperature_value)
            } else {
                ctof(temperature_value)
            };
        }
        _ => {
            println!("The input has to be either Y or N. Please try again!")
        }
    };
}

fn ctof(temperature_value: f32) -> () {
    let computed_value = 32. + (9. / 5.) * temperature_value;
    println!("The temperature in F is {computed_value}")
}

fn ftoc(temperature_value: f32) -> () {
    let computed_value = (5. / 9.) * (temperature_value - 32.);
    println!("The temperature in C is {computed_value}")
}

// Key takeaways:
// When we read user input via io:stdin(), in read_line, we always use the address
// of mutable variable that we are passing.

// When using match statement, if matching string, we need to trim the string, since the user presses
// enter to proceed, and that is taken as a character in Rust. trim() functio helps get rid of
// such characters

// In match clauses, it is important to handle the `_` match case, which implies that the user
// input has not matched any value and thus the message from `_` case will be printed out

// When shadowing a variable, we need to mention it's type. This can most likely be when we take
// an input from the user as String::new(), and want to cast it to an int or float. In this case
// of shadowing, it is important that we mention the type of the new shadow variable that we are
// creating.

// When doing arithmetic with floats and ints, if there is a float, then the ints should also be
// floats by default, and this can be done by adding a `.` at the end of int values.
