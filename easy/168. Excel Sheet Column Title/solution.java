class Solution {
    public String convertToTitle(int columnNumber) {
        StringBuilder result = new StringBuilder();
        while (columnNumber > 0) {
            columnNumber--;
            result.insert(0, (char)('A' + columnNumber % 26));
            columnNumber /= 26;
        }
        return result.toString();
    }
}