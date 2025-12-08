using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Solution
{
    static void Main()
    {
        var lines = File.ReadAllLines("input.csv");
        var points = lines.Select(line =>
        {
            var parts = line.Split(',');
            return (x: long.Parse(parts[0]), y: long.Parse(parts[1]), z: long.Parse(parts[2]));
        }).ToList();

        int n = points.Count;
        
        // Calculate all pairwise distances with indices
        var pairs = new List<(long distSq, int i, int j)>();
        for (int i = 0; i < n; i++)
        {
            for (int j = i + 1; j < n; j++)
            {
                long dx = points[i].x - points[j].x;
                long dy = points[i].y - points[j].y;
                long dz = points[i].z - points[j].z;
                long distSq = dx * dx + dy * dy + dz * dz;
                pairs.Add((distSq, i, j));
            }
        }

        // Sort by distance
        pairs.Sort((a, b) => a.distSq.CompareTo(b.distSq));

        // Union-Find
        int[] parent = new int[n];
        int[] rank = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;

        int Find(int x)
        {
            if (parent[x] != x) parent[x] = Find(parent[x]);
            return parent[x];
        }

        bool Union(int x, int y)
        {
            int px = Find(x), py = Find(y);
            if (px == py) return false; // Already in same component
            if (rank[px] < rank[py]) (px, py) = (py, px);
            parent[py] = px;
            if (rank[px] == rank[py]) rank[px]++;
            return true; // Actually merged
        }

        // Connect pairs until all in one circuit
        int components = n;
        int lastI = -1, lastJ = -1;
        
        foreach (var (distSq, i, j) in pairs)
        {
            if (Union(i, j))
            {
                components--;
                lastI = i;
                lastJ = j;
                if (components == 1) break;
            }
        }

        long result = points[lastI].x * points[lastJ].x;
        Console.WriteLine($"Last connection: ({points[lastI].x},{points[lastI].y},{points[lastI].z}) and ({points[lastJ].x},{points[lastJ].y},{points[lastJ].z})");
        Console.WriteLine($"Answer: {points[lastI].x} * {points[lastJ].x} = {result}");
    }
}
