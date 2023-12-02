import { For, createEffect, createSignal } from "solid-js";
import { useGame } from "../game/context";

// TypeScript interfaces for the enums
enum TileKind {
  TileEmpty,
  TileYou,
  TileComp,
  TileOkay,
  TileNotOkay,
}

export enum TileAuthor {
  Computer,
  User,
}

// TypeScript interface for the Tile struct
export interface Tile {
  letter: string; // Assuming that the letter is a string, change it to char if needed
  author: TileAuthor;
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
        "text-gray-300 border-gray-300":
          tile_type == TileType.empty_select_tile,
        "text-red-600 border-red-600":
          tile_type == TileType.not_okay_select_tile,
        "border-green-600 text-green-600":
          tile_type == TileType.okay_select_tile,
        "bg-green-300 border-green-500 ": tile_type == TileType.you_tile,
        "bg-gray-300 border-gray-400": tile_type == TileType.computer_tile,
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

function Tiles(props: { tiles: Tile[] }) {
  const tiles = props.tiles;
  const [game, set_game] = useGame();
  const [selected_letter, set_selected_letter] = createSignal(
    game().selected_letter
  );

  createEffect(() => {
    console.log(game().selected_letter);
  });

  return (
    <div class="flex-1 flex items-center justify-center">
      <ul class="flex flex-wrap gap-y-1 gap-x-1 max-w-screen text-2xl justify-center items-center uppercase">
        <For each={tiles}>
          {(tile, index) => {
            return (
              <TileView
                letter={tile.letter}
                tile_type={tile_to_tile_type(tile)}
              />
            );
          }}
        </For>
        <TileView
          letter={selected_letter()}
          tile_type={TileType.empty_select_tile}
        />
      </ul>
    </div>
  );
}

export { Tiles };
