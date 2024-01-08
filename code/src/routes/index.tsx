import {
  type Component,
  createSignal,
  createContext,
  createEffect,
} from "solid-js";
import { GameInfo } from "../components/game/view";
import { SessionView } from "../components/session/view";
import { GameProvider, useGame } from "../components/game/context";
import { SessionProvider, useSession } from "../components/session/context";
import { Meta } from "@solidjs/meta";
import { get_game_key, get_todays_game } from "../components/game/service";
import { get_current_number_played } from "../util/service";
import { get_default_session } from "../components/session/service";
import { GameInfoDialogProvider } from "../components/game_info_dialog/context";
import { GameInfoDialog } from "../components/game_info_dialog/dialog";
import { InfoDialogProvider } from "../components/info_dialog/context";
import { InfoDialog } from "../components/info_dialog/dialog";

const App: Component = () => {
  // dialog context
  let game_info_dialog = createSignal(false);

  let [game, set_game] = useGame();
  let [_, set_session] = useSession();

  createContext(game_info_dialog, { name: "info_dialog" });

  createEffect(() => {
    if (game.game_key != get_game_key()) {
      set_game({
        ...get_todays_game(),
      });
      set_session(get_default_session());
    }

    set_game("game_key", get_game_key());
  });

  createEffect(() => {});

  return (
    <>
      <Meta
        name="viewport"
        content="width=device-width, initial-scale=1, maximum-scale=1, minimum-scale=1, user-scalable=0"
      />
      <InfoDialogProvider>
        <GameProvider>
          <SessionProvider>
            <GameInfoDialogProvider>
              <div class="w-full flex h-full flex-col justify-center items-center">
                <div class="flex text-lg justify-between flex-col p-4 space-y-4 h-full text-stack-700 w-96">
                  <GameInfo />
                  <SessionView />
                  <GameInfoDialog />
                  <InfoDialog />
                </div>
              </div>
            </GameInfoDialogProvider>
          </SessionProvider>
        </GameProvider>
      </InfoDialogProvider>
    </>
  );
};

export default App;
