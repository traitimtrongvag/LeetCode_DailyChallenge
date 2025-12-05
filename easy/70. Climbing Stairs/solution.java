class Solution {
    public static void helper(int n , int count[],HashMap<Integer,Integer> map ){
        if(n < 0 ) return ;
        if(n == 0){
            count[0] = count[0] + 1;
            return; }

        if(map.containsKey(n-1)) count[0] = count[0] + map.get(n-1);
        else helper(n - 1, count,map);
        
        if(map.containsKey(n-2)) count[0] = count[0] + map.get(n-2);
        else helper(n - 2, count,map);

        map.put(n,count[0]);
    }

    public int climbStairs(int n) {
        int count[] = {0};
        HashMap<Integer,Integer> map = new HashMap<>();
        helper(n,count,map);
        return count[0];
    }
}