/*
Time: O(n * d)
Space: O(1)

Idea: store all prefixes of nums1 in a set, then check prefixes of nums2
*/

class Solution {
public:
    int longestCommonPrefix(vector<int>& arr1, vector<int>& arr2) {
        unordered_set<int> st;

        // store all prefixes of arr1 numbers
        for (int x : arr1) {
            while (x > 0) {
                st.insert(x);
                x /= 10;
            }
        }

        int ans = 0;

        // check prefixes of arr2 numbers
        for (int x : arr2) {
            while (x > 0) {
                if (st.count(x)) {
                    ans = max(ans, (int)to_string(x).size());
                    break;
                }
                x /= 10;
            }
        }

        return ans;
    }
};