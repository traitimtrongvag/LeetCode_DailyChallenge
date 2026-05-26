# Problem 3408: Count the Number of Special Characters I

**Difficulty:** Easy  
**Topics:** Hash Table, String

---

## Problem Description

You are given a string `word`. A letter is called 
special if it appears 
both in lowercase and uppercase in `word`.

Return the number of* *
special letters in* *`word`.

 

### Example
Example 1
**Input:** word = "aaAbcBC"

**Output:**
3

**Explanation:** The special characters in `word` are `'a'`, `'b'`, and `'c'`.

### Example
Example 2
**Input:** word = "abc"

**Output:**
0

**Explanation:** No character in `word` appears in uppercase.

### Example
Example 3
**Input:** word = "abBCab"

**Output:**
1

**Explanation:** The only special character in `word` is `'b'`.

 

### Constraints
- `1 <= word.length <= 50`

 - `word` consists of only lowercase and uppercase English letters.
