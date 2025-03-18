use std::io;


fn main() {
    let instruction: String = "Write down all the words and write 'f' when finished".to_string();
    println!("{instruction}"); 
    let input:String =    process_input();
    let mut words: Vec<String>= vec![];
    let length = input.len();

    if input.ne("f") {
        words = validate_words_input(length);
    }
    


    println!("The words included are: ");
    for word in words {
        println!("{word}");        
    }  


}

fn validate_words_input(length: usize) -> Vec<String>{
    let mut input:String =    process_input();
    let mut words: Vec<String>= vec![];
    while input.ne("f"){
        input = process_input();
        if input.ne("f"){
            if input.len()==length{
                words.push(input.clone());
            }
        }
    }
    words


}

fn process_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    //the terminal will add characters even if you have only typed a single character, so we trim the input here
    input.trim().to_string()
}


