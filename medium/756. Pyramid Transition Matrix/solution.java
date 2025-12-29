// idea: https://leetcode.com/problems/pyramid-transition-matrix/solutions/7447127/trying-all-combinations-by-tw3rkekjq6-4suh/?envType=daily-question&envId=2025-12-29
class Solution {
    HashMap<String,ArrayList<Character>> map;
    HashMap<String,Boolean> memo;
    public void backtrack(ArrayList<String> res, String prev, StringBuilder sb, int k){
        if (k==prev.length()-1) {
            res.add(sb.toString());
            return;
        }
        String key = "" + prev.charAt(k) + prev.charAt(k+1);
        ArrayList<Character> list  = map.get(key); 
        if (list==null) return;
        for (char c : list){
            sb.append(c);
            backtrack(res,prev,sb,k+1);
            sb.deleteCharAt(sb.length()-1 );
        }
    }
    public boolean dfs(String row){
        if (row.length()==1) return true;
        if (memo.containsKey(row)) return memo.get(row);
        ArrayList<String> rows = new ArrayList<>();
        backtrack(rows,row,new StringBuilder(),0); 
        for (String s : rows){
            if (dfs(s)){
                memo.put(row,true);
                return true;
            }
        }
        memo.put(row,false);
        return false;
    }
    public boolean pyramidTransition(String bottom, List<String> allowed) {
        map = new HashMap<>();
        memo = new HashMap<>();
        for (String s : allowed){
            StringBuilder sb = new StringBuilder(); 
            sb.append(s.charAt(0));
            sb.append(s.charAt(1)); 
            map.computeIfAbsent(sb.toString(),k->new ArrayList<Character>()).add(s.charAt(2));
        }
        return dfs(bottom);
    }
}