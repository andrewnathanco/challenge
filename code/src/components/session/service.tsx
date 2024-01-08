import { Game } from "../game/model";
import { TileAuthor, invert_tile_author } from "../tiles/tiles";
import { Session, SessionStatus } from "./model";
import words from "../../util/valid_words.json";

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

  const share_url = `${
    import.meta.env.VITE_BASE_URL
  }/share?word=${words.indexOf(
    game.current_tiles.map((tile) => tile.letter.toLowerCase()).join("")
  )}&status=${session.status}`;

  return [
    `Challenge #${game.game_key}\nScore: ${
      session.status == SessionStatus.UserWon ? game.current_tiles.length : "❎"
    }`,
    share_url,
  ];
}

export { get_default_session, get_share };
