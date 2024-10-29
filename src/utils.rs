use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::read_to_string;

const DEFAULT_TITLE: &str = "Shogun";
const RANDOM_TITLES_FILE: &str = "random_titles.txt";

fn read_titles(file_contents: &str) -> Vec<&str> {
    file_contents
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .collect()
}

pub(crate) fn random_search() -> String {
    let file_contents = read_to_string(RANDOM_TITLES_FILE).unwrap_or_default();
    let titles = read_titles(&file_contents);

    format!(
        "/search?search={}",
        titles.choose(&mut thread_rng()).unwrap_or(&DEFAULT_TITLE)
    )
}
