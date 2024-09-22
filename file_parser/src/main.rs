use std::env;
use std::fs;

fn main(){
    let file_path = "test1.txt";

    let contents = fs::read_to_string(file_path).expect("Could not find file");

    let words = contents.split(" ");

    let mut word_vec: Vec<&str> = Vec::new();

    for word in words{
        word_vec.push(word);
    }

    
// println!("{:?}", word_vec);

 let mut the_count: i32 = 0;


 for x in word_vec{
    if x.trim() == "the" || x .trim() == "The"{
        the_count = the_count + 1;
    }
 }
  
  println!("there are : {} the or The in {}", the_count, file_path);
    
}

/*

for (i, count) in letter_count.iter().enumerate() {
        let letter = (b'a' + i as u8) as char;
        let upper_letter = (b'A' + i as u8) as char;
        println!("{}{} : {}", letter, upper_letter, count);
    }
  for letter in letters {
        println!("{}", letter);
        match letter {
            "a" => letter_count[0] = letter_count[0] + 1,
            "b" => letter_count[1] = letter_count[1] + 1,
            "c" => letter_count[2] = letter_count[2] + 1,
            "d" => letter_count[3] = letter_count[3] + 1,
            "e" => letter_count[4] = letter_count[4] + 1,
            "f" => letter_count[5] = letter_count[5] + 1,
            "g" => letter_count[6] = letter_count[6] + 1,
            "h" => letter_count[7] = letter_count[7] + 1,
            "i" => letter_count[8] = letter_count[8] + 1,
            "j" => letter_count[9] = letter_count[9] + 1,
            "k" => letter_count[10] = letter_count[10] + 1,
            "l" => letter_count[11] = letter_count[11] + 1,
            "m" => letter_count[12] = letter_count[12] + 1,
            "n" => letter_count[13] = letter_count[13] + 1,
            "o" => letter_count[14] = letter_count[14] + 1,
            "p" => letter_count[15] = letter_count[15] + 1,
            "q" => letter_count[16] = letter_count[16] + 1,
            "r" => letter_count[17] = letter_count[17] + 1,
            "s" => letter_count[18] = letter_count[18] + 1,
            "t" => letter_count[19] = letter_count[19] + 1,
            "u" => letter_count[20] = letter_count[20] + 1,
            "v" => letter_count[21] = letter_count[21] + 1,
            "w" => letter_count[22] = letter_count[22] + 1,
            "x" => letter_count[23] = letter_count[23] + 1,
            "y" => letter_count[24] = letter_count[24] + 1,
            "z" => letter_count[25] = letter_count[25] + 1,
            "A" => letter_count[0] = letter_count[0] + 1,
            "B" => letter_count[1] = letter_count[1] + 1,
            "C" => letter_count[2] = letter_count[2] + 1,
            "D" => letter_count[3] = letter_count[3] + 1,
            "E" => letter_count[4] = letter_count[4] + 1,
            "F" => letter_count[5] = letter_count[5] + 1,
            "G" => letter_count[6] = letter_count[6] + 1,
            "H" => letter_count[7] = letter_count[7] + 1,
            "I" => letter_count[8] = letter_count[8] + 1,
            "J" => letter_count[9] = letter_count[9] + 1,
            "K" => letter_count[10] = letter_count[10] + 1,
            "L" => letter_count[11] = letter_count[11] + 1,
            "M" => letter_count[12] = letter_count[12] + 1,
            "N" => letter_count[13] = letter_count[13] + 1,
            "O" => letter_count[14] = letter_count[14] + 1,
            "P" => letter_count[15] = letter_count[15] + 1,
            "Q" => letter_count[16] = letter_count[16] + 1,
            "R" => letter_count[17] = letter_count[17] + 1,
            "S" => letter_count[18] = letter_count[18] + 1,
            "T" => letter_count[19] = letter_count[19] + 1,
            "U" => letter_count[20] = letter_count[20] + 1,
            "V" => letter_count[21] = letter_count[21] + 1,
            "W" => letter_count[22] = letter_count[22] + 1,
            "X" => letter_count[23] = letter_count[23] + 1,
            "Y" => letter_count[24] = letter_count[24] + 1,
            "Z" => letter_count[25] = letter_count[25] + 1,
             _ => continue,
        };
    }
 */