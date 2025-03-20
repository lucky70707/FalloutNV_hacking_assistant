mod serialization;

use std::{
    
    error::Error,
    fs::{self, File},
    io::Write,
};


use std::{collections::HashSet, io};

use serde_json::json;

pub fn run() {
    //let unparsed_list = open_saved_words_file().unwrap();
    let instruction: String = "Write down all the words and write 'f' when finished".to_string();
    println!("{instruction}");

    let input = process_input();
    let length: usize = input.len();

    let words: HashSet<String> = validate_words_input(input, length);

    println!("The words included are: ");
    for word in &words {
        println!("{word}");
    }

    let output = serialize_to_json(words.clone(), length);
    match write_to_file(output){
        Ok(())=>(),
        Err(error)=>println!("problem writing to file: {error}"),
    };
    println!("Pick a word and write the word here");
    let current_word = process_input();
    println!("How many letters were correct?");
    let amount_correct = process_input().parse::<i8>().unwrap();

    let remaining_words = check_word_against_list(words, current_word, amount_correct);

    for word in &remaining_words {
        println!("{word}")
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

/*fn overwrite_words_json() {
    let filename = "saved_words2.json".to_string();
    let file = File::create(filename);
}*/

//needs to deal with errors inside function, file not found error should be handled by creating the file
fn open_saved_words_file() -> Result<String, Box<dyn Error>> {
    let file = "saved_words.json".to_string();
    let contents = fs::read_to_string(file)?;
    Ok(contents)
}

fn serialize_to_json(words: HashSet<String>, length: usize) -> String {
    let words: Vec<String> = words.into_iter().collect();
    let test = serde_json::Value::from(words);
    let json_text = json!({
        format!("length{length}"):
            test
    });

    //println!("{json_text}");
    //println!("test: {test}");

    json_text.to_string()
}

fn write_to_file(output: String) -> std::io::Result<()> {
    let mut file = File::create("test.json")?;
    let output = output;
    println!("{output}");
    file.write_all(output.as_bytes())?;
    Ok(())
}

fn validate_words_input(input: String, length: usize) -> HashSet<String> {
    let mut input = input;
    let mut words: HashSet<_> = HashSet::new();
    // let mut log:String;
    //let already_exists_error= "word {} has already been inserted before.".to_string();

    while input.ne("f") {
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
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_saved_words_file() {
        let contents = open_saved_words_file();
        assert!(!contents.unwrap().is_empty())
    }

    #[test]
    fn input_validation_test() {
        let input = "healing".to_string();
        let length = input.len();
        assert_eq!(true,validate_single_word(&input, length))
    }

    #[test]

    fn validate_json_serialization() {
        let mut words: HashSet<String> = HashSet::new();
        words.insert("ceiling".to_string());
        words.insert("special".to_string());
        words.insert("looking".to_string());
        let length: usize = "ceiling".len();
        serialize_to_json(words, length);
    }
}
