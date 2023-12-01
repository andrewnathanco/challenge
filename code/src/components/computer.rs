use core::fmt;
use std::{error::Error, io::ErrorKind};

use super::{tile::*, word::get_words};
use leptos::{*,error::Result, logging::log};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::components::word::get_word;


#[derive(Debug)]
struct IgnoreTileError;

impl Error for IgnoreTileError {}

impl fmt::Display for IgnoreTileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: Ignoring tile.")
    }
}


pub async fn get_tile_from_computer(
    tiles: Vec<Tile>,
) -> Result<Tile> {
    // get all possible answers
    let words = get_words().await.unwrap_or_default();

    // get only the ones based on current tiles
    let current_word = convert_tiles_to_word(tiles.clone()).to_lowercase();
    let curr_str = current_word.as_str();
    let len_curr_str = curr_str.len();

    // now get the possible answers
    let possible_answers: Vec<String> = words.clone().into_iter().filter(|word| {
        word.starts_with(curr_str)
    }).collect();

    let mut rng = rand::thread_rng();
    let len_probable = possible_answers.len();
    if len_probable > 0 {
        let index = rng.gen_range(0..len_probable);
        let comp_word = possible_answers.get(index).unwrap();
        let comp_letter = comp_word.chars().nth(len_curr_str).unwrap_or_default();

        Ok(Tile{
            letter: comp_letter,
            author: TileAuthor::Computer,
        })
    } else {
        Ok(Tile{
            letter: '?',
            author: TileAuthor::Computer,
        })
    }
}

pub async fn get_word_for_computer() -> Result<String> {
    Ok("test".to_string())
}

























































































































