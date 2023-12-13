import { makePersisted } from "@solid-primitives/storage";
import { createContext, useContext } from "solid-js";
import { SetStoreFunction, createStore } from "solid-js/store";
import { Session } from "./model";
import { get_default_session } from "./service";

const SessionContext = createContext<[Session, SetStoreFunction<Session>]>([
  {} as Session,
  () => {},
]);

export function SessionProvider(props: any) {
  let value = makePersisted(createStore(get_default_session()), {
    name: "challenge_session",
  });

  return (
    <SessionContext.Provider value={value}>
      {props.children}
    </SessionContext.Provider>
  );
}

export function useSession() {
  return useContext(SessionContext);
}
