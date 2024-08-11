package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"unicode"
)

func main() {
	gameMap := read_file("input.txt")
	result := game1(gameMap)
	fmt.Println(result)
}

func game1(gameMap map[int]string) int {
	height := len(gameMap)
	width := len(gameMap[0])
	ans := 0
	for y := range height {
		for x := 0; x < width; x++ {
			if unicode.IsDigit(rune(gameMap[y][x])) {
				num := 0
				flag := false
				for x < height && unicode.IsDigit(rune(gameMap[y][x])) {
					if is_valid(gameMap, x, y, width, height) {
						flag = true
					}
					num = num*10 + int(gameMap[y][x]-'0')
					x += 1
				}
				if flag {
					ans += num
				}
				// fmt.Printf("ans: %d\n", ans)

			}

		}
	}
	return ans
}

func is_valid(gameMap map[int]string, x int, y int, width int, height int) bool {
	directions := [][]int{
		{-1, 0},
		{0, 1},
		{1, 0},
		{0, -1},
		{-1, 1},
		{1, 1},
		{1, -1},
		{-1, -1},
	}
	for _, direction := range directions {
		nx := x + direction[0]
		ny := y + direction[1]
		if nx < 0 || nx >= width || ny < 0 || ny >= height {
			continue
		}
		if unicode.IsDigit(rune(gameMap[ny][nx])) {
			continue
		} else if gameMap[ny][nx] == '.' {
			continue
		} else {
			return true
		}
	}
	return false
}

func read_file(filename string) map[int]string {
	file, err := os.Open(filename)
	if err != nil {
		log.Fatalf("failed to open file: %s", err)
	}
	defer file.Close() // Ensure the file is closed when the function exits

	gameMap := make(map[int]string)
	// Create a new scanner
	scanner := bufio.NewScanner(file)

	// Read the filPe line by line
	index := 0
	for scanner.Scan() {
		line := scanner.Text()
		gameMap[index] = line
		index++
	}
	return gameMap
}
