use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::read_to_string;

const DEFAULT_TITLE: &str = "Shogun";
const RANDOM_TITLES_FILE: &str = "random_titles.txt";

pub(crate) fn random_search() -> String {
    let mut rng = thread_rng();
    let file_contents = read_to_string(RANDOM_TITLES_FILE).unwrap_or(String::from(DEFAULT_TITLE));
    let titles: Vec<&str> = file_contents.split("\n").collect();

    format!(
        "/search?search={}&y=2024",
        titles.choose(&mut rng).unwrap_or(&DEFAULT_TITLE)
    )
}
