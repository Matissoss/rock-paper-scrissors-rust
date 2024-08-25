use rand::{self, Rng};
use std::io;
use std::fs;
use std::fs::read_to_string;
fn main() {
        println!("____________________________________________________________________________________________________");
        println!("|                               Rock Paper Scrissors in Rust                                       |");
    let mut score : u32 = 0;
    loop{
        println!("|__________________________________________________________________________________________________|");
        println!("| 1 = Rock | 2 = Paper | 3 = Scrissors | 0 = ending program | 4 = saving score | 5 = loading score |");
        let mut input_string : String = "".to_string();
        print!("| ");
        io::stdin().read_line(&mut input_string).expect("Error with input");

        let input : u8 = input_string.trim().parse().expect("Error with parsing");

        let another_input : u8 = rand::thread_rng().gen_range(1..4);
        if input == 0{
            break;
        }
        else if input == 4{
            fs::write("score.txt", score.to_string()).expect("Error with writing file score.txt");
            println!("|__________________________________File Succesfully Saved________________________________________|");
        }
        else if input == 5{
            let f : String =  read_to_string("score.txt").expect("Error with loading score. Probably score.txt file missing");
            score = f.trim().parse().expect("Error with parsing");
        }

        if input == another_input{
            println!("Draw!");
        }
        else if input == 1 && another_input == 2{
            println!("You losed!");
        }
        else if input == 1 && another_input == 3{
            println!("You won!");
            score += 1;
        }
        else if input == 2 && another_input == 1{
            println!("You won!");
            score += 1;
        }
        else if input == 2 && another_input == 3{
            println!("You losed!");
        }
        else if input == 3 && another_input == 1{
            println!("You losed!");
        }
        else if input == 3 && another_input == 2{
            println!("You won!");
            score += 1;
        }

        let mut name_of_tool : String = "".to_string();

        if another_input == 1{
            name_of_tool = "Rock".to_string();
        }
        else if another_input == 2
        {
            name_of_tool = "Paper".to_string();
        }
        else if another_input == 3{
            name_of_tool = "Scrissors".to_string();
        }

        if input != 4 || input != 5 || input != 0{
            println!("| Enemy tool was: {}", name_of_tool);
        }
        

    }
}


