#include <algorithm>
#include <climits>
#include <vector>

class Solution {
public:
    std::vector<std::vector<int>> minimumAbsDifference(std::vector<int>& arr) {
        std::sort(arr.begin(), arr.end());
        int minDiff = INT_MAX;

        for (std::size_t i = 1; i < arr.size(); i++) {
            minDiff = std::min(minDiff, arr[i] - arr[i - 1]);
        }

        std::vector<std::vector<int>> result;
        for (std::size_t i = 1; i < arr.size(); i++) {
            if (arr[i] - arr[i - 1] == minDiff) {
                result.push_back({arr[i - 1], arr[i]});
            }
        }

        return result;
    }
};
