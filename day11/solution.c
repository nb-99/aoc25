#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define MAX_NODES 1024
#define MAX_NAME_LEN 8
#define MAX_EDGES 32
#define MAX_LINE_LEN 512

typedef struct {
    char name[MAX_NAME_LEN];
    int edges[MAX_EDGES];
    int edge_count;
} Node;

static Node nodes[MAX_NODES];
static int node_count = 0;
static int64_t memo[MAX_NODES];  // -1 = not computed
static int64_t memo2[MAX_NODES][4];  // Part 2: memo[node][state], -1 = not computed
static int you_idx = -1;
static int out_idx = -1;
static int svr_idx = -1;
static int dac_idx = -1;
static int fft_idx = -1;

// Find or create node by name
int get_or_create_node(const char *name) {
    for (int i = 0; i < node_count; i++) {
        if (strcmp(nodes[i].name, name) == 0) {
            return i;
        }
    }
    // Create new node
    strncpy(nodes[node_count].name, name, MAX_NAME_LEN - 1);
    nodes[node_count].name[MAX_NAME_LEN - 1] = '\0';
    nodes[node_count].edge_count = 0;
    return node_count++;
}

// Count paths from node_idx to out_idx using memoization (Part 1)
int64_t count_paths(int node_idx) {
    if (node_idx == out_idx) {
        return 1;
    }
    
    if (memo[node_idx] != -1) {
        return memo[node_idx];
    }
    
    int64_t total = 0;
    Node *node = &nodes[node_idx];
    
    for (int i = 0; i < node->edge_count; i++) {
        total += count_paths(node->edges[i]);
    }
    
    memo[node_idx] = total;
    return total;
}

// Part 2: Count paths from node_idx to out that visit both dac and fft
// state is a bitmask: bit 0 = dac visited, bit 1 = fft visited
int64_t count_paths2(int node_idx, int state) {
    // Update state if we're at dac or fft
    if (node_idx == dac_idx) state |= 1;
    if (node_idx == fft_idx) state |= 2;
    
    if (node_idx == out_idx) {
        // Only count if both dac and fft were visited (state == 3)
        return (state == 3) ? 1 : 0;
    }
    
    if (memo2[node_idx][state] != -1) {
        return memo2[node_idx][state];
    }
    
    int64_t total = 0;
    Node *node = &nodes[node_idx];
    
    for (int i = 0; i < node->edge_count; i++) {
        total += count_paths2(node->edges[i], state);
    }
    
    memo2[node_idx][state] = total;
    return total;
}

int main(void) {
    FILE *fp = fopen("input.csv", "r");
    if (!fp) {
        perror("Failed to open input.csv");
        return 1;
    }
    
    // Initialize memo arrays
    memset(memo, -1, sizeof(memo));
    memset(memo2, -1, sizeof(memo2));
    
    char line[MAX_LINE_LEN];
    
    // Parse input
    while (fgets(line, sizeof(line), fp)) {
        // Remove newline
        size_t len = strlen(line);
        if (len > 0 && line[len-1] == '\n') line[len-1] = '\0';
        if (len > 1 && line[len-2] == '\r') line[len-2] = '\0';
        
        // Skip empty lines
        if (strlen(line) == 0) continue;
        
        // Parse "name: dest1 dest2 ..."
        char *colon = strchr(line, ':');
        if (!colon) continue;
        
        *colon = '\0';
        char *src_name = line;
        char *rest = colon + 1;
        
        // Trim leading space from src_name
        while (*src_name == ' ') src_name++;
        
        int src_idx = get_or_create_node(src_name);
        
        // Parse destinations
        char *token = strtok(rest, " \t");
        while (token) {
            int dest_idx = get_or_create_node(token);
            if (nodes[src_idx].edge_count < MAX_EDGES) {
                nodes[src_idx].edges[nodes[src_idx].edge_count++] = dest_idx;
            }
            token = strtok(NULL, " \t");
        }
    }
    
    fclose(fp);
    
    // Find required node indices
    for (int i = 0; i < node_count; i++) {
        if (strcmp(nodes[i].name, "you") == 0) you_idx = i;
        if (strcmp(nodes[i].name, "out") == 0) out_idx = i;
        if (strcmp(nodes[i].name, "svr") == 0) svr_idx = i;
        if (strcmp(nodes[i].name, "dac") == 0) dac_idx = i;
        if (strcmp(nodes[i].name, "fft") == 0) fft_idx = i;
    }
    
    if (you_idx == -1 || out_idx == -1) {
        fprintf(stderr, "Could not find 'you' or 'out' nodes\n");
        return 1;
    }
    
    // Part 1: Count all paths from "you" to "out"
    int64_t result1 = count_paths(you_idx);
    printf("Part 1: %lld\n", (long long)result1);
    
    // Part 2: Count paths from "svr" to "out" that visit both dac and fft
    if (svr_idx == -1 || dac_idx == -1 || fft_idx == -1) {
        fprintf(stderr, "Could not find 'svr', 'dac', or 'fft' nodes for Part 2\n");
        return 1;
    }
    
    int64_t result2 = count_paths2(svr_idx, 0);
    printf("Part 2: %lld\n", (long long)result2);
    
    return 0;
}
