import { SetStoreFunction, Store } from "solid-js/store";
import { Tile } from "../tiles/tiles";

interface Session {}

export const DEFAULT_LETTER = "_";
export interface Game {
  game_key: number;
  starting_word: string;
  starting_tiles: Tile[];
  starting_letters: string;

  selected_letter: string;
  available_letters: string[];
  current_tiles: Tile[];

  sessions: Session[];
}

export type GameStore = [Store<Game>, SetStoreFunction<Game>];
