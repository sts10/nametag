use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

pub fn get_usernames(
    list_file_path: Option<PathBuf>,
    number_to_print: usize,
    maximum_length: usize,
    title_case: bool,
) -> Vec<String> {
    let (word_list1, word_list2) = match list_file_path {
        Some(list_file_path) => (make_list(&list_file_path), make_list(&list_file_path)),
        None => make_default_lists(),
    };
    let mut usernames = Vec::new();
    for _n in 1..=number_to_print {
        usernames.push(make_username(
            &word_list1,
            &word_list2,
            maximum_length,
            title_case,
        ));
    }
    usernames
}
fn make_username(
    word_list1: &[String],
    word_list2: &[String],
    maximum_length: usize,
    title_case: bool,
) -> String {
    let (word1, word2) = (
        get_random_element(&word_list1),
        get_random_element(&word_list2),
    );
    // check if we need to make the words title case
    let (word1, word2) = if title_case {
        (make_title_case(&word1), make_title_case(&word2))
    } else {
        (word1, word2)
    };

    if maximum_length > 10 {
        let username = format!(
            "{}{}{}{}",
            word1.trim_end(),
            get_random_element(&["_".to_string(), "-".to_string(), "".to_string()]),
            word2.trim_end(),
            rand::thread_rng().gen_range(0, 999)
        );
        if username.len() > maximum_length {
            make_username(word_list1, word_list2, maximum_length, title_case)
        } else {
            username
        }
    } else {
        let username = format!(
            "{}{}",
            word2.trim_end(),
            rand::thread_rng().gen_range(0, 999)
        );
        if username.len() > maximum_length {
            make_username(word_list1, word_list2, maximum_length, title_case)
        } else {
            username
        }
    }
}

fn make_list(file_path: &PathBuf) -> Vec<String> {
    let file_input: Vec<String> = match read_by_line(file_path.to_path_buf()) {
        Ok(r) => r,
        Err(e) => panic!("Error reading word list file: {}", e),
    };
    let mut word_list: Vec<String> = vec![];
    for line in file_input {
        word_list.push(line);
    }
    word_list
}

fn make_default_lists() -> (Vec<String>, Vec<String>) {
    (
        include_str!("../word-lists/adjectives.txt")
            .split('\n')
            .map(|w| w.to_string())
            .collect(),
        include_str!("../word-lists/nouns.txt")
            .split('\n')
            .map(|w| w.to_string())
            .collect(),
    )
}

fn get_random_element(word_list: &[String]) -> String {
    match word_list.choose(&mut rand::thread_rng()) {
        Some(word) => word.to_string(),
        None => panic!("Couldn't pick a random word"),
    }
}

fn read_by_line<T: FromStr>(file_path: PathBuf) -> io::Result<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut vec = Vec::new();
    let f = match File::open(file_path) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line?.parse() {
            Ok(l) => vec.push(l),
            Err(e) => panic!("Error parsing line from file: {:?}", e),
        }
    }
    Ok(vec)
}

fn make_title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
