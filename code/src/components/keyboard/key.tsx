import { useGame } from "../game/context";

const TOP: string[] = ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"];
const MIDDLE: string[] = ["A", "S", "D", "F", "G", "H", "J", "K", "L"];
const BOTTOM: string[] = ["Z", "X", "C", "V", "B", "N", "M"];

function Key(props: { letter: string }) {
  const [_, set_game] = useGame();
  const letter = props.letter;
  return (
    <button
      class="w-8 h-16 bg-gray-300 rounded-lg cursor-pointer border-2 border-gray-400"
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
