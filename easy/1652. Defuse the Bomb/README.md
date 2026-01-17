# Problem 1755: Defuse the Bomb

**Difficulty:** Easy  
**Topics:** Array, Sliding Window

---

## Problem Description

You have a bomb to defuse, and your time is running out! Your informer will provide you with a 
circular array `code` of length of `n` and a key `k`.

To decrypt the code, you must replace every number. All the numbers are replaced 
simultaneously.

 - If `k > 0`, replace the `i^th` number with the sum of the 
next `k` numbers.

 - If `k 

### Example
Example 2
**Input:** code = [1,2,3,4], k = 0

**Output:** [0,0,0,0]

**Explanation:** When k is zero, the numbers are replaced by 0. 

### Example
Example 3
**Input:** code = [2,4,9,3], k = -2

**Output:** [12,5,6,13]

**Explanation:** The decrypted code is [3+9, 2+3, 4+2, 9+4]. Notice that the numbers wrap around again. If k is negative, the sum is of the 
previous numbers.

### Constraints
- `n == code.length`

 - `1 <= n <= 100`

 - `1 <= code[i] <= 100`

 - `-(n - 1) <= k <= n - 1`
