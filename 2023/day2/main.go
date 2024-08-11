package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type game struct {
	red   int
	blue  int
	green int
}

func main() {
	gameMap := read_file("input.txt")
	result := game1(12, 13, 14, gameMap)
	fmt.Println(result)

	result = game2(12, 13, 14, gameMap)
	fmt.Println(result)
}

func game1(r int, g int, b int, gameMap map[int][]game) int {
	gameIdSum := 0

	for gameId := 1; gameId <= len(gameMap); gameId++ {
		flag := true
		for _, game := range gameMap[gameId] {
			if game.red > r || game.blue > b || game.green > g {
				flag = false
				break
			}
		}
		if flag {
			gameIdSum += gameId
		}
	}
	return gameIdSum
}

func game2(r int, g int, b int, gameMap map[int][]game) int {
	gameIdSum := 0

	for gameId := 1; gameId <= len(gameMap); gameId++ {
		maxBall := game{
			red:   0,
			blue:  0,
			green: 0,
		}
		for _, game := range gameMap[gameId] {
			maxBall.red = max(maxBall.red, game.red)
			maxBall.blue = max(maxBall.blue, game.blue)
			maxBall.green = max(maxBall.green, game.green)
		}
		gameIdSum += maxBall.red * maxBall.blue * maxBall.green
	}
	return gameIdSum
}

func read_file(filename string) map[int][]game {
	file, err := os.Open(filename)
	if err != nil {
		log.Fatalf("failed to open file: %s", err)
	}
	defer file.Close() // Ensure the file is closed when the function exits

	// Create a new scanner
	scanner := bufio.NewScanner(file)

	// Read the filPe line by line
	gameMap := make(map[int][]game)
	for scanner.Scan() {
		line := scanner.Text() // Get the current line
		gameSplit := strings.Split(line, ":")

		gameId, err := strconv.Atoi(strings.Split(gameSplit[0], " ")[1])
		if err != nil {
			log.Fatalf("error reading gameId: %s", err)
		}
		revealedCubes := strings.Split(gameSplit[len(gameSplit)-1], ";")

		arr := make([]game, 0)
		for _, cubes := range revealedCubes {
			cubeSplit := strings.Split(cubes, ",")
			singleGame := game{
				red:   0,
				blue:  0,
				green: 0,
			}
			for _, cube := range cubeSplit {
				cubeTrim := strings.TrimSpace(cube)
				splitted := strings.Split(cubeTrim, " ")

				color := splitted[1]
				num, err := strconv.Atoi(splitted[0])
				if err != nil {
					log.Fatalf("error reading cube number: %s", err)
				}
				// fmt.Printf("color: %s, num:%d\n", color, num)
				switch color {
				case "red":
					singleGame.red += num
				case "blue":
					singleGame.blue += num
				case "green":
					singleGame.green += num
				default:
					log.Fatalf("unknown color: %s", color)
				}
			}
			arr = append(arr, singleGame)
		}
		gameMap[gameId] = arr
	}

	// Check for errors during the scan
	if err := scanner.Err(); err != nil {
		log.Fatalf("error reading file: %s", err)
	}
	return gameMap
}
