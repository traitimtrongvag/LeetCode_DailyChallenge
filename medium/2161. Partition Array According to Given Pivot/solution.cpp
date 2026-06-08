class Solution {
public:
    vector<int> pivotArray(vector<int>& nums, int pivot) {
        int n = nums.size();
        vector<int> ans(n);
        
        int left = 0;
        int right = n - 1;
        int i = 0, j =n -1;
        
        while(i < n){
            if (nums[i] < pivot) {
                ans[left++] = nums[i];
            }
            
            if (nums[j] > pivot) {
                ans[right--] = nums[j];
            }

            i++;
            j--;
        }
        
        //remaining indices will be filled with pivot
        while (left <= right) {
            ans[left++] = pivot;
        }
        
        return ans;
    }
};