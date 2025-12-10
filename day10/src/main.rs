use rayon::prelude::*;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use z3::ast::{Ast, Int};
use z3::{Config, Context, Optimize, SatResult};

fn main() {
    let input = fs::read_to_string("input.csv").expect("Failed to read input");
    
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    let total_lines = lines.len();
    
    eprintln!("Processing {} machines in parallel...", total_lines);
    
    // Atomic counter for progress
    let progress = AtomicUsize::new(0);
    
    // Process all lines in parallel
    let results: Vec<(u64, u64)> = lines
        .par_iter()
        .map(|line| {
            let (lights, buttons, joltages) = parse_line(line);
            
            // Part 1: Toggle problem (GF(2) linear algebra)
            let p1 = solve_part1(&lights, &buttons);
            
            // Part 2: Counter increment problem (ILP)
            let p2 = solve_part2(&buttons, &joltages);
            
            // Progress update
            let done = progress.fetch_add(1, Ordering::Relaxed) + 1;
            eprintln!("[{:3}/{}] p1={}, p2={}", done, total_lines, p1, p2);
            
            (p1, p2)
        })
        .collect();
    
    // Sum up results
    let part1_total: u64 = results.iter().map(|(p1, _)| p1).sum();
    let part2_total: u64 = results.iter().map(|(_, p2)| p2).sum();
    
    println!("Part 1: {}", part1_total);
    println!("Part 2: {}", part2_total);
}

fn parse_line(line: &str) -> (Vec<bool>, Vec<Vec<usize>>, Vec<u64>) {
    let mut lights = Vec::new();
    let mut buttons = Vec::new();
    let mut joltages = Vec::new();
    
    // Parse [lights]
    let bracket_start = line.find('[').unwrap();
    let bracket_end = line.find(']').unwrap();
    let light_str = &line[bracket_start + 1..bracket_end];
    for c in light_str.chars() {
        lights.push(c == '#');
    }
    
    // Parse (button) groups and {joltages}
    let rest = &line[bracket_end + 1..];
    
    // Find all parentheses groups
    let mut i = 0;
    let chars: Vec<char> = rest.chars().collect();
    while i < chars.len() {
        if chars[i] == '(' {
            let start = i + 1;
            while i < chars.len() && chars[i] != ')' {
                i += 1;
            }
            let button_str: String = chars[start..i].iter().collect();
            let indices: Vec<usize> = button_str
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse().unwrap())
                .collect();
            buttons.push(indices);
        } else if chars[i] == '{' {
            let start = i + 1;
            while i < chars.len() && chars[i] != '}' {
                i += 1;
            }
            let jolt_str: String = chars[start..i].iter().collect();
            joltages = jolt_str
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse().unwrap())
                .collect();
        }
        i += 1;
    }
    
    (lights, buttons, joltages)
}

// Part 1: Solve XOR/toggle system - find minimum button presses
fn solve_part1(target: &[bool], buttons: &[Vec<usize>]) -> u64 {
    let n_lights = target.len();
    let n_buttons = buttons.len();
    
    if n_buttons == 0 {
        return if target.iter().all(|&x| !x) { 0 } else { u64::MAX };
    }
    
    // Build augmented matrix over GF(2): [A | b]
    // Each row is a light, each column (except last) is a button
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; n_buttons + 1]; n_lights];
    
    for (btn_idx, btn) in buttons.iter().enumerate() {
        for &light_idx in btn {
            if light_idx < n_lights {
                matrix[light_idx][btn_idx] = 1;
            }
        }
    }
    
    // Target vector (b)
    for (i, &t) in target.iter().enumerate() {
        matrix[i][n_buttons] = if t { 1 } else { 0 };
    }
    
    // Gaussian elimination over GF(2)
    let mut pivot_cols = Vec::new();
    let mut row = 0;
    
    for col in 0..n_buttons {
        // Find pivot
        let mut pivot_row = None;
        for r in row..n_lights {
            if matrix[r][col] == 1 {
                pivot_row = Some(r);
                break;
            }
        }
        
        if let Some(pr) = pivot_row {
            matrix.swap(row, pr);
            pivot_cols.push(col);
            
            // Eliminate
            for r in 0..n_lights {
                if r != row && matrix[r][col] == 1 {
                    for c in 0..=n_buttons {
                        matrix[r][c] ^= matrix[row][c];
                    }
                }
            }
            row += 1;
        }
    }
    
    // Check for inconsistency (row with all zeros in A but 1 in b)
    for r in row..n_lights {
        if matrix[r][n_buttons] == 1 {
            return u64::MAX; // No solution
        }
    }
    
    // Free variables
    let free_vars: Vec<usize> = (0..n_buttons)
        .filter(|c| !pivot_cols.contains(c))
        .collect();
    
    let n_free = free_vars.len();
    
    // Enumerate all 2^n_free combinations to find minimum weight solution
    let mut min_presses = u64::MAX;
    
    for mask in 0..(1u64 << n_free) {
        let mut solution = vec![0u8; n_buttons];
        
        // Set free variables
        for (i, &fv) in free_vars.iter().enumerate() {
            solution[fv] = ((mask >> i) & 1) as u8;
        }
        
        // Back-substitute to find pivot variables
        for (r, &pc) in pivot_cols.iter().enumerate().rev() {
            let mut val = matrix[r][n_buttons];
            for c in (pc + 1)..n_buttons {
                val ^= matrix[r][c] * solution[c];
            }
            solution[pc] = val;
        }
        
        let presses: u64 = solution.iter().map(|&x| x as u64).sum();
        min_presses = min_presses.min(presses);
    }
    
    min_presses
}

// Part 2: Solve counter increment problem using Z3 ILP optimizer
fn solve_part2(buttons: &[Vec<usize>], target: &[u64]) -> u64 {
    let n_counters = target.len();
    let n_buttons = buttons.len();
    
    if n_buttons == 0 {
        return if target.iter().all(|&x| x == 0) { 0 } else { u64::MAX };
    }
    
    // Create Z3 context and optimizer
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);
    
    // Create integer variables for each button press count
    let btn_vars: Vec<Int> = (0..n_buttons)
        .map(|i| Int::new_const(&ctx, format!("btn_{}", i)))
        .collect();
    
    let zero = Int::from_i64(&ctx, 0);
    
    // Constraint: all button presses >= 0
    for btn in &btn_vars {
        opt.assert(&btn.ge(&zero));
    }
    
    // Constraint: for each counter, sum of affecting button presses = target
    for (counter_idx, &target_val) in target.iter().enumerate() {
        let target_z3 = Int::from_u64(&ctx, target_val);
        
        // Sum of button presses that affect this counter
        let affecting_sum: Int = buttons
            .iter()
            .enumerate()
            .filter(|(_, btn)| btn.contains(&counter_idx))
            .map(|(btn_idx, _)| btn_vars[btn_idx].clone())
            .fold(Int::from_i64(&ctx, 0), |acc, x| Int::add(&ctx, &[&acc, &x]));
        
        opt.assert(&affecting_sum._eq(&target_z3));
    }
    
    // Objective: minimize total button presses
    let total_presses = Int::add(&ctx, &btn_vars.iter().collect::<Vec<_>>());
    opt.minimize(&total_presses);
    
    // Solve
    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().unwrap();
            let result: u64 = btn_vars
                .iter()
                .map(|v| model.eval(v, true).unwrap().as_u64().unwrap())
                .sum();
            result
        }
        _ => u64::MAX,
    }
}

