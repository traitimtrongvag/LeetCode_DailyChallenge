# Problem 1756: Minimum Deletions to Make String Balanced

**Difficulty:** Medium  
**Topics:** String, Dynamic Programming, Stack

---

## Problem Description

You are given a string `s` consisting only of characters `'a'` and `'b'`​​​​.

You can delete any number of characters in `s` to make `s` 
balanced. `s` is 
balanced if there is no pair of indices `(i,j)` such that `i babbab" -> "aaabbb"), or
Delete the characters at 0-indexed positions 3 and 6 ("aababbab" -> "aabbbb").

### Example
Example 2
**Input:** s = "bbaaaaabb"

**Output:**
2

**Explanation:** The only solution is to delete the first two characters.

### Constraints
- `1 <= s.length <= 10^5`

 - `s[i]` is `'a'` or `'b'`​​.
