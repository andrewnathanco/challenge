import { createEffect } from "solid-js";
import { useGame } from "../game/context";

const TOP: string[] = ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"];
const MIDDLE: string[] = ["A", "S", "D", "F", "G", "H", "J", "K", "L"];
const BOTTOM: string[] = ["Z", "X", "C", "V", "B", "N", "M"];

function Key(props: { letter: string }) {
  const [game, set_game] = useGame();
  const letter = props.letter;

  return (
    <button
      classList={{
        "bg-stack-400 border-stack-400 text-sun-50":
          game.available_letters.includes(letter.toLowerCase()),
        "bg-stack-200 border-stack-200 text-sun-50":
          !game.available_letters.includes(letter.toLowerCase()),
      }}
      class="w-8 h-16 rounded-lg cursor-pointer border-2 "
      onclick={() => {
        set_game("selected_letter", props.letter.toLocaleLowerCase());
      }}
    >
      {letter}
    </button>
  );
}

function TopKeys() {
  return (
    <>
      {TOP.map((lett) => (
        <Key letter={lett.toString()} />
      ))}
    </>
  );
}

function MiddleKeys() {
  return (
    <>
      {MIDDLE.map((lett) => (
        <Key letter={lett.toString()} />
      ))}
    </>
  );
}

function BottomKeys() {
  return (
    <>
      {BOTTOM.map((lett) => (
        <Key letter={lett.toString()} />
      ))}
    </>
  );
}
export { BottomKeys, MiddleKeys, TopKeys };
