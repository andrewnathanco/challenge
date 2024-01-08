import { createEffect, createSignal } from "solid-js";
import { useGame } from "./context";
import { get_game_key, get_todays_game } from "./service";
import { get_current_number_played } from "../../util/service";
import { useInfoDialog } from "../info_dialog/context";

export function GameInfo() {
  const [game, set_game] = useGame();
  const [__, { open }] = useInfoDialog();
  const [version, _] = createSignal<string>(
    import.meta.env.VITE_VERSION ?? "v0.1.0"
  );

  createEffect(() => {
    if (get_game_key() != game.game_key) {
      localStorage.removeItem("challenge_game");
      set_game(get_todays_game());
    }
  });

  return (
    <div class="flex space-between items-center">
      <div class="flex flex-col space-y-2 justify-center flex-1">
        <div class="text-4xl">Challenge #{game.game_key}</div>
        <div
          id="game-version"
          class="font-semibold w-min h-min text-stack-700 text-xs border-2 px-1 border-stack-700 rounded-lg"
        >
          {version()}
        </div>
      </div>
      <button
        class="flex items-center space-x-2 justify-center rounded-lg p-2 bg-stone-700 text-sun-50 w-fit h-fit"
        onclick={() => {
          open();
        }}
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="w-6 h-6"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="m11.25 11.25.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z"
          />
        </svg>
      </button>
    </div>
  );
}
