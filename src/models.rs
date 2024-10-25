use serde::Deserialize;

fn default_value() -> String {
    String::from("N/A")
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OmDb {
    #[serde(default = "default_value")]
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Rated")]
    pub rated: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Released")]
    pub released: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Runtime")]
    pub runtime: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Genre")]
    pub genre: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Director")]
    pub director: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Writer")]
    pub writer: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Actors")]
    pub actors: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Plot")]
    pub plot: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Awards")]
    pub awards: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Poster")]
    pub poster: String,
    #[serde(rename = "Ratings")]
    pub ratings: Option<Vec<Rating>>,
    #[serde(default = "default_value")]
    #[serde(rename = "Metascore")]
    pub metascore: String,
    #[serde(default = "default_value")]
    pub imdb_rating: String,
    #[serde(default = "default_value")]
    pub imdb_votes: String,
    #[serde(default = "default_value")]
    #[serde(rename = "imdbID")]
    pub imdb_id: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(default = "default_value")]
    #[serde(rename = "DVD")]
    pub dvd: String,
    #[serde(default = "default_value")]
    pub total_seasons: String,
    #[serde(rename = "BoxOffice")]
    #[serde(default = "default_value")]
    pub box_office: String,
    #[serde(rename = "Production")]
    #[serde(default = "default_value")]
    pub production: String,
    #[serde(rename = "Website")]
    #[serde(default = "default_value")]
    pub website: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Response")]
    pub response: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    #[serde(default = "default_value")]
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(default = "default_value")]
    #[serde(rename = "Value")]
    pub value: String,
}
