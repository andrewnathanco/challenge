import { makePersisted } from "@solid-primitives/storage";
import { createEffect, createSignal } from "solid-js";

interface Game {
  game_key: number;
}

const GameHeader = () => {
  const [game, setGame] = makePersisted(createSignal<Game>({ game_key: 1 }));

  const [version, setVersion] = createSignal<string>("v.0.1.0");

  return (
    <div class="flex space-x-2 justify-center items-center">
      <div class="text-4xl">Challenge #{game().game_key}</div>
      <div
        id="game-version"
        class="font-semibold w-min h-min text-gray-600 text-xs border-2 px-1 border-gray-600 rounded-lg"
      >
        {version()}
      </div>
    </div>
  );
};

export { GameHeader };
