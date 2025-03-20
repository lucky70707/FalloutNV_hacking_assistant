//TODO
//function that gets all values from saved_words.json or creates saved_words.json if it doesn't exist
//the values from the array that equal the length of user words being added are to be converted into a hashset
//the hashset of current user values and old user values of the same length are to be combined upon completion of user input
//the updated list of values should be saved together with the old values of other lengths
//core functionality for the assistance should be added.
//move functionality to lib.rs and other files
//add GUI functionality using slint

//NOTE
//can test out efficacy of program with https://jetholt.com/hacking/

//the words in the minigame have lengths ranging from 4 to 15 according to https://fallout.wiki/wiki/Hacking_(Fallout:_New_Vegas)
/* 
struct WordsWithLengths{
    length4: Vec<String>,
    length5: Vec<String>,
    length6: Vec<String>,
    length7: Vec<String>,
    length8: Vec<String>,
    length9: Vec<String>,
    length10: Vec<String>,
    length11: Vec<String>,
    length12: Vec<String>,
    length13: Vec<String>,
    length14: Vec<String>,
    length15: Vec<String>,
}*/


fn main() {
        fallout_hacking_assistant::run();   
    }