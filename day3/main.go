package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func readInput(path string) string {
  content, err := os.ReadFile(path)

  if err != nil {
    fmt.Println("Unable to read file")
    return ""
  }

  return string(content)
}

func makeCollisionKey(firstNum, secondNum int) string {
  return fmt.Sprintf("%d-%d", firstNum, secondNum)
}

func make2DArray(inputs string) ([][]string, map[string]bool) {
  lineByLine := strings.Split(inputs, "\n")
  output := make([][]string, len(lineByLine))
  symbolsCollisions := make(map[string]bool)

  for rowIdx, row := range lineByLine {
    lineItems := strings.Split(row, "")
    output[rowIdx] = make([]string, len(row))

    for colIdx, col := range lineItems {
      output[rowIdx][colIdx] = col 

      _, err := strconv.Atoi(col)

      if err != nil && col != "." {
        topCollision := makeCollisionKey(rowIdx - 1, colIdx) 
        bottomCollision := makeCollisionKey(rowIdx + 1, colIdx)

        topLeft := makeCollisionKey(rowIdx - 1, colIdx - 1)
        topRight := makeCollisionKey(rowIdx - 1, colIdx + 1)

        bottomLeft := makeCollisionKey(rowIdx + 1, colIdx - 1)
        bottomRight := makeCollisionKey(rowIdx + 1, colIdx + 1)

        left := makeCollisionKey(rowIdx, colIdx - 1)
        right := makeCollisionKey(rowIdx, colIdx + 1)

        symbolsCollisions[right] = true
        symbolsCollisions[left] = true
        symbolsCollisions[topCollision] = true
        symbolsCollisions[bottomCollision] = true
        symbolsCollisions[topLeft] = true
        symbolsCollisions[topRight] = true
        symbolsCollisions[bottomLeft] = true
        symbolsCollisions[bottomRight] = true
      }
    }
  }

  return output, symbolsCollisions;
}

func hasCollision(
  potentialCollisions []string,
  symbolsCollisions map[string]bool,
) bool {

  for _, collisionVal := range potentialCollisions {
    if symbolsCollisions[collisionVal] == true {
      return true
    }
  }

  return false
}

func main() {
  inputContent := readInput("./input")
  schematic, symbolsCollisions := make2DArray(inputContent)

  var runningSum int
  for index, value := range schematic {
    var currentNum string 
    var currentIndexes []string
    for lineItemIdx, lineItemVal := range value {
      _, err := strconv.Atoi(lineItemVal)

      if err != nil {
        if currentNum != "" {
          if hasCollision(currentIndexes, symbolsCollisions) == true {
            num, _ := strconv.Atoi(currentNum)
            fmt.Println(num)
            runningSum += num
          }
        }

        currentIndexes = nil
        currentNum = ""
        continue
      }

      currentCollision := makeCollisionKey(index, lineItemIdx)
      currentIndexes = append(currentIndexes, currentCollision)
      currentNum = currentNum + lineItemVal

      if lineItemIdx == len(value) - 1 && currentNum != "" {
        if hasCollision(currentIndexes, symbolsCollisions) == true {
          num, _ := strconv.Atoi(currentNum)
          fmt.Println(num)
          runningSum += num
        }
      }
    }
  }

  fmt.Printf("\n----------------\noutput %d\n----------------\n", runningSum)
}
