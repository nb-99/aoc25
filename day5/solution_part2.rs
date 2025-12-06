use std::fs;

fn main() {
    let content = fs::read_to_string("input.csv").expect("Failed to read input file");
    let parts: Vec<&str> = content.split("\n\n").collect();
    
    // Parse ranges
    let mut ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<u64> = line.split('-').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();
    
    // Sort by start
    ranges.sort_by_key(|r| r.0);
    
    // Merge overlapping ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }
    
    // Count total unique IDs
    let total: u64 = merged.iter().map(|(s, e)| e - s + 1).sum();
    
    println!("Total fresh ingredient IDs: {}", total);
}
