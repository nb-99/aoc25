import Foundation

// Read input
let inputPath = CommandLine.arguments.count > 1 ? CommandLine.arguments[1] : "input.csv"
let contents = try! String(contentsOfFile: inputPath, encoding: .utf8)

// Parse coordinates
let points = contents.trimmingCharacters(in: .whitespacesAndNewlines)
    .split(separator: "\n")
    .map { line -> (Int, Int) in
        let parts = line.split(separator: ",")
        return (Int(parts[0])!, Int(parts[1])!)
    }

// Find max rectangle area
// For any two points as opposite corners: area = (|x2-x1|+1) * (|y2-y1|+1) counting tiles
var maxArea = 0

for i in 0..<points.count {
    for j in (i+1)..<points.count {
        let (x1, y1) = points[i]
        let (x2, y2) = points[j]
        let area = (abs(x2 - x1) + 1) * (abs(y2 - y1) + 1)
        maxArea = max(maxArea, area)
    }
}

print("Answer: \(maxArea)")
