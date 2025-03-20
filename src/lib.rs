mod serialization;




use std::{collections::HashSet, io};



pub fn run() {  
    //let unparsed_list = open_saved_words_file().unwrap();
    let instruction: String = "Write down all the words and write 'f' when finished".to_string();
    println!("{instruction}");

    let mut input = process_input();
    let length: usize = input.len();
    while input.len() < 4 ||input.len()>15{
        if input.len()>4{
            println!("word is too short to be valid");
        }else {
            println!("word is too large to be valid");
        }
        input = process_input();
        
    }
    let words: HashSet<String> = validate_words_input(input, length);
    
    //this can be done in the background with a new thread
    serialization::process_user_inputted_words(words.clone(),length);    
    
    solve(words);

    
}

fn solve(words: HashSet<String>){
    println!("Pick a word and write the word here");
    let current_word = process_input();
    println!("How many letters were correct?");
    let amount_correct = process_input().parse::<i8>().unwrap();

    let mut remaining_words = check_word_against_list(words, current_word, amount_correct);

    while remaining_words.len()>1{
        println!("Pick another word from the following list:");
        for word in &remaining_words{
            println!("{word}");
        }
        let current_word = process_input();
        println!("How many letters were correct?");
        let amount_correct = process_input().parse::<i8>().unwrap();
        remaining_words = check_word_against_list( HashSet::from_iter(remaining_words.iter().cloned()), current_word, amount_correct);
    }
    if remaining_words.len()==1{
        println!("correct answer is {}", remaining_words.get(0).unwrap()
    );
    }else{
        println!("no valid answer");
    }

}

fn check_word_against_list(
    words: HashSet<String>,
    current_word: String,
    amount_correct: i8,
) -> Vec<String> {
    let mut remaining_words_list: Vec<String> = Vec::new();

    for word in words {
        let mut overlap_count: i8 = 0;
        for i in 0..word.len() {
            if word.chars().nth(i) == current_word.chars().nth(i) {
                overlap_count += 1;
            }
        }
        if overlap_count == amount_correct {
            remaining_words_list.push(word);
        }
    }
    remaining_words_list
}



fn validate_words_input(input: String, length: usize) -> HashSet<String> {
    let mut input = input;
    let mut words: HashSet<_> = HashSet::new();
    // let mut log:String;
    //let already_exists_error= "word {} has already been inserted before.".to_string();

    while input.ne("F") {
        if validate_single_word(&input, length) {
            words.insert(input);
            //  log.push(format!(already_exists_error,input));
        }
        input = process_input();
    }

    words
}

fn validate_single_word(input: &String, length: usize) -> bool {
    if input.len() == length {
        true
    } else {
        println!("latest word is a different length than previous word(s)");
        false
    }
}

fn process_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    //the terminal will add characters even if you have only typed a single character, so we trim the input here
    input.trim().to_string().to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn input_validation_test() {
        let input = "healing".to_string();
        let length = input.len();
        assert_eq!(true,validate_single_word(&input, length))
    }

}
