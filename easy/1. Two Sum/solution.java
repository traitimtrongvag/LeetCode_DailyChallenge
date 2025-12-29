class Solution{
    public int[] twoSum(int[] nums,int target){
        java.util.HashMap<Integer,Integer> m=new java.util.HashMap<>();
        for(int i=0;i<nums.length;i++){
            int c=target-nums[i];
            if(m.containsKey(c))return new int[]{m.get(c),i};
            m.put(nums[i],i);
        }
        return new int[0];
    }
}