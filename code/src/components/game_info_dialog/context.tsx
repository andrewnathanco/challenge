import { Accessor, createContext, createSignal, useContext } from "solid-js";

type GameInfoDialog = [
  Accessor<boolean>,
  {
    close: () => void;
    open: () => void;
  }
];

const GameInfoDialogContext = createContext<GameInfoDialog>();

export function GameInfoDialogProvider(props: { children: any }) {
  const [dialog_status, set_dialog_status] = createSignal(false);
  const dialog: GameInfoDialog = [
    dialog_status,
    {
      close() {
        set_dialog_status(false);
      },
      open() {
        set_dialog_status(true);
      },
    },
  ];

  return (
    <GameInfoDialogContext.Provider value={dialog}>
      {props.children}
    </GameInfoDialogContext.Provider>
  );
}

export function useGameInfoDialog(): GameInfoDialog {
  return useContext(GameInfoDialogContext) as GameInfoDialog;
}
