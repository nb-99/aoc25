const fs = require('fs');

// Read input
const input = fs.readFileSync('./input.csv', 'utf8').trim();

// Parse ranges
const ranges = input.split(',').filter(r => r.length > 0).map(range => {
    const [start, end] = range.split('-').map(Number);
    return { start, end };
});

// Check if a number is "invalid" (digit sequence repeated twice)
function isInvalidId(num) {
    const str = num.toString();
    const len = str.length;
    
    // Must have even length to be a repeated pattern
    if (len % 2 !== 0) return false;
    
    const half = len / 2;
    const firstHalf = str.substring(0, half);
    const secondHalf = str.substring(half);
    
    return firstHalf === secondHalf;
}

// Generate the smallest repeated-pattern number with given half-length
// e.g., halfLen=1 -> 11, halfLen=2 -> 1010, halfLen=3 -> 100100
function minRepeated(halfLen) {
    const minHalf = Math.pow(10, halfLen - 1); // smallest number with halfLen digits
    return BigInt(minHalf.toString() + minHalf.toString());
}

// Generate the largest repeated-pattern number with given half-length
// e.g., halfLen=1 -> 99, halfLen=2 -> 9999, halfLen=3 -> 999999
function maxRepeated(halfLen) {
    const maxHalf = Math.pow(10, halfLen) - 1; // largest number with halfLen digits
    return BigInt(maxHalf.toString() + maxHalf.toString());
}

// Find all invalid IDs in a range
function findInvalidIds(start, end) {
    const invalidIds = [];
    const startBig = BigInt(start);
    const endBig = BigInt(end);
    
    // Determine the range of half-lengths to check
    const startLen = start.toString().length;
    const endLen = end.toString().length;
    
    // For each possible half-length
    for (let halfLen = 1; halfLen <= 10; halfLen++) {
        const fullLen = halfLen * 2;
        
        // Skip if this full length is outside our range's digit count
        if (fullLen < startLen || fullLen > endLen + 1) continue;
        
        // Iterate through all possible "halves" for this half-length
        const minHalf = Math.pow(10, halfLen - 1);
        const maxHalf = Math.pow(10, halfLen) - 1;
        
        for (let half = minHalf; half <= maxHalf; half++) {
            // Construct the repeated number
            const repeated = BigInt(half.toString() + half.toString());
            
            // Check if it falls within our range
            if (repeated >= startBig && repeated <= endBig) {
                invalidIds.push(repeated);
            }
        }
    }
    
    return invalidIds;
}

// Process all ranges and sum invalid IDs
let totalSum = BigInt(0);
let allInvalidIds = [];

for (const range of ranges) {
    const invalidIds = findInvalidIds(range.start, range.end);
    for (const id of invalidIds) {
        allInvalidIds.push(id);
        totalSum += id;
    }
}

console.log('Invalid IDs found:', allInvalidIds.length);
console.log('Sum of all invalid IDs:', totalSum.toString());
