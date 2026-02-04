# Problem 3956: Trionic Array II

**Difficulty:** Hard  
**Topics:** Array, Dynamic Programming

---

## Problem Description

You are given an integer array nums` of length n`.

A trionic subarray is a contiguous subarray nums[l...r]` (with 0 nums[l...p]` is 
strictly increasing,

 nums[p...q]` is 
strictly decreasing,

 nums[q...r]` is 
strictly increasing.

Return the 
maximum sum of any trionic subarray in nums`.

### Example
Example 1
**Input:** nums = [0,-2,-1,-3,0,2,-1]

**Output:** -4

**Explanation:** Pick l = 1`, p = 2`, q = 3`, r = 5`
 nums[l...p] = nums[1...2] = [-2, -1]` is strictly increasing (-2 nums[p...q] = nums[2...3] = [-1, -3]` is strictly decreasing (-1 > -3`)

 nums[q...r] = nums[3...5] = [-3, 0, 2]` is strictly increasing (-3 Sum = `(-2) + (-1) + (-3) + 0 + 2 = -4`.

### Example
Example 2
**Input:** nums = [1,4,2,7]

**Output:** 14

**Explanation:** Pick l = 0`, p = 1`, q = 2`, r = 3`
 nums[l...p] = nums[0...1] = [1, 4]` is strictly increasing (1 nums[p...q] = nums[1...2] = [4, 2]` is strictly decreasing (4 > 2`).

 nums[q...r] = nums[2...3] = [2, 7]` is strictly increasing (2 Sum = `1 + 4 + 2 + 7 = 14`.

### Constraints
4 -10^9 It is guaranteed that at least one trionic subarray exists.
