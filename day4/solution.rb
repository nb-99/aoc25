#!/usr/bin/env ruby

# Advent of Code Day 4
# Count paper rolls (@) accessible by forklift (fewer than 4 adjacent rolls)

grid = File.readlines('input.csv', chomp: true).map(&:chars)
rows = grid.length
cols = grid[0].length

# 8 directions: up, down, left, right, and 4 diagonals
DIRS = [[-1, -1], [-1, 0], [-1, 1],
        [0, -1],           [0, 1],
        [1, -1],  [1, 0],  [1, 1]]

def count_adjacent_rolls(grid, row, col, rows, cols)
  DIRS.count do |dr, dc|
    nr, nc = row + dr, col + dc
    nr.between?(0, rows - 1) && nc.between?(0, cols - 1) && grid[nr][nc] == '@'
  end
end

def find_accessible(grid, rows, cols)
  accessible = []
  (0...rows).each do |r|
    (0...cols).each do |c|
      next unless grid[r][c] == '@'
      accessible << [r, c] if count_adjacent_rolls(grid, r, c, rows, cols) < 4
    end
  end
  accessible
end

# Part 1: count accessible in initial state
puts "Part 1: #{find_accessible(grid, rows, cols).length}"

# Part 2: repeatedly remove accessible rolls until none left
grid2 = File.readlines('input.csv', chomp: true).map(&:chars)
total_removed = 0

loop do
  accessible = find_accessible(grid2, rows, cols)
  break if accessible.empty?
  
  total_removed += accessible.length
  accessible.each { |r, c| grid2[r][c] = '.' }
end

puts "Part 2: #{total_removed}"
