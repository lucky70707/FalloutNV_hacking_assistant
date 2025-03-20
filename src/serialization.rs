use std::{
    collections::HashSet,
    error::Error,
    fs::{self, File},
    io::{ ErrorKind, Write},
};

use serde_derive::{Deserialize, Serialize};


///the words in the minigame have lengths ranging from 4 to 15 according to https://fallout.wiki/wiki/Hacking_(Fallout:_New_Vegas)

#[derive(Serialize, Deserialize)]
pub struct WordsByLengths {
    pub lengths: [Vec<String>; 12],
}

impl WordsByLengths {
    pub fn get(&self, length: usize) -> &Vec<String> {
        &self.lengths[length - 4]
    }
    pub fn set(&mut self, length: usize, list: Vec<String>){
        self.lengths[length-4]=list;
    }
}

//needs to deal with errors inside function, file not found error should be handled by creating the file

pub fn open_saved_words_file(file: String) -> Result<String, Box<dyn Error>> {
    match fs::read_to_string(&file) {
        Ok(value) => Ok(value),
        Err(err) => {if err.kind() == ErrorKind::NotFound{
            create_empty_file(&file);
            open_saved_words_file(file)
        }else {
            panic!("{err}");
        }}        
    }
    
}

fn create_empty_file(path: &String) {  
    println!("creating empty file {path}");
    let frumble: [Vec<std::string::String>; 12] = [
        vec![], //4
        vec![], //5
        vec![], //6
        vec![], //7
        vec![], //8
        vec![], //9
        vec![], //10
        vec![], //11
        vec![], //12
        vec![], //13
        vec![], //14
        vec![], //15
    ];
    let contents: WordsByLengths = WordsByLengths { lengths: frumble };
    let output = serialize_to_json(contents);
    
    match write_to_file(output, path.to_string()) {
        Ok(_value) => println!("file success!"),
        Err(_error) => println!("file failure"),
    }
}

pub fn serialize_to_json(words: WordsByLengths) -> String {
    
    let json_text = match serde_json::to_string_pretty(&words) {
        Err(err) => panic!("parsing failure: {err}"),
        Ok(value) => value,
    };

    json_text
}

pub fn write_to_file(output: String, path: String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let output = output;
    

    file.write_all(output.as_bytes())?;

    Ok(())
}

pub fn parse_json_struct(contents: String) -> WordsByLengths {
    match serde_json::from_str(&contents) {
        Ok(value) => return value,
        Err(err) => panic!("parsing error: {err}"),
    };
}

pub fn process_user_inputted_words(words_inputted: HashSet<String>, word_length: usize) {
    let file = "saved_words.json".to_string();
    let contents = match open_saved_words_file(file.clone()) {
        Ok(value) => value,
        Err(err) => panic!("{}", err),
    };
    let mut all_words: WordsByLengths = parse_json_struct(contents);
    let mut unique_words: HashSet<String> =
        HashSet::from_iter(all_words.get(word_length).iter().cloned());

    let unique_words_length_at_start = unique_words.len();
    for word in &words_inputted {
        unique_words.insert(word.clone());
    }
    let unique_words_at_end = unique_words.len();

    if unique_words_length_at_start < unique_words_at_end {
        let uniqe_words_vec: Vec<String> = Vec::from_iter(unique_words.iter().cloned());

        all_words.set(word_length, uniqe_words_vec);//I am watching you!
        
       

        let output = serialize_to_json(all_words);

        match write_to_file(output, file) {
            Ok(()) => (),
            Err(err) => panic!("oh no {err}"),
        };
    }
}

#[cfg(test)]
mod tests {
    

    use super::*;

    /*
    more or less an integration test
    opens file test.json
    gets content from test.json
    extracts multiple lists from it
    turns list with the same length into a hashmap
    adds words inputted to the hashmap
    turns hashmap back into vec
    writes all lists into test.json
     */
    #[ignore]
    #[test]
    fn alter_file_test() {
        let file = "test3.json".to_string();
        let word = "legions".to_string();
        let word_length = word.len();
        let words_inputted = vec![word, "healing".to_string()];
        let contents = match open_saved_words_file(file.clone()) {
            Ok(value) => value,
            Err(err) => panic!("{}", err),
        };

        let mut words_by_length: WordsByLengths = parse_json_struct(contents);

        let mut unique_words: HashSet<String> =
            HashSet::from_iter(words_by_length.get(word_length).iter().cloned());
        let unique_words_length_at_start = unique_words.len();
        for word in words_inputted {
            println!("{word}");
            unique_words.insert(word.to_string());
        }
        let uniqe_words_at_end = unique_words.len();

        if unique_words_length_at_start < uniqe_words_at_end {
            let uniqe_words_vec: Vec<String> = Vec::from_iter(unique_words.iter().cloned());

            words_by_length.set(word_length, uniqe_words_vec);

            let output = serialize_to_json(words_by_length);

            match write_to_file(output, file) {
                Ok(()) => (),
                Err(err) => panic!("oh no {err}"),
            };
        }
    }


    #[test]

    fn validate_json_serialization() {
        let mut words: HashSet<String> = HashSet::new();
        words.insert("ceiling".to_string());
        words.insert("special".to_string());
        words.insert("looking".to_string());
        let length: usize = "ceiling".len();
       // serialize_to_json(words, length);
    }
    #[ignore]
    #[test]
    fn construct_test_file() {
        let path = "saved_words.json".to_string();

        let frumble: [Vec<std::string::String>; 12] = [
            vec!["fire".to_string(), "soap".to_string()],      //4
            vec!["truck".to_string()],                         //5
            vec!["wordle".to_string()],                        //6
            vec!["enemies".to_string(), "bracer".to_string()], //7
            vec![],                              //8
            vec![],                              //9
            vec![],                              //10
            vec![],                              //11
            vec![],                              //12
            vec![],                              //13
            vec![],                              //14
            vec![],                              //15
        ];
        let contents: WordsByLengths = WordsByLengths { lengths: frumble };
        let output = serialize_to_json(contents);
        
        assert!(write_to_file(output, path).is_ok())
    }

}


