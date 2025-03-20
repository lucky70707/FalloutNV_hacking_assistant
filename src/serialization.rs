use std::{
    collections::HashSet,
    error::Error,
    fs::{self, File},
    io::Write,
};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WordsByLengths {
     lengths: [Vec<String>; 12],
}

impl WordsByLengths {
    fn get(&self, length: usize) -> &Vec<String> {
        &self.lengths[length - 4]
    }    
}

//needs to deal with errors inside function, file not found error should be handled by creating the file
#[allow(dead_code)]
pub fn open_saved_words_file(file: String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;
    Ok(contents)
}
#[allow(dead_code)]
pub fn serialize_to_json(words: WordsByLengths) -> String {
    let json_text = match serde_json::to_string(&words) {
        Err(err) => panic!("parsing failure: {err}"),
        Ok(value) => value,
    };

    //println!("{json_text}");
    //println!("test: {test}");

    json_text
}
#[allow(dead_code)]
pub fn write_to_file(output: String, path:String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let output = output;
    println!("{output}");

    file.write_all(output.as_bytes())?;
    Ok(())
}

fn parse_json_struct(contents: String)->WordsByLengths{
    match serde_json::from_str(&contents){
        Ok(value)=> return value,
        Err(err)=>panic!("parsing error: {err}")
    };
}

#[allow(dead_code)]
fn get_words_by_length(length: usize, contents: String) -> Result<Vec<String>, serde_json::Error> {
    let contents = contents;
    let string = format!("r#{}", contents);

    let lengths: WordsByLengths = serde_json::from_str(&contents)?;

    let array: Vec<String> = lengths.get(length).to_vec();

    Ok(array)
}

#[cfg(test)]
mod tests {
    use std::{fmt::format, process::Output};

    use super::*;

    /*opens file test.json
    gets content from test.json
    extracts multiple lists from it
    turns list with the same length into a hashmap
    adds words inputted to the hashmap
    turns hashmap back into vec
    writes all lists into test.json
     */
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
        
        let mut words_by_length = parse_json_struct(contents);       
       
                
        let mut unique_words: HashSet<String> = HashSet::from_iter(words_by_length.get(word_length).iter().cloned());
        let unique_words_length_at_start = unique_words.len();
        for word in words_inputted {
            println!("{word}");
            unique_words.insert(word.to_string());
        }
        let uniqe_words_at_end = unique_words.len();

        if unique_words_length_at_start < uniqe_words_at_end{
            let uniqe_words_vec: Vec<String> = Vec::from_iter(unique_words.iter().cloned());

            words_by_length.lengths[0]=uniqe_words_vec;

            let output = serialize_to_json(words_by_length);

            match write_to_file(output, file){
                Ok(())=>(),
                Err(err)=>panic!("oh no {err}")
            };
        }


    }

    #[test]
    fn construct_test_file() {
        let path = "test3.json".to_string();

        let frumble: [Vec<std::string::String>; 12] = [
            vec!["fire".to_string(),"soap".to_string()],                         //4
            vec!["truck".to_string()],                              //5
            vec!["wordle".to_string()],                              //6
            vec!["enemies".to_string(), "bracer".to_string()], //7
            vec!["".to_string()],                              //8
            vec!["".to_string()],                              //9
            vec!["".to_string()],                              //10
            vec!["".to_string()],                              //11
            vec!["".to_string()],                              //12
            vec!["".to_string()],                              //13
            vec!["".to_string()],                              //14
            vec!["".to_string()],                              //15
        ];
        let contents: WordsByLengths = WordsByLengths { lengths: frumble };
        let output = serialize_to_json(contents);
        //println!("{output}");
        match write_to_file(output, path){
            Ok(value)=>println!("file success!"),
            Err(error)=>println!("file failure")
        }
    }
}

/*{
    "length4":[],
    "length5":[],
    "length6":[],
    "length7":[
        "leaving",
        "healing",
        "decries",
        "quality",
        "siphons",
        "leading",
        "driving",
        "special",
        "looking",
        "imagine",
        "dealing",
        "ceiling",
        "sealing",
        "slavers",
        "bracers",
        "enemies"
        ] ,
    "length8":[],
    "length9":[],
    "length10":[],
    "length11":[],
    "length12":[],
    "length13":[],
    "length14":[],
    "length15":[]
} */
