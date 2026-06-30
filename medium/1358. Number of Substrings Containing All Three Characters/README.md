# Problem 1460: Number of Substrings Containing All Three Characters

**Difficulty:** Medium  
**Topics:** Hash Table, String, Sliding Window

---

## Problem Description

Given a string `s` consisting only of characters *a*, *b* and *c*.

Return the number of substrings containing at least one occurrence of all these characters *a*, *b* and *c*.

 

### Example
Example 1
**Input:** s = "abcabc"

**Output:**
10

**Explanation:** The substrings containing at least one occurrence of the characters *a*, *b* and *c are "*abc*", "*abca*", "*abcab*", "*abcabc*", "*bca*", "*bcab*", "*bcabc*", "*cab*", "*cabc*" *and* "*abc*" *(
again)*. *

### Example
Example 2
**Input:** s = "aaacb"

**Output:**
3

**Explanation:** The substrings containing at least one occurrence of the characters *a*, *b* and *c are "*aaacb*", "*aacb*" *and* "*acb*".** *

### Example
Example 3
**Input:** s = "abc"

**Output:**
1

 

### Constraints
- `3 <= s.length <= 5 x 10^4`

 - `s` only consists of *a*, *b* or *c *characters.
