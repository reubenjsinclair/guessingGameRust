use std::io::stdin;

fn main() {
    println!("Welcome to the hangman game!");
    let mut option= String::new();
    loop {
        println!("\nPress q to leave, or any key to start!");
        stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        if option.trim() == "q"{
            println!("Okay, bye!");
            break;
        }
        guessing_game::do_main_loop();
    }
}