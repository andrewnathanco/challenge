package main

import (
	"bufio"
	"encoding/csv"
	"fmt"
	"os"
	"strings"
)

func main() {
	// Specify the file paths
	inputFilePath := "./wiki-10k-unformatted.txt"
	outputFilePath := "./wiki-10k-formatted.csv"

	// Open the input file
	inputFile, err := os.Open(inputFilePath)
	if err != nil {
		fmt.Println("Error opening input file:", err)
		return
	}
	defer inputFile.Close()

	// Create a scanner to read the input file line by line
	scanner := bufio.NewScanner(inputFile)

	// Create a slice to store the extracted words
	var words []string

	// Iterate through each line in the input file
	for scanner.Scan() {
		line := scanner.Text()

		// Split the line by tabs
		parts := strings.Split(line, "\t")

		// Check if there are at least three parts
		if len(parts) >= 3 {
			// Extract the word in the middle (index 1)
			word := parts[1]

			// Add the extracted word to the slice
			words = append(words, word)
		}
	}

	// Check for scanner errors
	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading input file:", err)
		return
	}

	// Open the output file for writing
	outputFile, err := os.Create(outputFilePath)
	if err != nil {
		fmt.Println("Error creating output file:", err)
		return
	}
	defer outputFile.Close()

	// Create a CSV writer
	csvWriter := csv.NewWriter(outputFile)

	// Write each word to a new line in the CSV file
	for _, word := range words {
		csvWriter.Write([]string{word})
	}

	// Flush and close the CSV writer
	csvWriter.Flush()

	// Check for CSV writer errors
	if err := csvWriter.Error(); err != nil {
		fmt.Println("Error writing to CSV file:", err)
		return
	}

	fmt.Println("Words exported to", outputFilePath)
}
