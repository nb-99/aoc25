const fs = require('fs');

// Read input
const input = fs.readFileSync('./input.csv', 'utf8').trim();

// Parse ranges
const ranges = input.split(',').filter(r => r.length > 0).map(range => {
    const [start, end] = range.split('-').map(Number);
    return { start, end };
});

// Check if a number is "invalid" (digit sequence repeated at least twice)
function isInvalidId(num) {
    const str = num.toString();
    const len = str.length;
    
    // Try each possible pattern length (must divide evenly into total length)
    for (let patternLen = 1; patternLen <= len / 2; patternLen++) {
        if (len % patternLen !== 0) continue;
        
        const pattern = str.substring(0, patternLen);
        const repetitions = len / patternLen;
        
        // Check if the entire string is this pattern repeated
        if (pattern.repeat(repetitions) === str) {
            return true;
        }
    }
    
    return false;
}

// Find all invalid IDs in a range by generating candidates
function findInvalidIds(start, end) {
    const invalidIds = new Set();
    const startBig = BigInt(start);
    const endBig = BigInt(end);
    
    const startLen = start.toString().length;
    const endLen = end.toString().length;
    
    // For each total length that could appear in our range
    for (let totalLen = startLen; totalLen <= endLen; totalLen++) {
        // For each possible pattern length that divides totalLen
        for (let patternLen = 1; patternLen <= totalLen / 2; patternLen++) {
            if (totalLen % patternLen !== 0) continue;
            
            const repetitions = totalLen / patternLen;
            
            // Generate all patterns of this length
            const minPattern = patternLen === 1 ? 1 : Math.pow(10, patternLen - 1);
            const maxPattern = Math.pow(10, patternLen) - 1;
            
            for (let pattern = minPattern; pattern <= maxPattern; pattern++) {
                // Construct the repeated number
                const patternStr = pattern.toString();
                const repeatedStr = patternStr.repeat(repetitions);
                const repeated = BigInt(repeatedStr);
                
                // Check if it falls within our range
                if (repeated >= startBig && repeated <= endBig) {
                    invalidIds.add(repeated);
                }
            }
        }
    }
    
    return Array.from(invalidIds);
}

// Process all ranges and sum invalid IDs
let totalSum = BigInt(0);
let totalCount = 0;

for (const range of ranges) {
    const invalidIds = findInvalidIds(range.start, range.end);
    for (const id of invalidIds) {
        totalCount++;
        totalSum += id;
    }
}

console.log('Invalid IDs found:', totalCount);
console.log('Sum of all invalid IDs:', totalSum.toString());
