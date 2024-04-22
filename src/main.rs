use word_list::{filter_char_count, read_word_list};

mod splice;
mod word_list;
mod wordle;

fn main() {
    // let dictionary: Vec<String> = filter_char_count(read_word_list().unwrap(), 5);
    // let _ = wordle::play_cli(word_list);
    let dictionary: Vec<String> = read_word_list().unwrap();
    let words = vec!["de", "en", "ret", "ri", "so", "te"]
        .iter()
        .map(|w| w.to_string())
        .collect();
    let _ = splice::play_cli(words, dictionary);
}
