import { Tile } from "../tiles/tiles";

export enum SessionStatus {
  Current,
  UserWon,
  ComputerWon,
}

export interface Session {
  status: SessionStatus;
  tiles: Tile[];
}

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
