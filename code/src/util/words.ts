import words from "./valid_words.json";

function get_available_letters(curr_word: string): string[] {
  const letters = new Set<string>();
  for (const word of words) {
    if (word.startsWith(curr_word)) {
      const avail_lett = word.toUpperCase().charAt(curr_word.length) || "";
      letters.add(avail_lett);
    }
  }

  return Array.from(letters);
}

export { get_available_letters };
