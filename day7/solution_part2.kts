// Advent of Code 2024 - Day 7: Laboratories - Part 2
// Kotlin script solution - Many-worlds interpretation (count timelines)

import java.io.File

fun solve(input: String): Long {
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
    
    // Track number of timelines at each column position
    // Key = column, Value = number of timelines at that column
    var timelines = mutableMapOf(startCol to 1L)
    
    // Process each row starting from row 1 (below S)
    for (row in 1 until height) {
        if (timelines.isEmpty()) break
        
        val newTimelines = mutableMapOf<Int, Long>()
        
        for ((col, count) in timelines) {
            when (grid[row][col]) {
                '^' -> {
                    // Splitter: each timeline splits into two
                    // Left path
                    if (col - 1 >= 0) {
                        newTimelines[col - 1] = newTimelines.getOrDefault(col - 1, 0L) + count
                    }
                    // Right path
                    if (col + 1 < width) {
                        newTimelines[col + 1] = newTimelines.getOrDefault(col + 1, 0L) + count
                    }
                }
                '.', 'S' -> {
                    // Empty space: timelines continue downward
                    newTimelines[col] = newTimelines.getOrDefault(col, 0L) + count
                }
            }
        }
        
        timelines = newTimelines
    }
    
    // Sum all timelines that made it through
    return timelines.values.sum()
}

// Read input and solve
val input = File("input.csv").readText()
val result = solve(input)
println("Total timelines: $result")
