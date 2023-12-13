import { createEffect, createSignal } from "solid-js";
import { useGame } from "./context";
import { get_game_key, get_todays_game } from "./service";

export function GameHeader() {
  const [game, set_game] = useGame();

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
    <div class="flex space-x-2 justify-center items-center">
      <div class="text-4xl">Challenge #{game.game_key}</div>
      <div
        id="game-version"
        class="font-semibold w-min h-min text-gray-600 text-xs border-2 px-1 border-gray-600 rounded-lg"
      >
        {version()}
      </div>
    </div>
  );
}
