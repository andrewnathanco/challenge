import { createEffect } from "solid-js";
import { get_computer_word } from "../../util/words";
import { useGame } from "../game/context";
import { get_todays_game } from "../game/service";
import { useGameInfoDialog } from "../game_info_dialog/context";
import Keyboard from "../keyboard/keyboard";
import { Tile, TileAuthor, Tiles } from "../tiles/tiles";
import { useSession } from "./context";
import { SessionStatus } from "./model";
import { get_default_session } from "./service";

function SessionView() {
  let [_, { open, close }] = useGameInfoDialog();
  let [session, set_session] = useSession();
  let [game, set_game] = useGame();

  return (
    <div class="flex flex-col justify-between space-y-2 h-full w-full">
      <Tiles />
      <Keyboard />
      <div class="w-full flex space-x-1">
        <button
          class="flex items-center space-x-2 justify-center rounded-lg p-2 bg-stone-700 text-sun-50 w-full"
          onclick={() => {
            set_game({
              ...get_todays_game(),
              sessions: [...game.sessions],
            });
            set_session(get_default_session());
            close();
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
              d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
            />
          </svg>
          <div>Reset</div>
        </button>
        {session.status != SessionStatus.Current ? (
          <a
            href={`https://www.merriam-webster.com/dictionary/${game.current_tiles
              .map((tile) => tile.letter.toLowerCase())
              .join("")}`}
            target="_blank"
            rel="noopener noreferrer"
            class="border-2 border-sahara-800 rounded-lg w-full p-2 text-sun-50 bg-sahara-800 flex items-center justify-center space-x-2"
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
                d="M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.967 8.967 0 0 0-6 2.292m0-14.25v14.25"
              />
            </svg>

            <div>Define</div>
          </a>
        ) : (
          <button
            class="flex items-center space-x-2 justify-center rounded-lg p-2 bg-stiletto-600 text-sun-50 w-full"
            onclick={() => {
              let comp_word = get_computer_word(game.current_tiles);
              let new_tiles = comp_word.split("").map((lett) => {
                return { letter: lett, author: TileAuthor.Computer } as Tile;
              });
              let new_session = {
                status: SessionStatus.Challenge,
                tiles: new_tiles,
              };

              set_session(new_session);

              // update game with new tiles
              set_game("current_tiles", (_) => [...new_tiles]);
              set_game("sessions", [...game.sessions, new_session]);
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
                d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z"
              />
            </svg>

            <div>Challenge</div>
          </button>
        )}
      </div>
      {session.status != SessionStatus.Current &&
      session.status != SessionStatus.Challenge ? (
        <button
          onclick={() => {
            open();
          }}
          class="border-2 border-mallard-700 rounded-lg w-full p-2 text-sun-50 bg-mallard-700 flex items-center justify-center space-x-2"
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
              d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0 0 20.25 18V6A2.25 2.25 0 0 0 18 3.75H6A2.25 2.25 0 0 0 3.75 6v12A2.25 2.25 0 0 0 6 20.25Z"
            />
          </svg>

          <div>Stats</div>
        </button>
      ) : (
        <></>
      )}
    </div>
  );
}

export { SessionView };
