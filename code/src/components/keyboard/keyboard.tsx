import { createEffect, onCleanup } from "solid-js";
import { TopKeys, MiddleKeys, BottomKeys } from "./key";
import { useGame } from "../game/context";
import { DEFAULT_LETTER } from "../game/model";
import { TileAuthor } from "../tiles/tiles";

function Keyboard() {
  const [game, set_game] = useGame();

  const submit_letter = () => {
    if (game.selected_letter != DEFAULT_LETTER) {
      set_game("current_tiles", (tiles) => {
        tiles.push({ author: TileAuthor.User, letter: game.selected_letter });
        return [...tiles];
      });

      set_game("selected_letter", DEFAULT_LETTER);
    }
  };

  const remove_letter = () => {
    if (game.selected_letter !== DEFAULT_LETTER) {
      set_game("selected_letter", DEFAULT_LETTER);
    }

    // This makes sure we don't go past the starting tiles
    const enough_tiles_to_strip =
      game.current_tiles.length > game.starting_tiles.length;

    if (enough_tiles_to_strip) {
      set_game("current_tiles", (curr_tiles) => {
        const tiles = curr_tiles.slice(0, -2);
        return [...tiles];
      });
    }
  };

  const handleKeyboard = (ev: KeyboardEvent) => {
    ev.preventDefault();

    const code = ev.code;
    if (code.startsWith("Key")) {
      const key = code.slice(3);
      set_game("selected_letter", key.charAt(0) || "");
    }

    if (code === "Enter") {
      submit_letter();
    }

    if (code === "Backspace") {
      remove_letter();
    }
  };

  createEffect(() => {
    window.addEventListener("keydown", handleKeyboard);

    onCleanup(() => {
      window.removeEventListener("keydown", handleKeyboard);
    });
  });

  return (
    <div
      id="keyboard"
      class="flex flex-col space-y-1 w-full items-center text-sm text-gray-700"
    >
      <div id="top-row" class="flex flex-row space-between space-x-1">
        <TopKeys />
      </div>
      <div id="middle-row" class="flex flex-row space-between space-x-1">
        <MiddleKeys />
      </div>
      <div id="bottom-row" class="flex flex-row space-between space-x-1">
        <button
          class="p-1 h-16 bg-green-300 rounded-lg cursor-pointer items-center justify-center flex"
          onclick={submit_letter}
        >
          ENTER
        </button>
        <BottomKeys />
        <button
          class="w-12 h-16 bg-red-300 rounded-lg cursor-pointer justify-center items-center flex"
          onclick={remove_letter}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="currentColor"
            class="w-6 h-6"
          >
            <path
              fill-rule="evenodd"
              d="M2.515 10.674a1.875 1.875 0 000 2.652L8.89 19.7c.352.351.829.549 1.326.549H19.5a3 3 0 003-3V6.75a3 3 0 00-3-3h-9.284c-.497 0-.974.198-1.326.55l-6.375 6.374zM12.53 9.22a.75.75 0 10-1.06 1.06L13.19 12l-1.72 1.72a.75.75 0 101.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 101.06-1.06L15.31 12l1.72-1.72a.75.75 0 10-1.06-1.06l-1.72 1.72-1.72-1.72z"
              clip-rule="evenodd"
            ></path>
          </svg>
        </button>
      </div>
    </div>
  );
}

export default Keyboard;
