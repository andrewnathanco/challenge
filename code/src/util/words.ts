import { Tile, TileAuthor } from "../components/tiles/tiles";
import words from "./valid_words.json";

function get_available_letters(curr_word: string): string[] {
  const letters = new Set<string>();
  for (const word of words) {
    if (word.toUpperCase().startsWith(curr_word.toUpperCase())) {
      const avail_lett = word.toUpperCase().charAt(curr_word.length) || "";
      letters.add(avail_lett);
    }
  }

  return Array.from(letters);
}

function get_tiles_from_computer(tiles: Tile[]): Tile[] {
  // get only the ones based on current tiles
  const current_word = tiles.map((tile) => tile.letter.toUpperCase()).join("");

  // now get the possible answers
  const all_possible: string[] = words.filter((word) =>
    word.toUpperCase().startsWith(current_word)
  );

  const possible_answers_where_comp_wins: string[] = all_possible.filter(
    (word) => {
      return current_word.length % 2 === word.length % 2;
    }
  );

  let comp_letter = "?";
  if (possible_answers_where_comp_wins.length > 0) {
    const index: number = Math.floor(
      Math.random() * possible_answers_where_comp_wins.length
    );
    const comp_word: string = possible_answers_where_comp_wins[index];
    comp_letter = comp_word.charAt(current_word.length) || "";

    return [
      {
        letter: comp_letter,
        author: TileAuthor.Computer,
      },
    ];
  } else {
    if (all_possible.length > 0) {
      const index: number = Math.floor(Math.random() * all_possible.length);
      const comp_word: string = all_possible[index];
      comp_letter = comp_word.charAt(current_word.length) || "";
    }

    return [
      {
        letter: comp_letter,
        author: TileAuthor.Computer,
      },
    ];
  }
}

export { get_available_letters, get_tiles_from_computer };
