export async function get_current_number_played() {
  const db_url = import.meta.env.VITE_DB_URL;
  const response = await fetch(`${db_url}/GET/challenge_games-played`);
  const games_played_res = await response.json();
  const num_played = parseInt(games_played_res["GET"]);
  return Number.isNaN(num_played) ? undefined : num_played;
}

export async function update_number_of_games_played() {
  const db_url = import.meta.env.VITE_DB_URL;
  const num_played = await get_current_number_played();
  if (!num_played) {
    await fetch(`${db_url}/SET/challenge_games-played/1`);
  } else {
    await fetch(`${db_url}/SET/challenge_games-played/${num_played + 1}`);
  }
}
