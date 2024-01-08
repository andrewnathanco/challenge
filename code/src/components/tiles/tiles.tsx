import { For, createEffect } from "solid-js";
import { useGame } from "../game/context";
import { DEFAULT_LETTER } from "../game/model";
import { useSession } from "../session/context";
import { SessionStatus } from "../session/model";

export enum TileAuthor {
  Computer,
  User,
}

// TypeScript interface for the Tile struct
export interface Tile {
  letter: string; // Assuming that the letter is a string, change it to char if needed
  author: TileAuthor;
}

export function invert_tile_author(author: TileAuthor) {
  switch (author) {
    case TileAuthor.Computer:
      return TileAuthor.User;
    case TileAuthor.User:
      return TileAuthor.Computer;
  }
}

enum TileType {
  empty_select_tile,
  not_okay_select_tile,
  okay_select_tile,
  you_tile,
  computer_tile,
  lost_tile,
}

function TileView(props: { letter: string; tile_type: TileType }) {
  let tile_type = props.tile_type;

  const letter = () => {
    const { letter } = props;
    return letter;
  };

  return (
    <li
      classList={{
        "text-stack-400 border-stack-400":
          tile_type == TileType.empty_select_tile,
        "text-stiletto-600 border-stiletto-600":
          tile_type == TileType.not_okay_select_tile,
        "border-mallard-600 text-mallard-600":
          tile_type == TileType.okay_select_tile,
        "bg-mallard-600 border-mallard-600 text-sun-50":
          tile_type == TileType.you_tile,
        "bg-stack-500 border-stack-500 text-sun-50":
          tile_type == TileType.computer_tile,
        "bg-red-300 border-red-500": tile_type == TileType.lost_tile,
      }}
      class="w-16 h-20 border-2 rounded-lg flex justify-center items-center"
    >
      {letter()}
    </li>
  );
}

function tile_to_tile_type(tile: Tile) {
  switch (tile.author) {
    case TileAuthor.Computer:
      return TileType.computer_tile;
    case TileAuthor.User:
      return TileType.you_tile;
  }
}

function Tiles(props: { tiles?: Tile[] }) {
  const [game, _] = useGame();
  const [session, __] = useSession();
  const tiles = props.tiles;

  const get_select_tile = () => {
    if (game.selected_letter == DEFAULT_LETTER) {
      return (
        <TileView
          letter={game.selected_letter}
          tile_type={TileType.empty_select_tile}
        />
      );
    } else if (
      game.available_letters.includes(game.selected_letter.toLowerCase())
    ) {
      return (
        <TileView
          letter={game.selected_letter}
          tile_type={TileType.okay_select_tile}
        />
      );
    } else {
      return (
        <TileView
          letter={game.selected_letter}
          tile_type={TileType.not_okay_select_tile}
        />
      );
    }
  };

  return (
    <div class="flex-1 flex items-center justify-center">
      <ul class="flex flex-wrap gap-y-1 gap-x-1 max-w-screen text-2xl justify-center items-center uppercase">
        <For each={tiles ?? game.current_tiles}>
          {(tile, _) => {
            return (
              <TileView
                letter={tile.letter}
                tile_type={tile_to_tile_type(tile)}
              />
            );
          }}
        </For>
        {session.status == SessionStatus.Current ? get_select_tile() : <></>}
      </ul>
    </div>
  );
}

export { Tiles };
