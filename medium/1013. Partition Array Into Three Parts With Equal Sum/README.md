# Problem 1062: Partition Array Into Three Parts With Equal Sum

**Difficulty:** Easy  
**Topics:** Array, Greedy

---

## Problem Description

Given an array of integers `arr`, return `true` if we can partition the array into three 
non-empty parts with equal sums.

Formally, we can partition the array if we can find indexes `i + 1 

### Example
Example 2
**Input:** arr = [0,2,1,-6,6,7,9,-1,2,0,1]

**Output:** false

### Example
Example 3
**Input:** arr = [3,3,6,5,-2,2,5,1,-9,4]

**Output:** true

**Explanation:** 3 + 3 = 6 = 5 - 2 + 2 + 5 + 1 - 9 + 4

### Constraints
- `3 <= arr.length <= 5 * 10^4`

 - `-10^4 <= arr[i] <= 10^4`
