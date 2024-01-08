import { SessionStatus } from "../session/model";
import { useSearchParams, useParams } from "@solidjs/router";
import { get_default_session } from "../session/service";
import { createEffect } from "solid-js";
import { TileAuthor, Tiles } from "../tiles/tiles";
import words from "../../util/valid_words.json";

export function ShareInfoDialog() {
  const [params, _] = useSearchParams();
  const session_status = parseInt(params.status ?? "1") as SessionStatus;
  const word_index = parseInt(params.word ?? "0");
  const word = word_index <= words.length ? words[word_index] : words[0];
  const user_name = params.name;

  return (
    <div class="block">
      <div class="z-10 absolute top-0 left-0 right-0 bottom-0 justify-center items-center bg-black flex opacity-70"></div>
      <div class="z-20 absolute top-0 left-0 right-0 bottom-0 bg-sun-50 flex m-4 rounded-lg">
        <div id="dialog-content" class="p-8 flex flex-col space-y-2 w-full">
          <div id="dialog-header" class="flex flex-col text-3xl space-y-2">
            <div class="text-xl italic">
              <a href={import.meta.env.SERVER_BASE_URL} class="underline">
                play today's game
              </a>
            </div>
            <div>
              {session_status == SessionStatus.UserWon
                ? `${user_name ?? "They"} won`
                : session_status == SessionStatus.ComputerWon
                ? `${user_name ?? "They"} lost`
                : ""}
            </div>
          </div>
          <div class="flex-1 flex flex-col justify-center items-center">
            <Tiles
              tiles={word.split("").map((lett) => {
                return {
                  letter: lett,
                  author:
                    session_status == SessionStatus.UserWon
                      ? TileAuthor.User
                      : TileAuthor.Computer,
                };
              })}
            />
          </div>
          <div class="flex flex-col space-y-2">
            <a
              href={`https://www.merriam-webster.com/dictionary/${word}`}
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
          </div>
        </div>
      </div>
    </div>
  );
}
