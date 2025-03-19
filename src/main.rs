use std::collections::hash_set;
use std::io::Write;
use std::{collections::HashSet, fs::File, io};

use serde_json::json;

fn main() {
    let instruction: String = "Write down all the words and write 'f' when finished".to_string();
    println!("{instruction}");

    let input = process_input();
    let length: usize = input.len();

    let words: HashSet<String> = validate_words_input(input, length);

    println!("The words included are: ");
    for word in &words {
        println!("{word}");
    }

    let output = serialize_to_json(words, length);
    write_to_file(output);
}

fn serialize_to_json(words: HashSet<String>, length: usize)->String{
    let json_text = json!({
        format!("length{length}"):[
            "quality",
            "cleaning",
            "enemies"
        ]
    });

    println!("{json_text}");

    
    "hi".to_string()
}

fn write_to_file(output: String) -> std::io::Result<()>{
    let mut file = File::create("test.txt")?;
    let output=output;   
    println!("{output}");
    file.write_all(output.as_bytes())?;
    Ok(())
}

fn validate_words_input(input: String, length: usize) -> HashSet<String> {
    let mut input = input;
    let mut words: HashSet<_> = HashSet::new();
    let mut log:String;
    let already_exists_error= "word {} has already been inserted before.".to_string();

    while input.ne("f") {
        if validate_single_word(&input, length) {
            words.insert(input);
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
    use std::collections::HashSet;

    use crate::serialize_to_json;

    #[test]
    fn input_validation_test() {
        let input = "healing".to_string();
        let length = input.len();
    }

    #[test]
    fn validate_json_serialization(){
        let words = HashSet::new();
        let length : usize = 8;
        let output = serialize_to_json(words, length);
    }
}
