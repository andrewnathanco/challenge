import { makeBroadcastChannel } from "@solid-primitives/broadcast-channel";
import { Meta } from "@solidjs/meta";
import { createEffect, createSignal } from "solid-js";

export default function Multi() {
  const { onMessage, postMessage } =
    makeBroadcastChannel<string>("multiplayer");
  const [message, setMessage] = createSignal("");
  const [toSend, setToSend] = createSignal("");
  const [game, setGame] = createSignal<string[]>([]);

  const syncItUp = () => {
    setGame([...game(), toSend()]);
    postMessage(toSend());
  };

  createEffect(() => {
    onMessage((ev) => {
      setMessage(ev.data);
    });
  });

  return (
    <div class="p-4">
      <Meta
        name="viewport"
        content="width=device-width, initial-scale=1, maximum-scale=1, minimum-scale=1, user-scalable=0"
      />
      <div>Message: {message()}</div>
      <div class="flex flex-col space-y-2">
        <input
          class="bg-stone-300 rounded-lg text-sun-80 p-2"
          onchange={(e) => {
            setToSend(e.target.value);
          }}
        />
        <button
          class="p-2 rounded-lg bg-stone-600 text-sun-50"
          onclick={() => {
            syncItUp();
          }}
        >
          send message
        </button>
      </div>
    </div>
  );
}
