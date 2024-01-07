import { createEffect, createSignal } from "solid-js";
import { useGame } from "./context";
import { get_game_key, get_todays_game } from "./service";
import { get_current_number_played } from "../../util/service";

export function GameInfo() {
  const [game, set_game] = useGame();
  const [number_played, set_number_played] = createSignal<number | undefined>(
    undefined
  );

  const [version, _] = createSignal<string>(
    import.meta.env.VITE_VERSION ?? "v0.1.0"
  );

  createEffect(() => {
    if (get_game_key() != game.game_key) {
      localStorage.removeItem("challenge_game");
      set_game(get_todays_game());
    }
  });

  createEffect(() => {
    get_current_number_played().then((x) => {
      set_number_played(x);
    });
  });

  return (
    <div class="flex flex-col space-y-2">
      <div class="flex space-x-2 justify-center items-center">
        <div class="text-4xl">Challenge #{game.game_key}</div>
        <div
          id="game-version"
          class="font-semibold w-min h-min text-stack-700 text-xs border-2 px-1 border-stack-700 rounded-lg"
        >
          {version()}
        </div>
      </div>
      {number_played() ? (
        <div class="flex flex-col space-y-1 items-start">
          <div id="games-played" class="text-contessa-500 text-4xl">
            {number_played()}
          </div>
          <div class="text-md">Have Played</div>
        </div>
      ) : (
        <></>
      )}
    </div>
  );
}
