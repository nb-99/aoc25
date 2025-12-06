use std::fs;

fn solve_part1(padded_lines: &[String], max_width: usize) -> i64 {
    let mut problems: Vec<(Vec<i64>, char)> = Vec::new();
    let mut col = 0;
    
    while col < max_width {
        let mut is_separator = true;
        for line in padded_lines {
            if col < line.len() {
                let ch = line.chars().nth(col).unwrap_or(' ');
                if ch != ' ' {
                    is_separator = false;
                    break;
                }
            }
        }
        
        if is_separator {
            col += 1;
            continue;
        }
        
        let start_col = col;
        while col < max_width {
            let mut all_space = true;
            for line in padded_lines {
                if col < line.len() {
                    let ch = line.chars().nth(col).unwrap_or(' ');
                    if ch != ' ' {
                        all_space = false;
                        break;
                    }
                }
            }
            if all_space {
                break;
            }
            col += 1;
        }
        let end_col = col;
        
        let mut numbers: Vec<i64> = Vec::new();
        let mut operator = '+';
        
        for (i, line) in padded_lines.iter().enumerate() {
            let slice: String = line.chars().skip(start_col).take(end_col - start_col).collect();
            let trimmed = slice.trim();
            
            if i == padded_lines.len() - 1 {
                if trimmed == "*" {
                    operator = '*';
                } else if trimmed == "+" {
                    operator = '+';
                }
            } else if !trimmed.is_empty() {
                if let Ok(num) = trimmed.parse::<i64>() {
                    numbers.push(num);
                }
            }
        }
        
        if !numbers.is_empty() {
            problems.push((numbers, operator));
        }
    }
    
    problems.iter().map(|(numbers, operator)| {
        if *operator == '*' {
            numbers.iter().product::<i64>()
        } else {
            numbers.iter().sum::<i64>()
        }
    }).sum()
}

fn solve_part2(padded_lines: &[String], max_width: usize) -> i64 {
    // Part 2: Each column is a number (digits stacked vertically, top=most significant)
    // Problems are processed right-to-left
    let chars: Vec<Vec<char>> = padded_lines.iter()
        .map(|l| l.chars().collect())
        .collect();
    
    let num_rows = chars.len();
    let mut problems: Vec<(Vec<i64>, char)> = Vec::new();
    let mut col = 0;
    
    while col < max_width {
        // Skip separator columns
        let mut is_separator = true;
        for row in &chars {
            if col < row.len() && row[col] != ' ' {
                is_separator = false;
                break;
            }
        }
        
        if is_separator {
            col += 1;
            continue;
        }
        
        // Found start of problem, find end
        let start_col = col;
        while col < max_width {
            let mut all_space = true;
            for row in &chars {
                if col < row.len() && row[col] != ' ' {
                    all_space = false;
                    break;
                }
            }
            if all_space {
                break;
            }
            col += 1;
        }
        let end_col = col;
        
        // Extract numbers column by column, each column forms one number
        let mut numbers: Vec<i64> = Vec::new();
        let mut operator = '+';
        
        for c in start_col..end_col {
            let mut digits = String::new();
            for r in 0..num_rows - 1 {
                if c < chars[r].len() {
                    let ch = chars[r][c];
                    if ch.is_ascii_digit() {
                        digits.push(ch);
                    }
                }
            }
            // Get operator from last row
            if c < chars[num_rows - 1].len() {
                let op_char = chars[num_rows - 1][c];
                if op_char == '*' {
                    operator = '*';
                } else if op_char == '+' {
                    operator = '+';
                }
            }
            
            if !digits.is_empty() {
                if let Ok(num) = digits.parse::<i64>() {
                    numbers.push(num);
                }
            }
        }
        
        if !numbers.is_empty() {
            problems.push((numbers, operator));
        }
    }
    
    // Process problems right-to-left (reverse order)
    problems.iter().map(|(numbers, operator)| {
        if *operator == '*' {
            numbers.iter().product::<i64>()
        } else {
            numbers.iter().sum::<i64>()
        }
    }).sum()
}

fn main() {
    let input = fs::read_to_string("input.csv").expect("Failed to read input file");
    let lines: Vec<&str> = input.lines().collect();
    
    if lines.is_empty() {
        println!("Empty input");
        return;
    }
    
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padded_lines: Vec<String> = lines.iter()
        .map(|l| format!("{:width$}", l, width = max_width))
        .collect();
    
    let part1 = solve_part1(&padded_lines, max_width);
    println!("Part 1 - Grand total: {}", part1);
    
    let part2 = solve_part2(&padded_lines, max_width);
    println!("Part 2 - Grand total: {}", part2);
}
