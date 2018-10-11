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
    word_model.add();
    word_model.show();
}