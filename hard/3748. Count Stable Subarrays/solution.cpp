#include <algorithm>
using namespace std;

struct Fenwick {
    int n;
    vector<long long> bit;
    Fenwick(int n_) : n(n_), bit(n_ + 2, 0) {}
    void update(int idx, long long delta) {
        idx++;
        while (idx <= n) {
            bit[idx] += delta;
            idx += idx & -idx;
        }
    }
    long long query(int idx) {
        idx++;
        long long sum = 0;
        while (idx > 0) {
            sum += bit[idx];
            idx -= idx & -idx;
        }
        return sum;
    }
    long long range_sum(int l, int r) {
        if (l > r) return 0;
        return query(r) - query(l - 1);
    }
};

class Solution {
public:
    vector<long long> countStableSubarrays(vector<int>& nums, vector<vector<int>>& queries) {
        int n = nums.size();
        vector<int> next(n, n);
        for (int i = n - 2; i >= 0; i--) {
            if (nums[i] <= nums[i + 1])
                next[i] = next[i + 1];
            else
                next[i] = i + 1;
        }
        vector<int> len(n);
        for (int i = 0; i < n; i++) {
            len[i] = next[i] - i;
        }

        int q = queries.size();
        vector<int> q_order(q);
        for (int i = 0; i < q; i++) q_order[i] = i;
        sort(q_order.begin(), q_order.end(), [&](int a, int b) {
            return queries[a][1] < queries[b][1];
        });

        Fenwick bitLen(n), bitCnt(n), bitIdx(n);
        vector<vector<int>> events(n + 1);
        for (int i = 0; i < n; i++) {
            if (next[i] < n) {
                events[next[i] - 1].push_back(i);
            }
        }

        vector<long long> ans(q);
        int r_ptr = 0;
        for (int idx : q_order) {
            int l = queries[idx][0], r = queries[idx][1];
            while (r_ptr <= r) {
                bitCnt.update(r_ptr, 1);
                bitIdx.update(r_ptr, r_ptr);
                for (int i : events[r_ptr]) {
                    bitCnt.update(i, -1);
                    bitIdx.update(i, -i);
                    bitLen.update(i, len[i]);
                }
                r_ptr++;
            }
            long long case1_sum = bitLen.range_sum(l, r);
            long long case2_cnt = bitCnt.range_sum(l, r);
            long long case2_idx_sum = bitIdx.range_sum(l, r);
            long long case2_sum = (r + 1) * case2_cnt - case2_idx_sum;
            ans[idx] = case1_sum + case2_sum;
        }
        return ans;
    }
};