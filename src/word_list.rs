use csv::ReaderBuilder;

pub fn read_word_list() -> Result<Vec<String>, String> {
    let file_path = String::from("ordliste.tsv");
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(file_path)
        .expect("COULD NOT READ WORDLIST");

    let mut columns = vec![];

    for result in reader.records() {
        let row = result.expect("ROW IS ERRORING");
        if let Some(column) = row.get(4) {
            columns.push(column.to_string());
        }
    }

    Ok(columns)
}

pub fn filter_char_count(words: Vec<String>, count: usize) -> Vec<String> {
    words
        .into_iter()
        .filter(|w| w.chars().count() == count)
        .collect()
}
