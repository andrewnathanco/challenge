use std::collections::HashSet;

use leptos::{error::Result, *, logging::log};
use csv::ReaderBuilder;

use super::{tile::{Tile, convert_tiles_to_word}, keyboard::get_all_letters};

pub async fn get_words() -> Result<Vec<String>> {
    // make the request
    let response = reqwasm::http::Request::get(&format!(
        "/valid_words.csv",
    ))
    .send()
    .await?;


    if response.status() == 200 {
        let csv_data = response.text().await?;
        let mut csv_reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());

        let available_words:Vec<String> = csv_reader
        .records()
        .into_iter()
        .filter_map(|data| data.ok())
        .map(|data| {
            data.get(0).unwrap_or_default().to_string()
        })
        .collect();
        
        return Ok(available_words)
    }

    Ok(["computer".to_string()].to_vec())
}

pub async fn get_word() -> Result<String> { 
    let avail_words = get_words().await?;
    Ok(avail_words[0].to_string())
}

pub async fn get_available_letters(
    tiles: Vec<Tile>
) -> Result<Vec<char>> {
    let mut letters :HashSet<char>=  HashSet::new();
    let curr_word = convert_tiles_to_word(tiles);
    let len_curr_word = curr_word.len();
    for word in get_words().await.unwrap() {
        if word.starts_with(&curr_word) {
            let avail_lett = word.to_uppercase().chars().nth(len_curr_word).unwrap_or_default();
            letters.insert(avail_lett);
        }
    }

    Ok(letters.into_iter().collect())
}











































