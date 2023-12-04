import { Tile, TileAuthor } from "../components/tiles/tiles";
import words from "./valid_words.json";

function get_available_letters(curr_word: string): string[] {
  const letters = new Set<string>();
  for (const word of words) {
    if (word.toLowerCase().startsWith(curr_word.toLowerCase())) {
      const avail_lett = word.toLowerCase().charAt(curr_word.length) || "";
      letters.add(avail_lett);
    }
  }

  return Array.from(letters);
}

function get_tiles_from_computer(tiles: Tile[]): Tile[] {
  // get only the ones based on current tiles
  let current_word = tiles.map((tile) => tile.letter.toLowerCase()).join("");

  // now get the possible answers
  let all_possible: string[] = words.filter((word) =>
    word.toLowerCase().startsWith(current_word)
  );

  let possible_answers_where_comp_wins: string[] = all_possible.filter(
    (word) => {
      return current_word.length % 2 === word.length % 2;
    }
  );

  let comp_letter = "";

  // this is the case where the user got themselves trapped and there's no winning
  if (possible_answers_where_comp_wins.length == all_possible.length) {
    // if there is a word that is one letter away from being found pick that one
    for (let word of all_possible) {
      if (word.length - 2 == current_word.length) {
        const index: number = Math.floor(
          Math.random() * possible_answers_where_comp_wins.length
        );

        const losing_word: string = possible_answers_where_comp_wins[index];
        let losing_tiles: Tile[] = [
          {
            letter: losing_word[losing_word.length - 2],
            author: TileAuthor.Computer,
          },
          {
            letter: losing_word[losing_word.length - 1],
            author: TileAuthor.User,
          },
        ];

        return losing_tiles;
      }
    }
  }

  // now lets try to get back an answer that will give the computer the win
  if (possible_answers_where_comp_wins.length > 0) {
    const index: number = Math.floor(
      Math.random() * possible_answers_where_comp_wins.length
    );
    const comp_word: string = possible_answers_where_comp_wins[index];

    // at this point we already have a word
    if (comp_word == current_word) {
      return [];
    }

    // now grab that letter
    comp_letter = comp_word.charAt(current_word.length) || "";
    return [
      {
        letter: comp_letter,
        author: TileAuthor.Computer,
      },
    ];
  } else {
    // this is the case where we don't have a word that would cause the computer to win
    if (all_possible.length > 0) {
      const index: number = Math.floor(Math.random() * all_possible.length);
      const comp_word: string = all_possible[index];

      if (comp_word == current_word) {
        return [];
      }
      comp_letter = comp_word.charAt(current_word.length) || "";
      return [
        {
          letter: comp_letter,
          author: TileAuthor.Computer,
        },
      ];
    }

    return [];
  }
}

export { get_available_letters, get_tiles_from_computer };
