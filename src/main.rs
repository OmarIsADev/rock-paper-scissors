use rand::Rng;
use std::io::stdin;

fn main() {
    println!("Welcome to Rock Paper Scissors!");

    const WIN_COMPS: [[u8; 2]; 3] = [[1, 3], [2, 1], [3, 2]];

    loop {
        // Generate random number between 1 and 3
        let computer_choice: u8 = rand::thread_rng().gen_range(1..4);

        println!("");
        println!("__________________");
        println!("Enter your choice: ");
        println!("(1) Rock");
        println!("(2) Paper");
        println!("(3) Scissors");
        println!("`exit` to quit");

        // variable to store user input
        let mut input: String = String::new();
        
        // Read user input
        stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Check if user wants to quit
        if input.trim() == "exit" {
            break;
        };

        // Check if user input is valid
        let player_choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Check if player choice is valid
        let player_choice: u8 = match player_choice {
            1 => 1,
            2 => 2,
            3 => 3,
            _ => {
                println!("Invalid choice");
                continue;
            }
        };

        // variable stores if player won
        let mut won: bool = false;

        // Check if player and computer chose the same
        if player_choice == computer_choice {
            println!("Tie!");
            continue;
        } else {
            // Check if player won by any of the win combinations
            for i in WIN_COMPS {
                if i == [player_choice, computer_choice] {
                    println!("You win!");
                    won = true;
                    break;
                };
            }

            // if player didnt win print you lose
            if !won {
                println!("You lose!");
            };

            continue;
        };
    }
}
