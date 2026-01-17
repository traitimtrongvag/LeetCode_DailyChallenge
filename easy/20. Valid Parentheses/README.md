# Problem 20: Valid Parentheses

**Difficulty:** Easy  
**Topics:** String, Stack

---

## Problem Description

Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.

An input string is valid if
 - Open brackets must be closed by the same type of brackets.

 - Open brackets must be closed in the correct order.

 - Every close bracket has a corresponding open bracket of the same type.

### Example
Example 1
**Input:** s = "()"

**Output:** true

### Example
Example 2
**Input:** s = "()[]{}"

**Output:** true

### Example
Example 3
**Input:** s = "(]"

**Output:** false

### Example
Example 4
**Input:** s = "([])"

**Output:** true

### Example
Example 5
**Input:** s = "([)]"

**Output:** false

### Constraints
- `1 <= s.length <= 10^4`

 - `s` consists of parentheses only `'()[]{}'`.
