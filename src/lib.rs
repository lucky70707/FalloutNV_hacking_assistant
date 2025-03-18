use std::{error::Error, fs::{self, File}, io::Write};

fn run(){
    let contents = open_saved_words_file().unwrap();
    let length: usize = 7;
    let mut words: Vec<String> = get_words_by_length(length, contents).unwrap();
}


fn overwrite_words_json(){
    
    let filename = "saved_words2.json".to_string();
    let mut file = File::create(filename);
    

}

fn open_saved_words_file() -> Result<String, Box<dyn Error>> {
    let file = "saved_words.json".to_string();
    let contents = fs::read_to_string(file)?;
    Ok(contents)
}

fn get_words_by_length(length: usize, contents: String) -> Result<Vec<String>, String> {
    let mut parsed = serde_json::Deserializer::from_str(&contents);
    let index = format!("length{length}");
    let array:  Vec<String> = parsed[index].members_mut().map(|value| value.to_string()).collect();


    Ok(array)
    
}

#[cfg(test)]
mod tests {
    use crate::{get_words_by_length, open_saved_words_file};


    #[test]
    fn test_open_saved_words_file() {
        let contents = open_saved_words_file();
        assert!(!contents.unwrap().is_empty())
    }

    #[test]
    fn test_get_words_by_length() {
        let contents = open_saved_words_file().unwrap();
        let result = get_words_by_length(7 as usize, contents).unwrap();
        let expected_result = vec!["help".to_string()];
        assert_eq!(result, expected_result);
    }
}
