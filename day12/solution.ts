import * as fs from 'fs';

type Shape = boolean[][];
type Point = [number, number];

function parseInput(input: string): { shapes: Shape[], regions: { width: number, height: number, counts: number[] }[] } {
    const parts = input.trim().split('\n\n');
    const shapes: Shape[] = [];
    const regions: { width: number, height: number, counts: number[] }[] = [];

    for (const part of parts) {
        const lines = part.split('\n');
        if (lines[0].match(/^\d+:$/)) {
            // Shape definition
            const shapeLines = lines.slice(1);
            const shape: Shape = shapeLines.map(line => 
                line.split('').map(c => c === '#')
            );
            shapes.push(shape);
        } else {
            // Region definitions
            for (const line of lines) {
                const match = line.match(/^(\d+)x(\d+): (.+)$/);
                if (match) {
                    const width = parseInt(match[1]);
                    const height = parseInt(match[2]);
                    const counts = match[3].split(' ').map(Number);
                    regions.push({ width, height, counts });
                }
            }
        }
    }

    return { shapes, regions };
}

function getShapePoints(shape: Shape): Point[] {
    const points: Point[] = [];
    for (let r = 0; r < shape.length; r++) {
        for (let c = 0; c < shape[r].length; c++) {
            if (shape[r][c]) {
                points.push([r, c]);
            }
        }
    }
    return points;
}

function normalizePoints(points: Point[]): Point[] {
    if (points.length === 0) return [];
    const minR = Math.min(...points.map(p => p[0]));
    const minC = Math.min(...points.map(p => p[1]));
    const normalized = points.map(p => [p[0] - minR, p[1] - minC] as Point);
    normalized.sort((a, b) => a[0] !== b[0] ? a[0] - b[0] : a[1] - b[1]);
    return normalized;
}

function rotatePoints(points: Point[]): Point[] {
    return points.map(([r, c]) => [c, -r] as Point);
}

function flipPoints(points: Point[]): Point[] {
    return points.map(([r, c]) => [r, -c] as Point);
}

function pointsToString(points: Point[]): string {
    return points.map(p => `${p[0]},${p[1]}`).join(';');
}

function getAllOrientations(shape: Shape): Point[][] {
    const orientations: Point[][] = [];
    const seen = new Set<string>();
    
    let points = getShapePoints(shape);
    
    for (let flip = 0; flip < 2; flip++) {
        for (let rot = 0; rot < 4; rot++) {
            const normalized = normalizePoints(points);
            const key = pointsToString(normalized);
            if (!seen.has(key)) {
                seen.add(key);
                orientations.push(normalized);
            }
            points = rotatePoints(points);
        }
        points = flipPoints(points);
    }
    
    return orientations;
}

function canPlace(grid: boolean[][], points: Point[], startR: number, startC: number, width: number, height: number): boolean {
    for (const [dr, dc] of points) {
        const r = startR + dr;
        const c = startC + dc;
        if (r < 0 || r >= height || c < 0 || c >= width || grid[r][c]) {
            return false;
        }
    }
    return true;
}

function place(grid: boolean[][], points: Point[], startR: number, startC: number, value: boolean): void {
    for (const [dr, dc] of points) {
        grid[startR + dr][startC + dc] = value;
    }
}

function solve(
    grid: boolean[][],
    width: number,
    height: number,
    shapeOrientations: Point[][][],
    remaining: number[],
    pieceIndex: number
): boolean {
    // Find next piece to place
    while (pieceIndex < remaining.length && remaining[pieceIndex] === 0) {
        pieceIndex++;
    }
    
    if (pieceIndex >= remaining.length) {
        return true; // All pieces placed
    }
    
    const orientations = shapeOrientations[pieceIndex];
    
    for (let r = 0; r < height; r++) {
        for (let c = 0; c < width; c++) {
            for (const points of orientations) {
                if (canPlace(grid, points, r, c, width, height)) {
                    place(grid, points, r, c, true);
                    remaining[pieceIndex]--;
                    
                    if (solve(grid, width, height, shapeOrientations, remaining, pieceIndex)) {
                        return true;
                    }
                    
                    remaining[pieceIndex]++;
                    place(grid, points, r, c, false);
                }
            }
        }
    }
    
    return false;
}

function canFitAll(
    width: number,
    height: number,
    shapeOrientations: Point[][][],
    counts: number[]
): boolean {
    // Quick check: total cells needed vs available
    let totalCells = 0;
    for (let i = 0; i < counts.length; i++) {
        if (counts[i] > 0 && shapeOrientations[i].length > 0) {
            totalCells += counts[i] * shapeOrientations[i][0].length;
        }
    }
    if (totalCells > width * height) {
        return false;
    }
    
    const grid: boolean[][] = Array.from({ length: height }, () => 
        Array(width).fill(false)
    );
    const remaining = [...counts];
    
    return solve(grid, width, height, shapeOrientations, remaining, 0);
}

function main() {
    const input = fs.readFileSync('input.csv', 'utf-8');
    const { shapes, regions } = parseInput(input);
    
    // Precompute all orientations for each shape
    const shapeOrientations = shapes.map(shape => getAllOrientations(shape));
    
    let count = 0;
    for (let i = 0; i < regions.length; i++) {
        const { width, height, counts } = regions[i];
        if (canFitAll(width, height, shapeOrientations, counts)) {
            count++;
        }
        if ((i + 1) % 100 === 0) {
            console.log(`Processed ${i + 1}/${regions.length} regions...`);
        }
    }
    
    console.log(`\nAnswer: ${count}`);
}

main();
