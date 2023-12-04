import { createEffect, createSignal } from "solid-js";
import { useGame } from "../game/context";
import { get_countdown_till_next_game, get_todays_game } from "../game/service";
import { useGameInfoDialog } from "./context";
import { get_default_session } from "../session/service";
import { useSession } from "../session/context";
import { SessionStatus } from "../session/model";
import { Tiles, invert_tile_author } from "../tiles/tiles";

export function GameInfoDialog() {
  const [game, set_game] = useGame();
  const [session, set_session] = useSession();

  const [is_open, { open, close }] = useGameInfoDialog();
  const [countdown, set_countdown] = createSignal(
    get_countdown_till_next_game()
  );

  createEffect(() => {
    setInterval(() => {
      set_countdown(get_countdown_till_next_game());
    }, 1000);
  });

  return (
    <div classList={{ hidden: !is_open(), block: is_open() }}>
      <div class="z-10 absolute top-0 left-0 right-0 bottom-0 justify-center items-center bg-black flex opacity-70"></div>
      <div class="z-20 absolute top-0 left-0 right-0 bottom-0 bg-white flex m-4 rounded-lg">
        <div id="dialog-content" class="p-8 flex flex-col space-y-2 w-full">
          <div
            id="dialog-header"
            class="flex justify-between items-center text-3xl w-full"
          >
            <div>
              {session.status == SessionStatus.UserWon
                ? "You Won"
                : session.status == SessionStatus.ComputerWon
                ? "Computer Won"
                : ""}
            </div>
            <button
              onClick={() => {
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
                  d="M5.47 5.47a.75.75 0 011.06 0L12 10.94l5.47-5.47a.75.75 0 111.06 1.06L13.06 12l5.47 5.47a.75.75 0 11-1.06 1.06L12 13.06l-5.47 5.47a.75.75 0 01-1.06-1.06L10.94 12 5.47 6.53a.75.75 0 010-1.06z"
                  clip-rule="evenodd"
                ></path>
              </svg>
            </button>
          </div>
          <div class="flex-1 flex flex-col justify-center items-center">
            {session.status != SessionStatus.Current ? (
              <Tiles
                tiles={game.current_tiles.map((tile) => {
                  return {
                    ...tile,
                    author: invert_tile_author(
                      game.current_tiles[game.current_tiles.length - 1].author
                    ),
                  };
                })}
              />
            ) : (
              <></>
            )}
          </div>
          <div class="flex flex-col space-y-2">
            <div id="timer" class="flex flex-col space-y-1 items-center">
              <div id="time" class="text-3xl">
                {countdown()}
              </div>
              <div id="text" class="text-2xl">
                Next Challenge
              </div>
            </div>
            <button
              class="border-2 border-gray-300 rounded-lg w-full p-2 text-gray-700 bg-gray-300 flex items-center justify-center space-x-2"
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
            <button
              onclick={() => {
                try {
                  navigator.share({
                    url: "mural.andrewnathan.net",
                  });
                } catch {
                  navigator.clipboard.writeText("sharing");
                }
              }}
              class="border-2 border-green-400 rounded-lg w-full p-2 text-gray-700 bg-green-400 flex items-center justify-center space-x-2"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="currentColor"
                class="w-6 h-6"
              >
                <path
                  fill-rule="evenodd"
                  d="M15.75 4.5a3 3 0 11.825 2.066l-8.421 4.679a3.002 3.002 0 010 1.51l8.421 4.679a3 3 0 11-.729 1.31l-8.421-4.678a3 3 0 110-4.132l8.421-4.679a3 3 0 01-.096-.755z"
                  clip-rule="evenodd"
                ></path>
              </svg>

              <div>Share</div>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
