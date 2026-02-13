# Problem 4056: Longest Balanced Substring II

**Difficulty:** Medium  
**Topics:** Hash Table, String, Prefix Sum

---

## Problem Description

You are given a string `s` consisting only of the characters `'a'`, `'b'`, and `'c'`.

A 
substring of `s` is called 
balanced if all 
distinct characters in the 
substring appear the 
same number of times.

Return the 
length of the longest balanced substring of `s`.

### Example
Example 1
**Input:** s = "abbac"

**Output:**
4

**Explanation:** The longest balanced substring is `"abba"` because both distinct characters `'a'` and `'b'` each appear exactly 2 times.

### Example
Example 2
**Input:** s = "aabcc"

**Output:**
3

**Explanation:** The longest balanced substring is `"abc"` because all distinct characters `'a'`, `'b'` and `'c'` each appear exactly 1 time.

### Example
Example 3
**Input:** s = "aba"

**Output:**
2

**Explanation:** One of the longest balanced substrings is `"ab"` because both distinct characters `'a'` and `'b'` each appear exactly 1 time. Another longest balanced substring is `"ba"`.

### Constraints
- `1 <= s.length <= 10^5`

 - `s` contains only the characters `'a'`, `'b'`, and `'c'`.
