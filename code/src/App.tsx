import {
  createEffect,
  type Component,
  createSignal,
  useContext,
  createContext,
} from "solid-js";
import { GameHeader } from "./components/game/view";
import { GameInfoDialog } from "./components/game_info_dialog/dialog";
import { SessionView } from "./components/session/view";
import { GameInfoDialogProvider } from "./components/game_info_dialog/context";
import { makePersisted } from "@solid-primitives/storage";
import { createMutable, createStore } from "solid-js/store";
import { get_todays_game } from "./components/game/service";
import { GameProvider } from "./components/game/context";
import { SessionProvider } from "./components/session/context";

const App: Component = () => {
  // dialog context
  let game_info_dialog = createSignal(false);
  createContext(game_info_dialog, { name: "info_dialog" });

  return (
    <GameProvider>
      <SessionProvider>
        <GameInfoDialogProvider>
          <div class="w-full flex h-full flex-col justify-center items-center">
            <div class="flex text-xl justify-between flex-col p-4 space-y-4 h-full text-gray-700 w-96">
              <div id="game-info" class="flex space-y-2 flex-col items-center">
                <GameHeader />
                <div class="text-base text-center">
                  Alternating with the computer, try to make the longest word
                  one letter at a time. You lose if you are the one to finish
                  the word. (3 letter words don't count) Click challenge to see
                  what word the computer is thinking of.
                </div>
              </div>
              <SessionView />
              <GameInfoDialog />
            </div>
          </div>
        </GameInfoDialogProvider>
      </SessionProvider>
    </GameProvider>
  );
};

export default App;
