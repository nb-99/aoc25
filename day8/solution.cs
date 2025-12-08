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

        // Sort by distance (squared is fine for comparison)
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

        void Union(int x, int y)
        {
            int px = Find(x), py = Find(y);
            if (px == py) return;
            if (rank[px] < rank[py]) (px, py) = (py, px);
            parent[py] = px;
            if (rank[px] == rank[py]) rank[px]++;
        }

        // Connect the 1000 closest pairs
        for (int i = 0; i < 1000; i++)
        {
            Union(pairs[i].i, pairs[i].j);
        }

        // Count component sizes
        var sizes = new Dictionary<int, int>();
        for (int i = 0; i < n; i++)
        {
            int root = Find(i);
            sizes[root] = sizes.GetValueOrDefault(root, 0) + 1;
        }

        // Get top 3 largest
        var top3 = sizes.Values.OrderByDescending(x => x).Take(3).ToList();
        long result = (long)top3[0] * top3[1] * top3[2];

        Console.WriteLine($"Top 3 circuit sizes: {string.Join(", ", top3)}");
        Console.WriteLine($"Answer: {result}");
    }
}
