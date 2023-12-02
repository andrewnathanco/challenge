package main

import (
	"encoding/csv"
	"encoding/json"
	"fmt"
	"log"
	"log/slog"
	"math/rand"
	"os"
	"slices"
	"strings"
	"time"

	"github.com/samber/lo"
)

func readCsvFile(filePath string) ([][]string, error) {
	f, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer f.Close()

	csvReader := csv.NewReader(f)
	records, err := csvReader.ReadAll()
	if err != nil {
		log.Fatal("Unable to parse file as CSV for "+filePath, err)
		return nil, err
	}

	// get rid of any comments (really just for the wiki)
	filtered := lo.Filter[[]string](records, func(item []string, index int) bool {
		if len(item) != 1 {
			return false
		}

		// filter out all 2 letter words
		return len(item[0]) > 2
	})

	return filtered, nil
}

func exportToJSON(data interface{}, filename string) {
	file, err := os.Create(filename)
	if err != nil {
		must(err)
	}
	defer file.Close()

	encoder := json.NewEncoder(file)

	// Encode data and write to file
	err = encoder.Encode(data)
	if err != nil {
		must(err)
	}
}

func exportToCSV(data []string, filename string) {
	file, err := os.Create(filename)
	if err != nil {
		must(err)
	}
	defer file.Close()

	writer := csv.NewWriter(file)
	defer writer.Flush()

	// Write each item in the data slice on a new line
	for _, item := range data {
		err := writer.Write([]string{item})
		if err != nil {
			must(err)
		}
	}
}

func must(err error) {
	if err != nil {
		slog.Error(err.Error())
		panic(1)
	}
}

func doesWordContainAnotherWord(word string, word_by_num [][]string) bool {
	if len(word) <= 1 {
		return false
	}

	contains_word := lo.Contains[string](word_by_num[len(word)-1], word)
	if contains_word {
		return true
	}

	return doesWordContainAnotherWord(word[0:len(word)-1], word_by_num)
}

func getWordsFromFile(file_name string) []string {
	records, err := readCsvFile(file_name)
	must(err)

	// map the current records into just a list of words, basically flatten the csv structure
	return lo.Map[[]string, string](records, func(item []string, index int) string {
		if len(item) != 1 {
			must(err)
		}

		return item[0]
	})
}

func elapsed(name string) func() {
	start := time.Now()
	return func() {
		fmt.Printf("%s took %v\n", name, time.Since(start))
	}
}

func getWords() ([][]string, []string) {
	all_words := getWordsFromFile("./data/scrabble.csv")
	medium_common := getWordsFromFile("./data/wiki-1m-formatted.csv")
	blacklist := getWordsFromFile("./data/blacklist.csv")

	all_words = lo.Intersect[string](all_words, medium_common)
	all_words, _ = lo.Difference[string](all_words, blacklist)

	all_words = lo.Filter(all_words, func(item string, index int) bool {
		return len(item) > 3
	})

	slices.SortFunc(all_words, func(a, b string) int {
		return len(b) - len(a)
	})

	max_length := len(all_words[0])
	word_by_num := make([][]string, max_length)
	for _, word := range all_words {
		if len(word) > 3 {
			words_in_this_bucket := word_by_num[len(word)-1]
			word_by_num[len(word)-1] = append(words_in_this_bucket, word)
		}
	}

	return word_by_num, all_words
}

func getAnswerSet() []string {
	word_buckets, all_words := getWords()
	defer elapsed("calc_answer_set")()

	answer_set := map[string]struct{}{}
	for _, word := range all_words {
		sub_word := word[0 : len(word)-1]
		if !doesWordContainAnotherWord(sub_word, word_buckets) {
			answer_set[word] = struct{}{}
		}
	}

	answer_list := []string{}
	for word := range answer_set {
		answer_list = append(answer_list, word)
	}

	return answer_list
}

func main() {
	answers := getAnswerSet()
	exportToJSON(answers, "./data/valid_words.json")
	exportToCSV(answers, "./data/valid_words.csv")

	game_active := true
	curr_word := ""
	comp_word := ""
	comp_letter := "me"
	curr_word += comp_letter

	// game loop
	fmt.Printf("The first letter is: %s\n", comp_letter)
	var computer_next_letter byte
	for game_active {
		fmt.Printf("Enter your letter: ")
		var user_letter string
		fmt.Scan(&user_letter)

		if user_letter == "CHALLENGE" {
			fmt.Printf("You lose, computer was thinking of: %s", comp_word)
			break
		}

		// assume its not valid to start
		valid_letter := false
		for !valid_letter {
			for _, answer := range answers {
				if strings.HasPrefix(answer, curr_word+user_letter) {
					valid_letter = true
					break
				}
			}

			if !valid_letter {
				fmt.Printf("That letter won't form a word eventually try again: ")
				fmt.Scanln(&user_letter)
			}
		}

		if user_letter == "CHALLENGE" {
			fmt.Printf("You lose, computer was thinking of: %s\n", comp_word)
			break
		}
		curr_word += user_letter

		if lo.Contains[string](answers, curr_word) {
			game_active = false
			fmt.Printf("The word is now: %s\n", curr_word)
			fmt.Println("you lose")
			break
		}

		available_answers := []string{}
		for _, answer := range answers {
			word_can_be_used := strings.HasPrefix(answer, curr_word)
			is_long_enough := len(answer)-len(curr_word) > 2
			will_let_computer_win := len(answer)%2 == 0

			if word_can_be_used && is_long_enough && will_let_computer_win {
				available_answers = append(available_answers, answer)
			}
		}

		if len(available_answers) == 0 {
			// if we can't find that, we need to just grab the next word and lose
			for _, answer := range answers {
				if strings.HasPrefix(answer, curr_word) {
					computer_next_letter = answer[len(curr_word)]
					comp_word = answer
				}
			}
		} else {
			random_answer := rand.Intn(len(available_answers))
			comp_word = available_answers[random_answer]
			computer_next_letter = comp_word[len(curr_word)]
		}

		fmt.Printf("The computer's letter is: %s\n", string(computer_next_letter))
		curr_word += string(computer_next_letter)
		fmt.Printf("The word is now: %s\n", curr_word)

		if lo.Contains[string](answers, curr_word) {
			game_active = false
			fmt.Printf("Computer was thinking of: %s\n", comp_word)
			fmt.Println("computer loses")
		}
	}
}
