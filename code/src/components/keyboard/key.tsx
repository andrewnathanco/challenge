import { createEffect } from "solid-js";
import { useGame } from "../game/context";
import { get_todays_game } from "../game/service";

const TOP: string[] = ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"];
const MIDDLE: string[] = ["A", "S", "D", "F", "G", "H", "J", "K", "L"];
const BOTTOM: string[] = ["Z", "X", "C", "V", "B", "N", "M"];

const Key = (props: { letter: any }) => {
  const letter = props.letter;
  const [game, set_game] = useGame();

  return (
    <button
      class="w-8 h-16 bg-gray-300 rounded-lg cursor-pointer border-2 border-gray-400"
      onclick={() => {
        set_game((g) => {
          g.selected_letter = letter;
          return g;
        });
      }}
    >
      {letter}
    </button>
  );
};

const TopKeys = () => {
  return (
    <>
      {TOP.map((lett) => (
        <Key letter={lett.toString()} />
      ))}
    </>
  );
};

const MiddleKeys = () => {
  return (
    <>
      {MIDDLE.map((lett) => (
        <Key letter={lett.toString()} />
      ))}
    </>
  );
};

const BottomKeys = () => {
  return (
    <>
      {BOTTOM.map((lett) => (
        <Key letter={lett.toString()} />
      ))}
    </>
  );
};
export { TopKeys, BottomKeys, MiddleKeys };
