import { createEffect, useContext } from "solid-js";
import Keyboard from "../keyboard/keyboard";
import { Tile, TileAuthor, Tiles } from "../tiles/tiles";
import { useGameInfoDialog } from "../game_info_dialog/context";
import { SetStoreFunction, Store } from "solid-js/store";
import { Game, GameStore } from "../game/model";
import { useGame } from "../game/context";

function SessionView() {
  let [_, { open }] = useGameInfoDialog();
  let [game, set_game] = useGame();

  return (
    <div class="flex flex-col justify-between space-y-2 h-full w-full">
      <Tiles />
      <Keyboard />
      <div class="flex flex-col space-y-2">
        <button
          class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700"
          onclick={() => {
            open();
          }}
        >
          Stats
        </button>
      </div>
    </div>
  );
}

export { SessionView };
