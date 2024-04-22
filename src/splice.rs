use std::io;

pub fn play_cli(words: Vec<String>, dictionary: Vec<String>) -> io::Result<()> {
    println!("Playing wordle!");
    let word_slices = words.iter().map(|w| w.as_str()).collect();
    let candidates = permutations_powered(word_slices);
    let res = candidates
        .iter()
        .filter(|c| word_list_has_word(&dictionary, c))
        .collect::<Vec<_>>();
    println!("Result: {:?}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::splice::{create_permutations, permutations_powered};

    #[test]
    fn test_can_permute_from_one_part() {
        let parts = vec!["one"];
        let res = create_permutations(parts);
        assert_eq!(res, vec!["one"]);
    }

    #[test]
    fn can_permute_from_two_parts() {
        let parts = vec!["one", "two"];
        let mut res = create_permutations(parts);
        let mut expected = vec!["onetwo", "twoone"];
        res.sort();
        expected.sort();
        assert_eq!(res, expected);
    }

    #[test]
    fn can_permute_from_three_parts() {
        let parts = vec!["one", "two", "three"];
        let mut res = create_permutations(parts);
        let mut expected = vec![
            "onetwothree",
            "onethreetwo",
            "twoonethree",
            "twothreeone",
            "threeonetwo",
            "threetwoone",
        ];
        res.sort();
        expected.sort();

        assert_eq!(res, expected);
    }

    #[test]
    fn can_permute_powered_one() {
        let parts = vec!["one"];
        let mut res = permutations_powered(parts);
        let mut expected = vec!["one"];
        res.sort();
        expected.sort();
        assert_eq!(res, expected);
    }

    #[test]
    fn can_permute_powered_two() {
        let parts = vec!["one", "two"];
        let mut res = permutations_powered(parts);
        let mut expected = vec!["one", "two", "onetwo", "twoone"];
        res.sort();
        expected.sort();
        assert_eq!(res, expected);
    }

    #[test]
    fn can_permute_powered_three() {
        let parts = vec!["one", "two", "three"];
        let mut res = permutations_powered(parts);
        let mut expected = vec![
            "one",
            "two",
            "three",
            "onetwo",
            "onethree",
            "twoone",
            "twothree",
            "threeone",
            "threetwo",
            "onetwothree",
            "onethreetwo",
            "twoonethree",
            "twothreeone",
            "threeonetwo",
            "threetwoone",
        ];
        res.sort();
        expected.sort();
        assert_eq!(res, expected);
    }
}

fn permutations_powered(parts: Vec<&str>) -> Vec<String> {
    let mut res = vec![];

    for i in 1..=parts.len() {
        for subset in combine(parts.clone(), i as i32) {
            res.extend(create_permutations(subset));
        }
    }

    res
}

fn combine(parts: Vec<&str>, len: i32) -> Vec<Vec<&str>> {
    if len == 0 {
        return vec![];
    }

    if parts.len() < len as usize {
        return vec![];
    }

    if len == 1 {
        return parts.into_iter().map(|item| vec![item]).collect();
    }

    if parts.len() == len as usize {
        return vec![parts.to_vec()];
    }

    let mut combinations = vec![];
    for i in 0..parts.len() {
        let new_parts = parts[i + 1..].to_vec();
        for mut r in combine(new_parts, len - 1) {
            r.push(parts[i]);
            combinations.push(r);
        }
    }

    combinations
}

fn create_permutations(parts: Vec<&str>) -> Vec<String> {
    let res = <Vec<String>>::new();
    let right: i32 = (parts.len() as i32) - 1;

    if right == 0 {
        return vec![parts[0].to_string()];
    }

    permutations(res, &parts, 0, right)
}

fn permutations(res: Vec<String>, parts: &[&str], left: i32, right: i32) -> Vec<String> {
    let mut res = res;

    if left == right {
        let word = parts.join("");
        res.push(word);
        return res;
    }

    for i in left..=right {
        let mut parts = parts.to_vec();
        swap(&mut parts, left as usize, i as usize);
        res = permutations(res, &parts, left + 1, right);
        swap(&mut parts, left as usize, i as usize);
    }

    res
}

fn swap(parts: &mut [&str], left: usize, i: usize) {
    let temp = parts[left];
    parts[left] = parts[i];
    parts[i] = temp;
}

//
// fn _word_match_parts(
//     words_list: Vec<String>,
//     candidate: &str,
//     parts: Vec<String>,
//     words: Vec<String>,
// ) -> Vec<String> {
//     let mut words = words;
//     if parts.is_empty() {
//         return words;
//     }
//
//     if word_list_has_word(words_list, candidate) {
//         words.push(candidate.to_string());
//     }
//
//     for part in parts {
//         let new_candidate = candidate.replace(&part, "");
//         let new_parts = parts.iter().filter(|p| p != &part).cloned().collect();
//         words.extend(_word_match_parts(
//             words_list,
//             &new_candidate,
//             new_parts,
//             words,
//         ));
//     }
//
//     words
// }
//

fn word_list_has_word(word_list: &[String], word: &str) -> bool {
    word_list.iter().any(|w| w == word)
}
