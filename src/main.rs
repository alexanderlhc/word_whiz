use word_list::{filter_char_count, read_word_list};
use wordle::play_cli;

mod word_list;
mod wordle;

fn main() {
    let word_list: Vec<String> = filter_char_count(read_word_list().unwrap(), 5);
    let _ = play_cli(word_list);
}
