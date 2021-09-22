use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::env;
use std::io;
use std::io::Write;
use rand::seq::SliceRandom;
use rand::thread_rng;
use colored::*;

macro_rules! input {
    ($buff:expr) => {
        $buff = String::new();
        io::stdin().read_line(&mut $buff).expect("failed to readline");
    };
}

const INSTRUCTION: &str = "\"q\" - выход | \"r\" - перезапустить";

#[derive(Deserialize)]
struct Words {
    ok_msg: String,
    err_msg: String,
    words: HashMap<String, String>
}

fn load_words(path: &str) -> Result<Words, Box<dyn Error>> {
    let mut dir = env::current_dir()?;
    dir.push(path);
    let file = File::open(dir)?;
    let reader = BufReader::new(file);
    let words: Words = serde_json::from_reader(reader)?;
    Ok(words)
}

fn main(){
    let words = load_words("words.json").unwrap();
    let mut keys: Vec<String> = words.words.keys().cloned().collect();
    keys.sort_by(|a, b| {
        let n1 = a.split(".").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
        let n2 = b.split(".").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
        return n1.cmp(&n2)
    });
    let mut shuffled_keys: Vec<String> = vec![];

    let mut answer: &String;
    let mut guess: String;
    let mut errors_count: u16 = 0;
    let mut buff: String;
    let mut from: usize;
    let mut to: usize;

    'slice: loop {
        print!("Задать промежуток? (Y | N): ");
        io::stdout().flush().unwrap();
        input!(buff);
        match buff.trim() {
            "Y" => {
                println!("Всего {} слов(а)", keys.len());
                println!("Промежуток от: ");
                input!(buff);
                from = buff.trim().parse::<usize>().unwrap_or(0);
                
                println!("Промежуток до: ");
                input!(buff);
                to = buff.trim().parse::<usize>().unwrap_or(keys.len());

                if from>=keys.len() || to<=from || to>=keys.len() {
                    println!("Некорректный промежуток!");
                    continue 'slice;
                }

                keys = keys[from-1..to].to_vec();
                break 'slice;
            },
            "N" => break 'slice,
            _ => {
                println!("Введи Y или N (yes или no)");
            }
        }
    }

    println!("\n{}\n{}\n", String::from("Удачи, солнышко!").yellow(), INSTRUCTION);

    'guess_loop: loop {
        if shuffled_keys.len() == 0 {
            shuffled_keys = keys.clone();
            shuffled_keys.shuffle(&mut thread_rng());
        }

        let choosen_word = &shuffled_keys.pop().unwrap();
        answer = words.words.get(choosen_word).unwrap();
        answer.to_lowercase();

        print!("Слово: {}\nОтвет: ", choosen_word);
        io::stdout().flush().unwrap();
        input!(guess);
        guess.to_lowercase();

        match guess.trim() {
            "q" => {
                println!("\n{}\n", "Удачи на зачете!".cyan());
                break 'guess_loop
            },
            "r" => {
                shuffled_keys = vec![];
                println!("\n{}\n", "Все получится!".magenta());
                continue 'guess_loop
            },
            _ => {
                if answer.eq_ignore_ascii_case(guess.trim()) {
                    println!("\n{} | осталось {} слов(а) | ошибок: {}\n", words.ok_msg.green(), shuffled_keys.len(), format!("{}", errors_count).red());
                } else {
                    errors_count = errors_count + 1;
                    println!("\n{} {} | осталось {} слов(а) | ошибок: {}\n", words.err_msg.red(), answer, shuffled_keys.len(), format!("{}", errors_count).red());
                }
            }
        }
    }
}