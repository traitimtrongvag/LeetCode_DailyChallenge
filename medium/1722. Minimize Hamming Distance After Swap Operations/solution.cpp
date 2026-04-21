// idea: https://youtu.be/L9reCSJHMHQ?si=tqmlQ9n4hF0VaWVg

class UnionFind {
private:
    vector<int> parent;
    vector<int> rank;
public:
    UnionFind(int n) {
        parent.resize(n);
        rank.resize(n, 0);
        for (int i = 0; i < n; ++i) parent[i] = i; // each node is its own parent
    }

    int find(int x) {
        // path compression
        if (parent[x] != x) parent[x] = find(parent[x]);
        return parent[x];
    }

    void unite(int x, int y) {
        int rx = find(x), ry = find(y);
        if (rx == ry) return;
        // union by rank
        if (rank[rx] < rank[ry]) parent[rx] = ry;
        else if (rank[rx] > rank[ry]) parent[ry] = rx;
        else {
            parent[ry] = rx;
            rank[rx]++;
        }
    }
};

class Solution {
public:
    int minimumHammingDistance(vector<int>& source, vector<int>& target, vector<vector<int>>& allowedSwaps) {
        int n = source.size();
        UnionFind uf(n);

        // build connected components from allowed swaps
        for (auto& swapPair : allowedSwaps) {
            uf.unite(swapPair[0], swapPair[1]);
        }

        // group indices by their component root
        unordered_map<int, vector<int>> components;
        for (int i = 0; i < n; ++i) {
            components[uf.find(i)].push_back(i);
        }

        int same = 0; // count of positions that can match after swaps

        // for each component, count matches between source and target
        for (auto& entry : components) {
            vector<int>& indices = entry.second;
            unordered_map<int, int> freq;
            // frequency of source values in this component
            for (int idx : indices) freq[source[idx]]++;

            // try to match target values
            for (int idx : indices) {
                int val = target[idx];
                if (freq[val] > 0) {
                    same++;
                    freq[val]--; // use one occurrence
                }
            }
        }

        // Hamming distance = total positions - positions we can make equal
        return n - same;
    }
};