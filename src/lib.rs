use isahc::prelude::*;
use std::io::*;

pub fn do_main_loop(){
    let response = get_word().unwrap();

    let word = parse_response(&response);
    let mut guessed = generate_guessed(word);
    let mut indices;
    let max_count = 10;
    let mut guessed_chars = Vec::new();

    let mut guess;

    let mut my_char: char;
    loop {
        display_guessed(&guessed);
        guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let length = guess.chars().count();
        if length > 2 {
            println!("That's not one char!");
            continue;
        }
        my_char = guess.chars().collect::<Vec<char>>()[0];
        match get_occurrences(my_char, word) {
            Some(a) => indices = a,
            None => {
                guessed_chars = handle_not_inside(my_char,guessed_chars);
                if guessed_chars.len() >= max_count{
                    println!("Sorry, that's too many turns! The word was {}!",word);
                    break;
                }

                continue;
            }
        }

        // fill in correct guesses
        indices.iter()
            .for_each(|i| guessed.replace_range(*i..*i+1, &my_char.to_string()));

        //     check if right
        if guessed == word{
            println!("Congrats, you guessed that the word was {}!",word);
            break;
        }
    }
//     start menu
}


pub fn get_word() -> Result<String>{
    let mut response = isahc::get("https://random-word-api.herokuapp.com/word?number=1&swear=0")?;

    Ok(response.text()?)
}

pub fn parse_response(response:&str) -> &str{
    let parts:Vec<&str> = response.split("\"").collect();

    parts[1]
}

pub fn generate_guessed(word:&str) -> String{
    let mut output = String::new();

    for _ in 0..word.len() {
        output.push('_');
    }

    return output
}

pub fn display_guessed(guessed:&String){
    guessed.chars()
        .for_each(|c| print!("{} ",c));
    println!();
}

pub fn get_occurrences(guess:char,word:&str) -> Option<Vec<usize>>{

    let mut output = Vec::new();

    word.chars().enumerate()
        .for_each(|(i,c)|
            if c==guess {output.push(i)}
        );

    match output.len(){
        0 => None,
        _ => Some(output)
    }
}

pub fn handle_not_inside(guess:char,mut chars_guessed:Vec<char>) -> Vec<char>{
    if chars_guessed.contains(&guess){
        println!("You've already guessed {}",guess);
        return chars_guessed;
    }

    println!("{} is not in the word!",guess);
    chars_guessed.push(guess);
    chars_guessed
}

