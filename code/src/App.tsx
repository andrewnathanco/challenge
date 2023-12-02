import {
  createEffect,
  type Component,
  createSignal,
  useContext,
  createContext,
} from "solid-js";
import Keyboard from "./components/keyboard/keyboard";
import { GameHeader } from "./components/game/view";
import words from "./util/valid_words.json";
import { GameInfoDialog } from "./components/game_info_dialog/dialog";
import { Tiles } from "./components/tiles/tiles";
import { SessionView } from "./components/session/session";
import { GameInfoDialogProvider } from "./components/game_info_dialog/context";
import { GameProvider } from "./components/game/context";
import { get_game_key, get_todays_game } from "./components/game/service";

const App: Component = () => {
  // dialog context
  let game_info_dialog = createSignal(false);
  createContext(game_info_dialog, { name: "info_dialog" });

  // game context
  createContext(get_todays_game(), { name: "game" });

  return (
    <GameProvider game={get_todays_game()}>
      <GameInfoDialogProvider>
        <div class="w-full flex h-full flex-col justify-center items-center">
          <div class="flex text-xl justify-between flex-col p-4 space-y-4 h-full text-gray-700 w-96">
            <div id="game-info" class="flex space-y-2 flex-col items-center">
              <GameHeader />
              <div class="text-base text-center">
                Alternating with the computer, try to make the longest word one
                letter at a time. You lose if you are the one to finish the
                word.
              </div>
            </div>
            <SessionView />
            <GameInfoDialog />
          </div>
        </div>
      </GameInfoDialogProvider>
    </GameProvider>
  );
};

export default App;
