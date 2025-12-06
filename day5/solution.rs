use std::fs;

fn main() {
    let content = fs::read_to_string("input.csv").expect("Failed to read input file");
    let parts: Vec<&str> = content.split("\n\n").collect();
    
    // Parse ranges
    let ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<u64> = line.split('-').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();
    
    // Parse ingredient IDs and count fresh ones
    let fresh_count = parts[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|&id| ranges.iter().any(|&(start, end)| id >= start && id <= end))
        .count();
    
    println!("Fresh ingredient IDs: {}", fresh_count);
}
