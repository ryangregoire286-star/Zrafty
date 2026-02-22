use std::{
    fs::File,
    io::{BufWriter, Write, stdin},
};

fn add_nums(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn sub_nums(n1: i32, n2: i32) -> i32 {
    n1 - n2
}

fn main() -> std::io::Result<()> {
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
                let file = File::create("Guess.txt")?;

                let mut buff = BufWriter::new(file);
                writeln!(buff, "{}", guess)?;
                println!("Correct Guess was: {} ", guess);
            }
        }

        "/Math" => {
            let mut option = String::new();

            println!("{}", "Enter Option: ");
            stdin().read_line(&mut option).unwrap();
            let is_add: bool = option == "+";
            let is_sub: bool = option == "-";

            if is_add {
                let num_1 = get_random_guess(5) as i32;
                let num_2 = get_random_guess(10) as i32;

                let mut answer = String::new();
                stdin().read_line(&mut answer).unwrap();

                let num_answer = answer.trim().parse::<i32>().unwrap();

                if num_answer == add_nums(num_1, num_2) {
                    println!("{}", "You are | Correct Answer")
                }
            }

            if is_sub {
                let num_1 = get_random_guess(5) as i32;
                let num_2 = get_random_guess(10) as i32;

                let mut answer = String::new();
                stdin().read_line(&mut answer).unwrap();

                let num_answer = answer.trim().parse::<i32>().unwrap();

                if num_answer == sub_nums(num_1, num_2) {
                    println!("{}", "You are | Correct Answer")
                }
            }
        }

        _ => println!("{}", "Default Value"),
    }
    Ok(())
}

fn get_random_guess(num1: i32) -> i32 {
    rand::random_range(1..num1)
}

fn get_command_upper(message: String) -> String {
    message.to_uppercase()
}
