extern crate rand;
use std::io;
use rand::Rng;

fn main() {
     let(mut player_score, mut computer_score, mut game_count): (i32, i32, i32) = (0, 0, 0);
    loop {
        let mut _rng = rand::thread_rng();
        println!("1 - Rock, 2 - Paper, 3 - Scissors, 4 - Lizard, 5 - SPOCK");

        let mut players_choice = String::new();
        let mut players_choice_int = 0;

        io::stdin()
            .read_line(&mut players_choice)
            .expect("Cannot read the players_choice");

        let comp_choice = rand::thread_rng().gen_range(1..=5);

        match comp_choice {
            1 => println!("Computer chose Rock"),
            2 => println!("Computer chose Paper"),
            3 => println!("Computer chose Scissors"),
            4 => println!("Computer chose Lizard"),
            5 => println!("Computer chose SPOCK"),
            _ => println!("Error!"),
        }

        if players_choice == "1\n" {
            players_choice_int = 1;
            println!("You chose Rock");
        } else if players_choice == "2\n" {
            players_choice_int = 2;
            println!("You chose Paper");
        } else if players_choice == "3\n" {
            players_choice_int = 3;
            println!("You chose Scissors"); 
        } else if players_choice == "4\n" {
            players_choice_int = 4;
            println!("You chose Lizard");
        }else if players_choice == "5\n" {
            players_choice_int = 5;
            println!("You chose SPOCK");
        } else if players_choice == "0\n" {
            players_choice_int = 0;
            println!("goodbye");
            break;
        } else {
            println!("Error!");
        }
        let compare = (players_choice_int, comp_choice);
        match compare {
            (1, 1) => {
                println!("You both chose rock so its a tie!");
            }
            (1, 2) =>{
                println!("You lose! Paper covers rock");
                computer_score +=1 ;
            }        
            (1, 3) => {
                println!("You win! Rock breaks scissors");
                player_score +=1 ;
            }
            (1,4) => {
                println!("You win! Rock crushes lizard");
                player_score +=1 ;
            }
            (1,5) => {
                println!("You lose! Spock vaporizes rock");
                computer_score +=1 ;
            }
            (2, 1) => {
                println!("You win! Paper covers rock");
                player_score +=1 ;
            }
            (2, 2) => {
                println!("You both chose paper so its a tie!");
            }
            (2, 3) => {
                println!("You lose! Scissors cut paper");
                computer_score +=1;
            }
            (2, 4) => {
                println!("You lose! Lizard eats paper");
                computer_score +=1;
            }
            (2, 5) => {
                println!("You win! Paper disproves Spock");
                player_score +=1;
            }
            (3, 1) => {
                println!("You lose! Rock breaks scissors");
                computer_score +=1;
            }
            (3, 2) => {
                println!("You win! Scissors cut paper");
                player_score +=1;
            }
            (3, 3) =>{
                println!("It's a tie! You both scissor");
            }
            (3, 4) =>{
                println!("You wom! Scissors decapitate lizard");
                player_score +=1;
            }
            (3,5) => {
                println!("You lose! Spock smashes (or melts) scissors");
                computer_score +=1;
            }
            (4,1) => {
                println!("You lose! Rock crushes lizard");
                computer_score +=1;
            }
            (4,2) => {
                println!("You won! Lizard eats paper");
                player_score +=1;
            }
            (4,3) => {
                println!("You lose! Scissors decapitate lizard");
                computer_score +=1;
            }
            (4,4) => {
                println!(" Its a tie! you both chose Lizard");
            }
            (4,5) => {
                println!("You WON! Lizard poisons Spock");
                player_score +=1;
            }
            (5,1) => {
                println!("You won! Spock vaporizes rock");
                player_score +=1;
            }
            (5,2) => {
                println!("You lose! Paper disapproves SPOCK");
                computer_score +=1;
            }
            (5,3) => {
               println!("You won! Spock smashes (or melts) scissors"); 
                player_score +=1;
            }
            (5,4) => {
                println!("You Lose! Lizard poisons Spock");
                computer_score +=1;
            }
            (5,5) => {
                println!(" Its a tie! you both chose SPOCK");
            }
            _ => println!("Error!"),
        }
        game_count +=1 ;
        println!("game count{}", game_count);
        if computer_score > player_score
        {
            println!("Computer's Score {}", computer_score);
        }
        else if computer_score < player_score
        {
            println!("Player's Score {}", player_score);
        }
        else if  computer_score == player_score
        {
            println!("Its a tie between user and cmp till next game resumes");
        }
    }
}

