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
  let [_, { open }] = useGameInfoDialog();
  let [session, set_session] = useSession();
  let [game, set_game] = useGame();

  return (
    <div class="flex flex-col justify-between space-y-2 h-full w-full">
      <Tiles />
      <Keyboard />
      <div class="flex flex-col space-y-2">
        {session.status == SessionStatus.Challenge ? (
          <button
            class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700 flex items-center justify-center space-x-2"
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
              viewBox="0 0 24 24"
              fill="currentColor"
              class="w-6 h-6"
            >
              <path
                fill-rule="evenodd"
                d="M4.755 10.059a7.5 7.5 0 0112.548-3.364l1.903 1.903h-3.183a.75.75 0 100 1.5h4.992a.75.75 0 00.75-.75V4.356a.75.75 0 00-1.5 0v3.18l-1.9-1.9A9 9 0 003.306 9.67a.75.75 0 101.45.388zm15.408 3.352a.75.75 0 00-.919.53 7.5 7.5 0 01-12.548 3.364l-1.902-1.903h3.183a.75.75 0 000-1.5H2.984a.75.75 0 00-.75.75v4.992a.75.75 0 001.5 0v-3.18l1.9 1.9a9 9 0 0015.059-4.035.75.75 0 00-.53-.918z"
                clip-rule="evenodd"
              ></path>
            </svg>
            <div>Try Again</div>
          </button>
        ) : session.status != SessionStatus.Current ? (
          <button
            class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700"
            onclick={() => {
              open();
            }}
          >
            Stats
          </button>
        ) : (
          <button
            class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700"
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
            Challenge
          </button>
        )}
      </div>
    </div>
  );
}

export { SessionView };
