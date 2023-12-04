import { get_available_letters } from "../../util/words";
import { Tile, TileAuthor } from "../tiles/tiles";
import { DEFAULT_LETTER, Game } from "./model";
import words from "../../util/valid_words.json";

function padZero(num: number): string {
  return num < 10 ? `0${num}` : `${num}`;
}

export function get_countdown_till_next_game(): string {
  // Get the current local time
  const now = new Date();

  // midnight EST in UTC
  const midnight = new Date(
    now.getFullYear(),
    now.getMonth(),
    now.getDate(),
    5,
    0,
    0,
    0
  );

  // Calculate the duration until midnight EST
  const durationUntilMidnight = midnight.getTime() - now.getTime();

  const hours = Math.floor(durationUntilMidnight / (1000 * 60 * 60)) + 24;
  const minutes =
    Math.floor((durationUntilMidnight % (1000 * 60 * 60)) / (1000 * 60)) + 60;
  const seconds = Math.floor((durationUntilMidnight % (1000 * 60)) / 1000) + 60;

  return `${padZero(hours)}:${padZero(minutes)}:${padZero(seconds)}`;
}

export function get_game_key() {
  const now: Date = new Date();
  const specificDate: Date = new Date(2023, 10, 26, 5, 0, 0);
  const duration: number =
    (now.getTime() - specificDate.getTime()) / (1000 * 60 * 60 * 24);

  return Math.floor(duration);
}

function get_starting_tiles_from_word(
  word: string
): [tiles: Tile[], letters: string] {
  let tiles: Tile[] = [];
  let starting_letters: string = "";

  word.split("").forEach((lett, index) => {
    if (index < 3) {
      tiles.push({
        letter: lett,
        author: TileAuthor.Computer,
      });

      starting_letters += lett;
    }
  });

  return [tiles, starting_letters];
}

export function get_todays_game(): Game {
  let today_game_key = get_game_key();
  let starting_word = words[today_game_key];
  let [starting_tiles, starting_letters] =
    get_starting_tiles_from_word(starting_word);

  return {
    game_key: today_game_key,
    starting_word,
    available_letters: get_available_letters(starting_word),
    starting_tiles,
    starting_letters,
    selected_letter: DEFAULT_LETTER,
    current_tiles: [...starting_tiles],
    sessions: [],
  };
}
