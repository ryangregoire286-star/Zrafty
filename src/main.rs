use std::io::stdin;

fn main() {
    let mut input = String::new();
    let message: String = String::from("Enter Command: ");
    let mut guess_input = String::new();

    println!("{}", get_command_upper(message));
    stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "/Guess" => {
            let guess = get_random_guess(5) as i32;

            println!("{}", "Enter Guess Between 1 and 5: ");
            stdin().read_line(&mut guess_input).unwrap();
            let num_guess = guess_input.trim().parse::<i32>().expect("No Input");

            if num_guess == guess {
                println!("{}", "Correct The Guessed Number");
            } else if num_guess != guess {
                println!("Correct Guess was: {} ", guess);
            }
        }

        "/Math" => {}

        _ => println!("{}", "Default Value"),
    }
}

fn get_random_guess(num1: i32) -> i32 {
    rand::random_range(1..num1)
}

fn get_command_upper(message: String) -> String {
    message.to_uppercase()
}
