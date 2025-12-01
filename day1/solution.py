#!/usr/bin/env python3
"""
Advent of Code 2025 - Day 1: Secret Entrance
"""

def count_zeros_crossed(position: int, direction: str, distance: int) -> int:
    """
    Count how many times the dial hits 0 during a rotation.
    This counts every click that lands on 0, including the final position.
    """
    if direction == 'R':
        # Right = toward higher numbers (increasing)
        # We cross 0 when wrapping from 99 to 0
        # Count = floor((position + distance) / 100)
        return (position + distance) // 100
    else:  # L
        # Left = toward lower numbers (decreasing)
        # We hit 0 when position - k ≡ 0 (mod 100), i.e., k = position, position+100, ...
        # Count how many of these k values are in range [1, distance]
        if position == 0:
            # First hit at k=100, then every 100
            return distance // 100
        elif position <= distance:
            # First hit at k=position, then every 100 after
            return (distance - position) // 100 + 1
        else:
            return 0


def solve_part1(input_file: str) -> int:
    """Part 1: count zeros only at end of rotations."""
    position = 50
    zero_count = 0
    
    with open(input_file, 'r') as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            
            direction = line[0]
            distance = int(line[1:])
            
            if direction == 'L':
                position = (position - distance) % 100
            else:
                position = (position + distance) % 100
            
            if position == 0:
                zero_count += 1
    
    return zero_count


def solve_part2(input_file: str) -> int:
    """Part 2: count zeros at every click during all rotations."""
    position = 50
    zero_count = 0
    
    with open(input_file, 'r') as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            
            direction = line[0]
            distance = int(line[1:])
            
            # Count all zeros crossed during this rotation
            zero_count += count_zeros_crossed(position, direction, distance)
            
            # Update position
            if direction == 'L':
                position = (position - distance) % 100
            else:
                position = (position + distance) % 100
    
    return zero_count


if __name__ == "__main__":
    # Test Part 2 with the example
    example_rotations = ["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"]
    position = 50
    total_zeros = 0
    
    print("Part 2 Example walkthrough:")
    print(f"Start at: {position}")
    
    for rotation in example_rotations:
        direction = rotation[0]
        distance = int(rotation[1:])
        
        zeros = count_zeros_crossed(position, direction, distance)
        total_zeros += zeros
        
        if direction == 'L':
            new_pos = (position - distance) % 100
        else:
            new_pos = (position + distance) % 100
        
        print(f"{rotation}: {position} -> {new_pos}, zeros crossed: {zeros}")
        position = new_pos
    
    print(f"\nExample answer: {total_zeros} (expected: 6)")
    
    # Verify R1000 from 50
    test_zeros = count_zeros_crossed(50, 'R', 1000)
    print(f"\nR1000 from 50 crosses 0: {test_zeros} times (expected: 10)")
    
    # Solve the actual puzzles
    print(f"\n=== Part 1 Answer: {solve_part1('input.csv')} ===")
    print(f"=== Part 2 Answer: {solve_part2('input.csv')} ===")
