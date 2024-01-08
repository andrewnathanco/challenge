import {
  type Component,
  createSignal,
  createContext,
  createEffect,
} from "solid-js";
import { Meta } from "@solidjs/meta";
import { ShareInfoDialog } from "../components/share/dialog";

const App: Component = () => {
  return (
    <>
      <Meta
        name="viewport"
        content="width=device-width, initial-scale=1, maximum-scale=1, minimum-scale=1, user-scalable=0"
      />
      <ShareInfoDialog />
    </>
  );
};

export default App;
