import {
  Accessor,
  Signal,
  createContext,
  createEffect,
  createSignal,
  useContext,
} from "solid-js";
import { Game } from "./model";
import { get_todays_game } from "./service";
import { createStoredSignal } from "../../util/storage";

const GameContext = createContext<Signal<Game>>();

export function GameProvider(props: { game: Game; children: any }) {
  const stored_game = createSignal(props.game);

  return (
    <GameContext.Provider value={stored_game}>
      {props.children}
    </GameContext.Provider>
  );
}

export function useGame(): Signal<Game> {
  return useContext(GameContext) as Signal<Game>;
}
