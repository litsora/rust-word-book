use std::io;

struct Word {
    english: String,
    japanese: String,
}

struct WordModel {
    words: Vec<Word>
}

impl Word {
    fn add_word(english: String, japanese: String) -> Word {
        Word{ english, japanese}
    }
}

impl WordModel {

    fn show (&mut self) {
        println!("All words!");
        for (id, word) in self.words.iter().enumerate()  {
            println!("WordId: {}", id);
            println!("English: {}", word.english);
            println!("Japanese: {}", word.japanese);
            println!("-----------------------------");
        }
    }

    fn add(&mut self) {
        println!("Please input English.");
        let mut English = String::new();
        io::stdin().read_line(&mut English)
            .expect("Failed to read line");
        println!("Please input Japanese.");
        let mut Japanese = String::new();
        io::stdin().read_line(&mut Japanese)
            .expect("Failed to read line");
        self.words.push(Word::add_word(English.trim().to_string(), Japanese.trim().to_string()))
    }
    fn delete(&mut self) {
        println!("Please input delete word.");
        self.show();
    }
}

fn main(){
    let mut word_model = WordModel{ words: Vec::new() };
    loop {
        println!("Please input mode.");
        println!("1: Add word.\n\
              2: Delete word.\n\
              3: Show all words.\n\
              4: exit."
        );
        let mut input_mode = String::new();
        io::stdin().read_line(&mut input_mode)
            .expect("Failed to read line");
        let mut input_mode_num: i32 = match input_mode.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Can't parse.");
                continue
            },
        };
        match input_mode_num {
            1 => word_model.add(),
            2 => word_model.delete(),
            3 => word_model.show(),
            4 => break,
            _ => println!("You must write number.")
        }
    }
}