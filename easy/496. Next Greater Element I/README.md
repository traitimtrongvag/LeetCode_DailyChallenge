# Problem 496: Next Greater Element I

**Difficulty:** Easy  
**Topics:** Array, Hash Table, Stack, Monotonic Stack

---

## Problem Description

The 
next greater element of some element `x` in an array is the 
first greater element that is 
to the right of `x` in the same array.

You are given two 
distinct 0-indexed integer arrays `nums1` and `nums2`, where `nums1` is a subset of `nums2`.

For each `0 4,2]. There is no next greater element, so the answer is -1.
- 1 is underlined in nums2 = [1,3,4,2]. The next greater element is 3.
- 2 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.

### Example
Example 2
**Input:** nums1 = [2,4], nums2 = [1,2,3,4]

**Output:** [3,-1]

**Explanation:** The next greater element for each value of nums1 is as follows
- 2 is underlined in nums2 = [1,2,3,4]. The next greater element is 3.
- 4 is underlined in nums2 = [1,2,3,4]. There is no next greater element, so the answer is -1.

### Constraints
- `1 <= nums1.length <= nums2.length <= 1000`

 - `0 <= nums1[i], nums2[i] <= 10^4`

 - All integers in `nums1` and `nums2` are 
unique.

 - All the integers of `nums1` also appear in `nums2`.

Follow up: Could you find an `O(nums1.length + nums2.length)` solution?
