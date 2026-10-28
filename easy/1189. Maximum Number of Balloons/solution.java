class Solution {
    public int maxNumberOfBalloons(String text) {
        int[] counts = new int[26];
        for (char c : text.toCharArray()) {
            counts[c - 'a']++;
        }

        return Math.min(
            Math.min(counts['b' - 'a'], counts['a' - 'a']),
            Math.min(counts['n' - 'a'],
                Math.min(counts['l' - 'a'] / 2, counts['o' - 'a'] / 2))
        );
    }
}
