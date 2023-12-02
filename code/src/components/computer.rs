use core::fmt;
use std::error::Error;

use super::{tile::*, word::get_words};
use leptos::{error::Result, logging::log};
use rand::Rng;


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
) -> Result<Vec<Tile>> {
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

    let possible_answers_where_the_computer_wins: Vec<String> = possible_answers.clone().into_iter().filter(|word| {
        let len_word = word.len();
        len_curr_str % 2 == len_word % 2
    }).collect();

    let mut rng = rand::thread_rng();
    let len_possible_wins = possible_answers_where_the_computer_wins.len();
    let len_all_possible = possible_answers.len();
    log!("all possible: {:?}", possible_answers);
    log!("all wins: {:?}", possible_answers_where_the_computer_wins);
    if len_possible_wins > 0 {
        let index = rng.gen_range(0..len_possible_wins);
        let comp_word = possible_answers_where_the_computer_wins.get(index).unwrap();
        let comp_letter = comp_word.chars().nth(len_curr_str).unwrap_or_default();
        Ok(vec![Tile{
            letter: comp_letter,
            author: TileAuthor::Computer,
        }])
    } else {
        if len_all_possible > 0 {
            let index = rng.gen_range(0..len_all_possible);
            let comp_word = possible_answers.get(index).unwrap();
            let comp_letter = comp_word.chars().nth(len_curr_str).unwrap_or_default();

            return Ok(vec![Tile{
                letter: comp_letter,
                author: TileAuthor::Computer,
            }])
        }

        Ok(vec![Tile{
            letter: '?',
            author: TileAuthor::Computer,
        }])
    }
}

