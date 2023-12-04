import { Tile } from "../tiles/tiles";

export enum SessionStatus {
  Current,
  Challenge,
  UserWon,
  ComputerWon,
}

export interface Session {
  status: SessionStatus;
  tiles: Tile[];
}
