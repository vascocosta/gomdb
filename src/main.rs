use anyhow::{Context, Result};
use axum::{extract::Query, routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use models::OmDb;
use std::collections::HashMap;
use std::env;
use std::io;
use std::time::Duration;
use utils::random_search;

mod models;
mod utils;

const ADDRESS: &str = "0.0.0.0:3000";
const OMDB_API_URL: &str = "https://www.omdbapi.com/";
const TIMEOUT: u64 = 10;

async fn omdb(search: &str, year: Option<&String>) -> Result<Markup> {
    let omdb: OmDb = tokio::time::timeout(
        Duration::from_secs(TIMEOUT),
        reqwest::get(format!(
            "{}/?apikey={}&t={}&plot=full&y={}",
            OMDB_API_URL,
            env::var("OMDB_KEY")?,
            search,
            year.unwrap_or(&String::new())
        )),
    )
    .await
    .context("Timeout while fetching data")?
    .context("Could not connect to server")?
    .json()
    .await
    .context("Could not decode data")?;

    let url = format!("https://www.imdb.com/title/{}", omdb.imdb_id);

    Ok(html! {
        ul class="pt-4 list-group list-group-flush" {
            li class="list-group-item text-primary-emphasis" {
                span class="text-warning-emphasis" { "Title: " }
                span class="text-primary-emphasis" { (omdb.title) }
            }
            li class="list-group-item text-primary-emphasis" {
                span class="text-warning-emphasis" { "Year: " }
                span class="text-primary-emphasis" { (omdb.year) }
            }
            li class="list-group-item text-primary-emphasis" {
                span class="text-warning-emphasis" { "Genre: " }
                span class="text-primary-emphasis" { (omdb.genre) }
            }
            li class="list-group-item text-primary-emphasis" {
                span class="text-warning-emphasis" { "Director: " }
                span class="text-primary-emphasis" { (omdb.director) }
            }
            li class="list-group-item text-primary-emphasis" {
                span class="text-warning-emphasis" { "Actors: " }
                span class="text-primary-emphasis" { (omdb.actors) }
            }
            li class="list-group-item text-primary-emphasis" {
                span class="text-warning-emphasis" { "Runtime: " }
                span class="text-primary-emphasis" { (omdb.runtime) }
            }
            li class="list-group-item text-primary-emphasis" {
                span class="text-warning-emphasis" { "Seasons: " }
                span class="text-primary-emphasis" { (omdb.total_seasons) }
            }
            li class="list-group-item text-primary-emphasis" {
                span class="text-warning-emphasis" { "Language: " }
                span class="text-primary-emphasis" { (omdb.language) }
            }
            li class="list-group-item text-primary-emphasis" {
                span class="text-warning-emphasis" {  "IMDB Rating: " }
                span class="text-primary-emphasis" { (omdb.imdb_rating) }
            }
            li class="list-group-item text-primary-emphasis" {
                span class="text-warning-emphasis" { "Awards: " }
                span class="text-primary-emphasis" { (omdb.awards) }
            }
        }
        p class="pt-4 fs-4 text-secondary-emphasis" { (omdb.plot) }
        a href=(url) target="_blank" { img height="32" width="32" src="https://www.gluonspace.com/files/imdb.png"; (url) }
    })
}

async fn search(Query(params): Query<HashMap<String, String>>) -> Markup {
    match omdb(
        params.get("search").unwrap_or(&String::new()),
        params.get("year"),
    )
    .await
    {
        Ok(markup) => markup,
        Err(error) => html! {
            p class="text-danger-emphasis" { "Error: "(error) }
        },
    }
}

async fn root() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-bs-theme="dark" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link
                    href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css"
                    rel="stylesheet"
                    integrity="sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH"
                    crossorigin="anonymous";
                script
                    src="https://unpkg.com/htmx.org@2.0.3"
                    integrity="sha384-0895/pl2MU10Hqc6jd4RvrthNlDiE9U1tWmX7WRESftEDRosgxNsQG/Ze9YMRzHq"
                    crossorigin="anonymous" {}
                title { "GOMDb" }
            }
            body class="bg-secondary-subtle" {
                div class="container d-flex justify-content-center mt-5" {
                    div class="w-75" {
                        h1 class="text-warning-emphasis" { "🎬 Gluon Open Movie Database" }
                        div class="htmx-indicator" { p class="text-success-emphasis" { "searching..." } }
                        form
                            class="form"
                            hx-get="/search"
                            hx-target="#results"
                            hx-trigger="input changed delay:500ms, search"
                            hx-indicator=".htmx-indicator" {
                            input hx-get=(random_search()) hx-target="#results" hx-trigger="load" type="hidden";
                            div class="container" {
                                div class="row" {
                                    div class="col-9" {
                                        input class="form-control" name="search" placeholder="search..." type="search";
                                    }
                                    div class="col" {
                                        input class="form-control" name="year" placeholder="year..." type="search";
                                    }
                                }
                            }
                        }
                        div id="results" {}
                    }
                }
                script
                    src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"
                    integrity="sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz"
                    crossorigin="anonymous" {}
            }
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/search", get(search));

    let listener = tokio::net::TcpListener::bind(ADDRESS).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
