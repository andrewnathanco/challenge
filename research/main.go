package main

import (
	"encoding/csv"
	"fmt"
	"log"
	"log/slog"
	"os"
	"slices"
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
	lo.Filter[[]string](records, func(item []string, index int) bool {
		if len(item) != 1 {
			return false
		}

		return item[0][0] != '#'
	})

	return records, nil
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

func getAllWords(file_name string) []string {
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

func main() {
	all_words := getAllWords("./data/scrabble.csv")
	common_words := getAllWords("./data/wiki-1m-formatted.csv")
	all_words = lo.Intersect[string](all_words, common_words)

	slices.SortFunc(all_words, func(a, b string) int {
		return len(b) - len(a)
	})

	// this is intense, but we need to get the largest word length to create our bucket
	max_length := len(all_words[0])
	word_by_num := make([][]string, max_length)
	for _, word := range all_words {
		words_in_this_bucket := word_by_num[len(word)-1]

		word_by_num[len(word)-1] = append(words_in_this_bucket, word)
	}

	defer elapsed("calc_answer_set")()
	answer_set := map[string]struct{}{}
	for _, word := range all_words {
		sub_word := word[0 : len(word)-1]
		if !doesWordContainAnotherWord(sub_word, word_by_num) {
			answer_set[word] = struct{}{}
		}
	}

	fmt.Println(len(answer_set))
}
