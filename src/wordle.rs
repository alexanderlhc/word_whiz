use std::io;

pub fn play_cli(word_list: Vec<String>) -> io::Result<()> {
    let mut std_read = String::new();
    let mut is_playing = String::from("y");
    let mut row: [Option<char>; 5] = [None; 5];
    let mut candidates: Vec<char> = vec![];
    let mut wrong_guesses: Vec<char> = vec![];

    while is_playing == "y" {
        println!("___________");
        println!("State:");
        println!("Row: {:?}", row);
        println!("Candidates: {:?}", candidates);
        println!("Wrong guesses: {:?}", wrong_guesses);
        println!("___________");

        println!("Please enter the row, format: ...a..");
        io::stdin().read_line(&mut std_read)?;
        let row_chars: Vec<Option<char>> = std_read
            .trim()
            .chars()
            .map(|c| if c == '.' { None } else { Some(c) })
            .collect();
        std_read.clear();
        for (i, ele) in row_chars.into_iter().enumerate() {
            if i < row.len() {
                row[i] = ele;
            }
        }

        println!("Please enter candidates:");
        println!("{:?}", candidates);
        io::stdin().read_line(&mut std_read)?;
        candidates.extend(std_read.trim().chars());
        std_read.clear();

        println!("Please enter wrong guesses");
        println!("{:?}", wrong_guesses);
        io::stdin().read_line(&mut std_read)?;
        wrong_guesses.extend(std_read.trim().chars());
        std_read.clear();

        let res = wordle_solve(&row, &candidates, &wrong_guesses, &word_list);
        println!("Result: {:?}", res);

        println!(".................");

        println!("Round, to continue: y/n");
        io::stdin().read_line(&mut std_read)?;
        is_playing = std_read.trim().to_string();
        std_read.clear();
    }
    Ok(())
}

fn wordle_solve(
    row: &[Option<char>; 5],
    candidates: &[char],
    wrong_guesses: &[char],
    word_list: &[String],
) -> Vec<String> {
    word_list
        .iter()
        .filter(|&word| is_possible(word, row, candidates, wrong_guesses))
        .cloned()
        .collect()
}

fn is_possible(
    word: &str,
    row: &[Option<char>; 5],
    candidates: &[char],
    wrong_guesses: &[char],
) -> bool {
    word_match_candidate(word, row)
        && candidates.iter().all(|&c| word.contains(c))
        && wrong_guesses.iter().all(|&c| !word.contains(c))
}

fn word_match_candidate(word: &str, row: &[Option<char>; 5]) -> bool {
    word.chars()
        .enumerate()
        .all(|(i, word_char)| row.get(i).unwrap_or(&None).map_or(true, |c| c == word_char))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_test_data() -> ([Option<char>; 5], Vec<char>, Vec<String>) {
        let empty_row: [Option<char>; 5] = [None; 5];
        (empty_row, <Vec<char>>::new(), <Vec<String>>::new())
    }

    #[test]
    fn it_gives_all_words_given_no_options() {
        let (empty_row, empty_list, word_list) = empty_test_data();
        let res = wordle_solve(&empty_row, &empty_list, &empty_list, &word_list);
        assert_eq!(res, vec!["puzzle"])
    }

    #[test]
    fn it_filters_out_words_not_matching_row() {
        let (empty_row, empty_list, word_list) = empty_test_data();
        let mut row: [Option<char>; 5] = empty_row.clone();
        row[0] = Some('p');
        let res = wordle_solve(&row, &empty_list, &empty_list, &word_list);
        assert_eq!(res, <Vec<String>>::new())
    }
}
