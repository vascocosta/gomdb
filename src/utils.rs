use rand::seq::SliceRandom;
use rand::thread_rng;

pub(crate) fn random_search() -> String {
    let mut rng = thread_rng();
    let titles = [
        "Shogun",
        "Fallout",
        "The Penguin",
        "The Gentlemen",
        "Ripley",
        "The Tattooist of Auschwitz",
        "Dark Matter",
        "3 Body Problem",
        "Sugar",
        "Masters of the Air",
        "The Brothers Sun",
        "The Walking Dead: The Ones Who Live",
        "WondLa",
        "Those About to Die",
        "The Edge of Sleep",
        "Griselda",
    ];

    format!(
        "/search?search={}&y=2024",
        titles.choose(&mut rng).unwrap_or(&"")
    )
}
