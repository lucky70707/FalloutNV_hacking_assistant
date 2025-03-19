use std::{collections::HashSet, io};

fn main() {
    let instruction: String = "Write down all the words and write 'f' when finished".to_string();
    println!("{instruction}");
    let mut words: HashSet<_> = HashSet::new();
    let input = process_input();
    let length: usize = input.len();

    words = validate_words_input(input, length);

    println!("The words included are: ");
    for word in words {
        println!("{word}");
    }
}

fn validate_words_input(input: String, length: usize) -> HashSet<String> {
    let mut input = input;
    let mut words: HashSet<_> = HashSet::new();

    while input.ne("f") {
        if validate_single_word(&input, length){
            words.insert(input);
        }
        input = process_input();
    }

    words
}

fn validate_single_word(input: &String, length:usize)->bool{
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
    #[test]
    fn input_validation_test() {
        let input = "healing".to_string();
        let length = input.len();
    }
}
