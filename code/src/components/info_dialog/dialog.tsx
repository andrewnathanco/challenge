import { createEffect, createSignal } from "solid-js";
import { useGame } from "../game/context";
import { get_countdown_till_next_game, get_todays_game } from "../game/service";
import { get_default_session, get_share } from "../session/service";
import { useSession } from "../session/context";
import { SessionStatus } from "../session/model";
import { Tiles, invert_tile_author } from "../tiles/tiles";
import { useInfoDialog } from "./context";

export function InfoDialog() {
  const [is_open, { open, close }] = useInfoDialog();
  const [countdown, set_countdown] = createSignal(
    get_countdown_till_next_game()
  );

  createEffect(() => {
    setInterval(() => {
      set_countdown(get_countdown_till_next_game());
    }, 1000);
  });

  return (
    <div
      classList={{
        hidden: !is_open.dialog_status,
        block: is_open.dialog_status,
      }}
    >
      <div class="z-10 absolute top-0 left-0 right-0 bottom-0 justify-center items-center bg-black flex opacity-70"></div>
      <div class="z-20 absolute top-0 left-0 right-0 bottom-0 bg-sun-50 flex m-4 rounded-lg">
        <div id="dialog-content" class="p-8 flex flex-col space-y-2 w-full">
          <div
            id="dialog-header"
            class="flex justify-between items-center text-3xl w-full"
          >
            <div>Challenge</div>
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
          <div class="flex flex-col space-y-2">
            <div class="flex flex-col">
              <div class="text-xl">What is Challenge?</div>
              <div class="text-md">
                Challenge is a daily puzzle game where you have to compete
                against the computer to spell a word. Puzzle refreshes daily at
                12:00AM EST.
              </div>
            </div>
            <div class="flex flex-col">
              <div class="text-xl">How to play?</div>
              <div class="text-md">
                Think of a word to spell and enter the next letter. The computer
                will think of a word and return with a letter. The computer wins
                if you enter a letter that completes a word, and you win if you
                get the computer to finish a word. Try to make the longest word
                possible.
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
