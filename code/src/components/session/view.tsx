import { useGame } from "../game/context";
import { useGameInfoDialog } from "../game_info_dialog/context";
import Keyboard from "../keyboard/keyboard";
import { Tiles } from "../tiles/tiles";
import { useSession } from "./context";
import { SessionStatus } from "./model";

function SessionView() {
  let [_, { open }] = useGameInfoDialog();
  let [session, __] = useSession();

  return (
    <div class="flex flex-col justify-between space-y-2 h-full w-full">
      <Tiles />
      <Keyboard />
      {session.status != SessionStatus.Current ? (
        <div class="flex flex-col space-y-2">
          <button
            class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700"
            onclick={() => {
              open();
            }}
          >
            Stats
          </button>
        </div>
      ) : (
        <div class="flex flex-col space-y-2">
          <button
            class="border-2 border-gray-500 rounded-lg w-full p-2 text-gray-700"
            onclick={() => {
              open();
            }}
          >
            Challenge
          </button>
        </div>
      )}
    </div>
  );
}

export { SessionView };
