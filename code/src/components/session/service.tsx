import { Game } from "../game/model";
import { TileAuthor, invert_tile_author } from "../tiles/tiles";
import { Session, SessionStatus } from "./model";

function get_default_session(): Session {
  return {
    status: SessionStatus.Current,
    tiles: [],
  };
}

function get_share(game: Game, session: Session) {
  let new_tiles = session.tiles.map((tile) => {
    return {
      ...tile,
      author: invert_tile_author(
        session.tiles[session.tiles.length - 1].author
      ),
    };
  });

  return `Challenge #${game.game_key}:\n\n${new_tiles
    .map((tile) => (tile.author == TileAuthor.Computer ? "⬜️" : "🟩"))
    .join("")} (${new_tiles.length})\n\nplay at ${
    import.meta.env.VITE_BASE_URL
  }`;
}

export { get_default_session, get_share };
