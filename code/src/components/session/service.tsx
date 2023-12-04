import { Session, SessionStatus } from "./model";

export function get_default_session(): Session {
  return {
    status: SessionStatus.Current,
    tiles: [],
  };
}
