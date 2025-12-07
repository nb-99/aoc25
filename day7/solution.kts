// Advent of Code 2024 - Day 7: Laboratories
// Kotlin script solution

import java.io.File

fun solve(input: String): Int {
    val lines = input.trim().lines()
    val grid = lines.map { it.toCharArray() }
    val height = grid.size
    val width = grid[0].size
    
    // Find starting position S
    var startCol = -1
    for (col in 0 until width) {
        if (grid[0][col] == 'S') {
            startCol = col
            break
        }
    }
    
    // Track active beam columns as a set (beams merge when at same position)
    var activeBeams = mutableSetOf(startCol)
    var totalSplits = 0
    
    // Process each row starting from row 1 (below S)
    for (row in 1 until height) {
        if (activeBeams.isEmpty()) break
        
        val newBeams = mutableSetOf<Int>()
        
        for (col in activeBeams) {
            when (grid[row][col]) {
                '^' -> {
                    // Beam hits a splitter - count it and create left/right beams
                    totalSplits++
                    // Left beam continues from col-1, right beam from col+1
                    if (col - 1 >= 0) newBeams.add(col - 1)
                    if (col + 1 < width) newBeams.add(col + 1)
                }
                '.', 'S' -> {
                    // Beam continues downward at same column
                    newBeams.add(col)
                }
            }
        }
        
        activeBeams = newBeams
    }
    
    return totalSplits
}

// Read input and solve
val input = File("input.csv").readText()
val result = solve(input)
println("Total splits: $result")
