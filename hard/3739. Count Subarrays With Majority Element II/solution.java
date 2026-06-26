class Solution {
    public long countMajoritySubarrays(int[] nums, int target) {
        int n = nums.length;
        
        int prefix []= new int[n+1];
        long count = 0 ;
        for(int i =0;i<n;i++){
           prefix[i+1] = prefix[i] + (nums[i] == target ? 1 : -1);
        }

        HashMap<Integer,Integer> map = new HashMap<>();
        int k = 0 ;

        for(int m =0;m<n;m++){
            if(prefix[m]<prefix[m+1]) k+=map.getOrDefault(prefix[m],0) +1;
            else k-=map.getOrDefault(prefix[m]-1,0);

            count+=k;
            map.merge(prefix[m],1,Integer::sum);
        }
        return count;
    }
}