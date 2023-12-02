const TOP: string[] = ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"];
const MIDDLE: string[] = ["A", "S", "D", "F", "G", "H", "J", "K", "L"];
const BOTTOM: string[] = ["Z", "X", "C", "V", "B", "N", "M"];

const Key = (props: { letter: any }) => {
  const letter = props.letter;

  return (
    <button class="w-8 h-16 bg-gray-300 rounded-lg cursor-pointer">
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
