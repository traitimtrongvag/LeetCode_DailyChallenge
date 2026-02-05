# Problem 3651: Transformed Array

**Difficulty:** Easy  
**Topics:** Array, Simulation

---

## Problem Description

You are given an integer array `nums` that represents a circular array. Your task is to create a new array `result` of the 
same size, following these rules
For each index `i` (where `0 0`: Start at index `i` and move `nums[i]` steps to the 
right in the circular array. Set `result[i]` to the value of the index where you land.

 - If `nums[i] 

**Input:** nums = [3,-2,1,1]

**Output:** [1,1,1,3]

**Explanation:** - For `nums[0]` that is equal to 3, If we move 3 steps to right, we reach `nums[3]`. So `result[0]` should be 1.

 - For `nums[1]` that is equal to -2, If we move 2 steps to left, we reach `nums[3]`. So `result[1]` should be 1.

 - For `nums[2]` that is equal to 1, If we move 1 step to right, we reach `nums[3]`. So `result[2]` should be 1.

 - For `nums[3]` that is equal to 1, If we move 1 step to right, we reach `nums[0]`. So `result[3]` should be 3.

### Example
Example 2
**Input:** nums = [-1,4,-1]

**Output:** [-1,-1,4]

**Explanation:** - For `nums[0]` that is equal to -1, If we move 1 step to left, we reach `nums[2]`. So `result[0]` should be -1.

 - For `nums[1]` that is equal to 4, If we move 4 steps to right, we reach `nums[2]`. So `result[1]` should be -1.

 - For `nums[2]` that is equal to -1, If we move 1 step to left, we reach `nums[1]`. So `result[2]` should be 4.

### Constraints
- `1 <= nums.length <= 100`

 - `-100 <= nums[i] <= 100`
