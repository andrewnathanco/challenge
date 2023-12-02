import { makePersisted } from "@solid-primitives/storage";
import { createEffect, createSignal } from "solid-js";
import { createStoredSignal } from "../../util/storage";
import words from "../../util/valid_words.json";
import { get_available_letters } from "../../util/words";
import { Tile, TileAuthor } from "../tiles/tiles";
import { DEFAULT_LETTER, Game } from "./model";
import { get_game_key, get_todays_game } from "./service";
import { useGame } from "./context";

export function GameHeader() {
  const [game, set_game] = useGame();
  const [version, _] = createSignal<string>(
    import.meta.env.VITE_VERSION ?? "v0.1.0"
  );

  // update the game key if the keys don't match
  createEffect(() => {
    if (get_game_key() != game().game_key) {
      set_game(get_todays_game());
    }
  });

  return (
    <div class="flex space-x-2 justify-center items-center">
      <div class="text-4xl">Challenge #{game().game_key}</div>
      <div
        id="game-version"
        class="font-semibold w-min h-min text-gray-600 text-xs border-2 px-1 border-gray-600 rounded-lg"
      >
        {version()}
      </div>
    </div>
  );
}
