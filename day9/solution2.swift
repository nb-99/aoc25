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

// Build polygon edges (consecutive points, wrapping around)
// Each edge is either horizontal or vertical
struct Edge {
    let x1, y1, x2, y2: Int
    var isVertical: Bool { x1 == x2 }
    var isHorizontal: Bool { y1 == y2 }
    var minY: Int { min(y1, y2) }
    var maxY: Int { max(y1, y2) }
    var minX: Int { min(x1, x2) }
    var maxX: Int { max(x1, x2) }
}

var edges: [Edge] = []
for i in 0..<points.count {
    let p1 = points[i]
    let p2 = points[(i + 1) % points.count]
    edges.append(Edge(x1: p1.0, y1: p1.1, x2: p2.0, y2: p2.1))
}

// For a rectilinear polygon, we can check if a rectangle is fully contained
// by checking if all four corners are inside/on boundary and no edge crosses out

// Get all unique Y coordinates from polygon vertices (these define horizontal strips)
let allYs = Set(points.map { $0.1 }).sorted()

// Build vertical edges sorted by x
let verticalEdges = edges.filter { $0.isVertical }.sorted { $0.x1 < $1.x1 }
let horizontalEdges = edges.filter { $0.isHorizontal }

// For interior testing, use a y slightly offset from vertex positions
func getInsideXRanges(atY y: Int) -> [(Int, Int)] {
    // Use ray casting: count crossings of vertical edges
    // Only count edges that strictly cross y (not just touch)
    var crossings: [Int] = []
    for edge in verticalEdges {
        // Edge strictly crosses y if y is strictly between minY and maxY
        if edge.minY < y && y < edge.maxY {
            crossings.append(edge.x1)
        }
    }
    crossings.sort()
    
    var ranges: [(Int, Int)] = []
    for i in stride(from: 0, to: crossings.count - 1, by: 2) {
        ranges.append((crossings[i], crossings[i + 1]))
    }
    return ranges
}

// Check if point (x, y) is inside or on boundary of polygon
func isPointInsideOrOnBoundary(x: Int, y: Int) -> Bool {
    // Check if on any edge
    for edge in edges {
        if edge.isHorizontal && edge.y1 == y && edge.minX <= x && x <= edge.maxX {
            return true
        }
        if edge.isVertical && edge.x1 == x && edge.minY <= y && y <= edge.maxY {
            return true
        }
    }
    // Check interior using ray casting (count crossings to the left)
    var crossings = 0
    for edge in verticalEdges {
        if edge.minY < y && y < edge.maxY && edge.x1 < x {
            crossings += 1
        }
    }
    return crossings % 2 == 1
}

// Function to check if horizontal segment [x1, x2] at y is inside polygon
func isHorizontalSegmentInside(x1: Int, x2: Int, y: Int) -> Bool {
    let minX = min(x1, x2)
    let maxX = max(x1, x2)
    
    // Check both endpoints
    if !isPointInsideOrOnBoundary(x: minX, y: y) { return false }
    if !isPointInsideOrOnBoundary(x: maxX, y: y) { return false }
    
    // Check if the entire segment stays inside
    // Get interior ranges at this y
    let interiorRanges = getInsideXRanges(atY: y)
    
    // Collect all horizontal edges at this y
    var boundaryRanges: [(Int, Int)] = []
    for edge in horizontalEdges where edge.y1 == y {
        boundaryRanges.append((edge.minX, edge.maxX))
    }
    
    // Merge interior and boundary ranges
    var allRanges = interiorRanges + boundaryRanges
    allRanges.sort { $0.0 < $1.0 }
    
    // Merge overlapping/adjacent ranges
    var merged: [(Int, Int)] = []
    for range in allRanges {
        if merged.isEmpty {
            merged.append(range)
        } else if range.0 <= merged.last!.1 {
            merged[merged.count - 1].1 = max(merged.last!.1, range.1)
        } else {
            merged.append(range)
        }
    }
    
    // Check if [minX, maxX] is contained in any merged range
    for (rx1, rx2) in merged {
        if rx1 <= minX && maxX <= rx2 {
            return true
        }
    }
    return false
}

// Get min/max x range of polygon interior (strictly inside) at y
// Only valid for y values NOT on horizontal edges
func getStrictInteriorXRange(atY y: Int) -> (Int, Int)? {
    var crossings: [Int] = []
    for edge in verticalEdges {
        if edge.minY < y && y < edge.maxY {
            crossings.append(edge.x1)
        }
    }
    guard crossings.count >= 2 else { return nil }
    crossings.sort()
    // Return the outermost interval (leftmost to rightmost)
    return (crossings.first!, crossings.last!)
}

// Function to check if a rectangle is fully inside the polygon
func isRectangleInside(x1: Int, y1: Int, x2: Int, y2: Int) -> Bool {
    let minX = min(x1, x2)
    let maxX = max(x1, x2)
    let minY = min(y1, y2)
    let maxY = max(y1, y2)
    
    // Both corners must be red points (vertices)
    let corner1 = (x1, y1)
    let corner2 = (x2, y2)
    let pointSet = Set(points.map { "\($0.0),\($0.1)" })
    if !pointSet.contains("\(corner1.0),\(corner1.1)") { return false }
    if !pointSet.contains("\(corner2.0),\(corner2.1)") { return false }
    
    // Get all critical y values (polygon vertex y-coords)
    let criticalYs = allYs.filter { $0 >= minY && $0 <= maxY }.sorted()
    
    // For each strip between consecutive critical y values, check if the x range fits
    for i in 0..<criticalYs.count {
        let y = criticalYs[i]
        
        // At each vertex y, check horizontal edges cover [minX, maxX]
        // Collect all horizontal edge x-ranges at this y
        var hRanges: [(Int, Int)] = []
        for edge in horizontalEdges where edge.y1 == y {
            hRanges.append((edge.minX, edge.maxX))
        }
        
        // Also need interior coverage at this y
        // Test slightly above and below to get interior x-range
        if i < criticalYs.count - 1 {
            let testY = (criticalYs[i] + criticalYs[i + 1]) / 2
            if let (intMinX, intMaxX) = getStrictInteriorXRange(atY: testY) {
                hRanges.append((intMinX, intMaxX))
            }
        }
        if i > 0 {
            let testY = (criticalYs[i - 1] + criticalYs[i]) / 2
            if let (intMinX, intMaxX) = getStrictInteriorXRange(atY: testY) {
                hRanges.append((intMinX, intMaxX))
            }
        }
        
        // Merge ranges
        hRanges.sort { $0.0 < $1.0 }
        var merged: [(Int, Int)] = []
        for range in hRanges {
            if merged.isEmpty {
                merged.append(range)
            } else if range.0 <= merged.last!.1 + 1 {
                merged[merged.count - 1].1 = max(merged.last!.1, range.1)
            } else {
                merged.append(range)
            }
        }
        
        // Check if [minX, maxX] fits in merged
        var fits = false
        for (rx1, rx2) in merged {
            if rx1 <= minX && maxX <= rx2 {
                fits = true
                break
            }
        }
        if !fits { return false }
    }
    
    // Check strips between critical y values
    for i in 0..<(criticalYs.count - 1) {
        let testY = (criticalYs[i] + criticalYs[i + 1]) / 2
        if let (intMinX, intMaxX) = getStrictInteriorXRange(atY: testY) {
            if intMinX > minX || intMaxX < maxX {
                return false
            }
        } else {
            return false
        }
    }
    
    return true
}

// Find max valid rectangle area
var maxArea = 0

for i in 0..<points.count {
    for j in (i+1)..<points.count {
        let (x1, y1) = points[i]
        let (x2, y2) = points[j]
        
        // Skip if same row or column (area would be 0 or a line)
        if x1 == x2 || y1 == y2 { continue }
        
        let area = (abs(x2 - x1) + 1) * (abs(y2 - y1) + 1)
        
        // Only check if potentially larger
        if area > maxArea {
            if isRectangleInside(x1: x1, y1: y1, x2: x2, y2: y2) {
                maxArea = area
            }
        }
    }
}

print("Answer: \(maxArea)")
