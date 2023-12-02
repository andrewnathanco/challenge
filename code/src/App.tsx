import type { Component } from "solid-js";

import logo from "./logo.svg";
import styles from "./App.module.css";
import Keyboard from "./components/keyboard/keyboard";
import { GameHeader } from "./components/game/game";

const App: Component = () => {
  return (
    <div class="w-full flex h-full flex-col justify-center items-center">
      <div class="flex text-xl justify-between flex-col p-4 space-y-4 h-full text-gray-700 w-96">
        <div id="game-info" class="flex space-y-2 flex-col items-center h-full">
          <GameHeader />
          <div class="text-base text-center">
            Alternating with the computer, try to make the longest word one
            letter at a time. You lose if you are the one to finish the word.
          </div>
        </div>
        <div class="flex-1"></div>
        <Keyboard />
      </div>
    </div>
  );
};

export default App;
