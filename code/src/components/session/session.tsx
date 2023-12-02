import { useContext } from "solid-js";
import Keyboard from "../keyboard/keyboard";
import { Tile, TileAuthor, Tiles } from "../tiles/tiles";
import { useGameInfoDialog } from "../game_info_dialog/context";
import { useGame } from "../game/context";

function SessionView() {
  let [game, __] = useGame();
  let [_, { open }] = useGameInfoDialog();
  return (
    <div class="flex flex-col justify-between space-y-2 h-full w-full">
      <Tiles tiles={game().current_tiles} />
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
