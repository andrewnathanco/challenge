import { Signal, createContext, createSignal, useContext } from "solid-js";
import { Game } from "./model";
import { get_todays_game } from "./service";
import { createStoredSignal } from "../../util/storage";
import { makePersisted } from "@solid-primitives/storage";
import { SetStoreFunction, createStore } from "solid-js/store";

const GameContext = createContext<[Game, SetStoreFunction<Game>]>([
  {} as Game,
  () => {},
]);

export function GameProvider(props: any) {
  let value = makePersisted(createStore(get_todays_game()), {
    name: "game",
  });

  return (
    <GameContext.Provider value={value}>{props.children}</GameContext.Provider>
  );
}

export function useGame() {
  return useContext(GameContext);
}
