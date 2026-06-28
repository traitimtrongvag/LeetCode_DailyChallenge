class Solution {
public:
    int maximumElementAfterDecrementingAndRearranging(vector<int>& arr) {
        int n = arr.size();
        vector<int> freq(n + 1,0);
        for(int i =0;i<n;i++){
            freq[min(arr[i],n)]++;
        }
        int ans = 1;
        for(int i =2;i<=n;i++){
            if(freq[i] == 0) continue;

            ans = min(ans + freq[i],i);
        }
        return ans;
    }
};