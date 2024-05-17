use rand::Rng;
use std::io::stdin;

fn main() {
    println!("Welcome to Rock Paper Scissors!");

    const WIN_COMPS: [[u8; 2]; 3] = [[1, 3], [2, 1], [3, 2]];

    loop {
        let computer_choice = rand::thread_rng().gen_range(1..4);

        println!("");
        println!("");
        println!("__________________");
        println!("Enter your choice: ");
        println!("(1) Rock");
        println!("(2) Paper");
        println!("(3) Scissors");
        println!("`exit` to quit");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        };

        let player_choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let player_choice = match player_choice {
            1 => 1,
            2 => 2,
            3 => 3,
            _ => {
                println!("Invalid choice");
                continue;
            },
        };

        let mut won:bool = false;

        if player_choice == computer_choice {
            println!("Tie!");
        } else {
            for i in WIN_COMPS {
                if i == [player_choice, computer_choice] {
                    println!("You win!");
                    won = true;
                    break;
                }
            }

            if !won {
                println!("You lose!");
            }
        };
    }
}
