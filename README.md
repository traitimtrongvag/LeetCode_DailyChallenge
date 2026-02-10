![Leet](.github/assets/Cover.jpg)

# LeetCode DailyChallenge

Welcome to this comprehensive collection of LeetCode problem solutions and algorithm study resources. This repository is designed to help you master data structures and algorithms through practical problem-solving, with solutions implemented in multiple programming languages including Java, C++, Python, and Rust.

## Table of Contents

- [About This Repository](#about-this-repository)
- [Repository Structure](#repository-structure)
- [Getting Started](#getting-started)
- [Core Algorithm Patterns](#core-algorithm-patterns)
  - [Binary Search](#binary-search)
  - [Two Pointers](#two-pointers)
  - [Sliding Window](#sliding-window)
  - [Dynamic Programming](#dynamic-programming)
  - [Greedy Algorithms](#greedy-algorithms)
  - [Backtracking](#backtracking)
  - [Graph Algorithms](#graph-algorithms)
  - [Tree Traversal](#tree-traversal)
  - [Sorting Algorithms](#sorting-algorithms)
  - [Bit Manipulation](#bit-manipulation)
- [Study Resources](#study-resources)
- [Problem-Solving Strategies](#problem-solving-strategies)
- [Time and Space Complexity Analysis](#time-and-space-complexity-analysis)
- [Tips and Tricks](#tips-and-tricks)
- [Interview Preparation](#interview-preparation)
- [Contributing](#contributing)
- [License](#license)

## About This Repository

This repository contains solutions to LeetCode problems, organized by difficulty level. The main goal is not just to solve problems, but to understand the underlying patterns and techniques that can be applied to a wide range of algorithmic challenges. Each solution is carefully crafted to be readable, efficient, and well-documented.

The problems are categorized into three difficulty levels:
- **Easy**: Fundamental problems to build your algorithmic foundation
- **Medium**: Intermediate problems that combine multiple concepts
- **Hard**: Advanced problems requiring deep understanding and optimization

## Repository Structure

```
.
├── easy/           # Easy difficulty problems
├── medium/         # Medium difficulty problems
└── hard/           # Hard difficulty problems
```

Each problem directory contains:
- Solution files in multiple languages (Java, C++, Python, Rust)
- Detailed explanations of the approach and complexity analysis

## Getting Started

If you're new to algorithmic problem-solving, here's a recommended learning path:

1. **Start with the fundamentals**: Begin with easy problems to build confidence and understand basic patterns
2. **Learn one pattern at a time**: Focus on mastering one algorithmic pattern before moving to the next
3. **Practice consistently**: Solve at least one problem daily to maintain momentum
4. **Review and reflect**: After solving a problem, review other solutions and understand different approaches
5. **Implement in multiple languages**: This helps you understand the core logic independent of language syntax

### Recommended Learning Order

1. Arrays and Strings
2. Hash Tables and Sets
3. Two Pointers and Sliding Window
4. Binary Search
5. Linked Lists
6. Stacks and Queues
7. Trees and Binary Search Trees
8. Recursion and Backtracking
9. Dynamic Programming
10. Graphs and Advanced Topics

## Core Algorithm Patterns

Understanding common algorithmic patterns is key to solving problems efficiently. Here are the most important patterns you'll encounter:

### Binary Search

Binary search is one of the most fundamental algorithms in computer science. It's used to efficiently search for an element in a sorted array by repeatedly dividing the search interval in half.

#### When to Use Binary Search

You should consider binary search when:
- The input is sorted (or can be sorted)
- You need to find a specific element or condition
- You're working with a continuous or discrete search space
- The problem involves finding minimum or maximum values with certain constraints

#### Basic Binary Search Template

```python
def binary_search(arr, target):
    left, right = 0, len(arr) - 1
    
    while left <= right:
        mid = left + (right - left) // 2
        
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1  # Element not found
```

#### Binary Search Variations

**1. Finding the First Occurrence**

When you need to find the leftmost position of a target value:

```python
def find_first(arr, target):
    left, right = 0, len(arr) - 1
    result = -1
    
    while left <= right:
        mid = left + (right - left) // 2
        
        if arr[mid] == target:
            result = mid
            right = mid - 1  # Continue searching in the left half
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return result
```

**2. Finding the Last Occurrence**

When you need to find the rightmost position of a target value:

```python
def find_last(arr, target):
    left, right = 0, len(arr) - 1
    result = -1
    
    while left <= right:
        mid = left + (right - left) // 2
        
        if arr[mid] == target:
            result = mid
            left = mid + 1  # Continue searching in the right half
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return result
```

**3. Search Insert Position**

Finding where to insert a value to maintain sorted order:

```python
def search_insert(arr, target):
    left, right = 0, len(arr) - 1
    
    while left <= right:
        mid = left + (right - left) // 2
        
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return left  # This is the insertion position
```

**4. Binary Search on Answer**

This technique involves binary searching on the answer space rather than the input array. It's particularly useful when you need to find the minimum or maximum value that satisfies certain conditions.

```python
def binary_search_on_answer(nums, condition_check):
    left, right = min_possible_value, max_possible_value
    result = -1
    
    while left <= right:
        mid = left + (right - left) // 2
        
        if condition_check(mid):
            result = mid
            right = mid - 1  # Try to find a smaller value
        else:
            left = mid + 1
    
    return result
```

#### Common Binary Search Problems

- **LeetCode 35**: Search Insert Position
- **LeetCode 69**: Sqrt(x)
- **LeetCode 153**: Find Minimum in Rotated Sorted Array
- **LeetCode 33**: Search in Rotated Sorted Array
- **LeetCode 34**: Find First and Last Position of Element in Sorted Array
- **LeetCode 4**: Median of Two Sorted Arrays (Hard)

#### Binary Search Tips and Tricks

1. **Avoid Integer Overflow**: Use `mid = left + (right - left) / 2` instead of `mid = (left + right) / 2`

2. **Choose the Right Boundary**: Decide whether to use `left <= right` or `left < right` based on your needs

3. **Handle Edge Cases**: Always consider empty arrays, single elements, and duplicates

4. **Think About Invariants**: Maintain clear invariants for your search space boundaries

5. **Rotated Arrays**: For rotated sorted arrays, determine which half is sorted first

### Two Pointers

The two pointers technique is a powerful approach for solving array and string problems. It involves using two pointers that traverse the data structure, typically from different positions or directions.

#### Types of Two Pointer Patterns

**1. Opposite Direction (Collision)**

Two pointers start from opposite ends and move toward each other:

```python
def reverse_string(s):
    left, right = 0, len(s) - 1
    
    while left < right:
        s[left], s[right] = s[right], s[left]
        left += 1
        right -= 1
```

**2. Same Direction (Fast and Slow)**

Both pointers start from the same end, but move at different speeds:

```python
def remove_duplicates(nums):
    if not nums:
        return 0
    
    slow = 0
    for fast in range(1, len(nums)):
        if nums[fast] != nums[slow]:
            slow += 1
            nums[slow] = nums[fast]
    
    return slow + 1
```

**3. Sliding Window Variant**

A variation where pointers define a window:

```python
def two_sum_sorted(numbers, target):
    left, right = 0, len(numbers) - 1
    
    while left < right:
        current_sum = numbers[left] + numbers[right]
        
        if current_sum == target:
            return [left + 1, right + 1]
        elif current_sum < target:
            left += 1
        else:
            right -= 1
    
    return []
```

#### When to Use Two Pointers

- Working with sorted arrays or linked lists
- Need to find pairs that satisfy certain conditions
- Need to partition or rearrange elements
- Problems involving palindromes
- Merging sorted arrays or lists

#### Common Two Pointer Problems

- **LeetCode 1**: Two Sum (with sorting modification)
- **LeetCode 15**: 3Sum
- **LeetCode 16**: 3Sum Closest
- **LeetCode 18**: 4Sum
- **LeetCode 11**: Container With Most Water
- **LeetCode 167**: Two Sum II - Input Array Is Sorted
- **LeetCode 125**: Valid Palindrome
- **LeetCode 283**: Move Zeroes

#### Two Pointer Tips

1. **Sort First**: Many two-pointer problems become easier after sorting
2. **Watch for Duplicates**: When finding unique combinations, skip duplicates
3. **Consider Edge Cases**: Empty arrays, single elements, all same elements
4. **Time Complexity**: Usually O(n) for single pass, O(n²) for nested scenarios

### Sliding Window

The sliding window technique is used to perform operations on a specific window size of array or string. The window can be fixed-size or dynamic.

#### Fixed-Size Sliding Window

When the window size is constant:

```python
def max_sum_subarray(arr, k):
    if len(arr) < k:
        return -1
    
    # Calculate sum of first window
    window_sum = sum(arr[:k])
    max_sum = window_sum
    
    # Slide the window
    for i in range(k, len(arr)):
        window_sum = window_sum - arr[i - k] + arr[i]
        max_sum = max(max_sum, window_sum)
    
    return max_sum
```

#### Dynamic Sliding Window

When the window size can grow or shrink:

```python
def longest_substring_k_distinct(s, k):
    if not s or k == 0:
        return 0
    
    char_count = {}
    left = 0
    max_length = 0
    
    for right in range(len(s)):
        # Expand window
        char_count[s[right]] = char_count.get(s[right], 0) + 1
        
        # Shrink window if needed
        while len(char_count) > k:
            char_count[s[left]] -= 1
            if char_count[s[left]] == 0:
                del char_count[s[left]]
            left += 1
        
        max_length = max(max_length, right - left + 1)
    
    return max_length
```

#### Sliding Window Template

Here's a general template for dynamic sliding window problems:

```python
def sliding_window_template(s):
    # Initialize window boundaries and data structures
    left = 0
    result = 0
    window_data = {}
    
    for right in range(len(s)):
        # Add current element to window
        # Update window data structures
        
        # Shrink window while condition is violated
        while condition_violated():
            # Remove left element from window
            # Update window data structures
            left += 1
        
        # Update result if needed
        result = max(result, right - left + 1)
    
    return result
```

#### Common Sliding Window Problems

- **LeetCode 3**: Longest Substring Without Repeating Characters
- **LeetCode 76**: Minimum Window Substring (Hard)
- **LeetCode 209**: Minimum Size Subarray Sum
- **LeetCode 424**: Longest Repeating Character Replacement
- **LeetCode 438**: Find All Anagrams in a String
- **LeetCode 567**: Permutation in String

#### Sliding Window Tips

1. **Hash Maps are Your Friend**: Use hash maps to track element frequencies
2. **Two Conditions**: Typically need conditions for expanding and shrinking
3. **Start Simple**: Begin with fixed-size windows before tackling dynamic ones
4. **Track Both Boundaries**: Always keep track of left and right pointers
5. **Update Carefully**: Make sure to update your data structures when expanding/shrinking

### Dynamic Programming

Dynamic programming (DP) is a method for solving complex problems by breaking them down into simpler subproblems. It's one of the most powerful algorithmic techniques but also one of the most challenging to master.

#### Key Characteristics of DP Problems

1. **Optimal Substructure**: The optimal solution can be constructed from optimal solutions of subproblems
2. **Overlapping Subproblems**: The same subproblems are solved multiple times

#### Approaches to Dynamic Programming

**1. Top-Down (Memoization)**

Start with the original problem and recursively break it down, caching results:

```python
def fib_memoization(n, memo=None):
    if memo is None:
        memo = {}
    
    if n in memo:
        return memo[n]
    
    if n <= 1:
        return n
    
    memo[n] = fib_memoization(n - 1, memo) + fib_memoization(n - 2, memo)
    return memo[n]
```

**2. Bottom-Up (Tabulation)**

Start with the smallest subproblems and build up to the original problem:

```python
def fib_tabulation(n):
    if n <= 1:
        return n
    
    dp = [0] * (n + 1)
    dp[1] = 1
    
    for i in range(2, n + 1):
        dp[i] = dp[i - 1] + dp[i - 2]
    
    return dp[n]
```

**3. Space-Optimized Bottom-Up**

When you only need the last few states:

```python
def fib_optimized(n):
    if n <= 1:
        return n
    
    prev2, prev1 = 0, 1
    
    for i in range(2, n + 1):
        current = prev1 + prev2
        prev2 = prev1
        prev1 = current
    
    return prev1
```

#### Common DP Patterns

**1. Linear DP (1D Array)**

Problems where the state depends on previous elements in a sequence:

```python
def climbing_stairs(n):
    if n <= 2:
        return n
    
    dp = [0] * (n + 1)
    dp[1] = 1
    dp[2] = 2
    
    for i in range(3, n + 1):
        dp[i] = dp[i - 1] + dp[i - 2]
    
    return dp[n]
```

**2. Grid DP (2D Array)**

Problems on grids or involving two sequences:

```python
def unique_paths(m, n):
    dp = [[1] * n for _ in range(m)]
    
    for i in range(1, m):
        for j in range(1, n):
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
    
    return dp[m - 1][n - 1]
```

**3. Subsequence DP**

Problems involving subsequences or substrings:

```python
def longest_common_subsequence(text1, text2):
    m, n = len(text1), len(text2)
    dp = [[0] * (n + 1) for _ in range(m + 1)]
    
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if text1[i - 1] == text2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1] + 1
            else:
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])
    
    return dp[m][n]
```

**4. Knapsack Pattern**

Problems involving selection with constraints:

```python
def knapsack_01(weights, values, capacity):
    n = len(weights)
    dp = [[0] * (capacity + 1) for _ in range(n + 1)]
    
    for i in range(1, n + 1):
        for w in range(capacity + 1):
            if weights[i - 1] <= w:
                dp[i][w] = max(
                    dp[i - 1][w],
                    values[i - 1] + dp[i - 1][w - weights[i - 1]]
                )
            else:
                dp[i][w] = dp[i - 1][w]
    
    return dp[n][capacity]
```

**5. State Machine DP**

Problems with different states and transitions:

```python
def max_profit_stock(prices):
    if not prices:
        return 0
    
    # States: hold stock, sold stock, cooldown
    hold = -prices[0]
    sold = 0
    cooldown = 0
    
    for i in range(1, len(prices)):
        prev_hold = hold
        prev_sold = sold
        
        hold = max(hold, cooldown - prices[i])
        sold = prev_hold + prices[i]
        cooldown = max(cooldown, prev_sold)
    
    return max(sold, cooldown)
```

#### Steps to Solve DP Problems

1. **Identify if it's a DP problem**: Look for optimal substructure and overlapping subproblems
2. **Define the state**: What information do we need to solve subproblems?
3. **Find the recurrence relation**: How does the current state relate to previous states?
4. **Determine base cases**: What are the simplest cases we can solve directly?
5. **Decide on implementation**: Top-down or bottom-up?
6. **Optimize space**: Can we reduce the space complexity?

#### Common DP Problems

**Easy:**
- **LeetCode 70**: Climbing Stairs
- **LeetCode 121**: Best Time to Buy and Sell Stock
- **LeetCode 53**: Maximum Subarray
- **LeetCode 198**: House Robber

**Medium:**
- **LeetCode 5**: Longest Palindromic Substring
- **LeetCode 55**: Jump Game
- **LeetCode 62**: Unique Paths
- **LeetCode 322**: Coin Change
- **LeetCode 300**: Longest Increasing Subsequence
- **LeetCode 139**: Word Break

**Hard:**
- **LeetCode 72**: Edit Distance
- **LeetCode 10**: Regular Expression Matching
- **LeetCode 123**: Best Time to Buy and Sell Stock III
- **LeetCode 188**: Best Time to Buy and Sell Stock IV

#### DP Tips and Common Pitfalls

1. **Start Small**: Practice with classic problems like Fibonacci before tackling complex ones
2. **Draw It Out**: Visualize the DP table with small examples
3. **Check Dimensions**: Make sure your DP array dimensions match your state definition
4. **Initialize Carefully**: Base cases are crucial for correctness
5. **Watch for Integer Overflow**: Some DP problems involve large numbers
6. **Consider Space Optimization**: Often you can reduce from 2D to 1D or even constant space
7. **Avoid Recomputation**: That's the whole point of DP!

### Greedy Algorithms

Greedy algorithms make locally optimal choices at each step, hoping to find a global optimum. They're simpler than dynamic programming but only work for specific types of problems.

#### When to Use Greedy

A greedy approach works when:
- The problem has the greedy-choice property (local optimum leads to global optimum)
- The problem has optimal substructure
- You can prove that the greedy choice is safe

#### Common Greedy Patterns

**1. Activity Selection / Interval Scheduling**

```python
def max_non_overlapping_intervals(intervals):
    if not intervals:
        return 0
    
    # Sort by end time
    intervals.sort(key=lambda x: x[1])
    
    count = 1
    current_end = intervals[0][1]
    
    for i in range(1, len(intervals)):
        if intervals[i][0] >= current_end:
            count += 1
            current_end = intervals[i][1]
    
    return count
```

**2. Huffman Coding / Merge Cost Minimization**

```python
import heapq

def min_cost_to_merge(stones):
    heapq.heapify(stones)
    total_cost = 0
    
    while len(stones) > 1:
        first = heapq.heappop(stones)
        second = heapq.heappop(stones)
        
        cost = first + second
        total_cost += cost
        
        heapq.heappush(stones, cost)
    
    return total_cost
```

**3. Fractional Knapsack**

```python
def fractional_knapsack(items, capacity):
    # Items are (value, weight) tuples
    # Sort by value per weight ratio
    items.sort(key=lambda x: x[0] / x[1], reverse=True)
    
    total_value = 0
    remaining_capacity = capacity
    
    for value, weight in items:
        if weight <= remaining_capacity:
            total_value += value
            remaining_capacity -= weight
        else:
            # Take fraction
            fraction = remaining_capacity / weight
            total_value += value * fraction
            break
    
    return total_value
```

**4. Jump Game**

```python
def can_jump(nums):
    max_reach = 0
    
    for i in range(len(nums)):
        if i > max_reach:
            return False
        max_reach = max(max_reach, i + nums[i])
        if max_reach >= len(nums) - 1:
            return True
    
    return True
```

#### Common Greedy Problems

- **LeetCode 55**: Jump Game
- **LeetCode 45**: Jump Game II
- **LeetCode 122**: Best Time to Buy and Sell Stock II
- **LeetCode 435**: Non-overlapping Intervals
- **LeetCode 452**: Minimum Number of Arrows to Burst Balloons
- **LeetCode 134**: Gas Station
- **LeetCode 135**: Candy

#### Greedy vs Dynamic Programming

How do you know if a problem requires greedy or DP?

**Use Greedy when:**
- You can prove the greedy choice is optimal
- The problem has a natural sorting or priority order
- Local decisions don't affect future choices significantly

**Use DP when:**
- The greedy approach gives suboptimal results
- You need to explore multiple possibilities
- Overlapping subproblems exist

**Example**: Coin change
- **Greedy**: Works for certain coin systems (e.g., US coins)
- **DP**: Required for arbitrary coin denominations

#### Greedy Tips

1. **Prove Correctness**: Always try to prove (or convince yourself) that greedy works
2. **Sort First**: Many greedy algorithms start with sorting
3. **Use Priority Queues**: For problems involving selection or merging
4. **Think Counterexamples**: If you're unsure, try to find a case where greedy fails
5. **Look for Patterns**: Interval problems, scheduling, and resource allocation often use greedy

### Backtracking

Backtracking is an algorithmic technique for solving problems by trying to build a solution incrementally, abandoning solutions ("backtracking") as soon as it determines that the solution cannot possibly be completed.

#### Backtracking Template

```python
def backtrack(path, choices):
    if is_solution(path):
        result.append(path.copy())
        return
    
    for choice in choices:
        # Make choice
        path.append(choice)
        
        # Explore with this choice
        backtrack(path, get_next_choices(choice))
        
        # Undo choice (backtrack)
        path.pop()
```

#### Common Backtracking Patterns

**1. Permutations**

Generate all permutations of a set:

```python
def permute(nums):
    result = []
    
    def backtrack(path, remaining):
        if not remaining:
            result.append(path[:])
            return
        
        for i in range(len(remaining)):
            # Choose
            path.append(remaining[i])
            
            # Explore
            backtrack(path, remaining[:i] + remaining[i+1:])
            
            # Unchoose
            path.pop()
    
    backtrack([], nums)
    return result
```

**2. Combinations**

Generate all combinations of size k:

```python
def combine(n, k):
    result = []
    
    def backtrack(start, path):
        if len(path) == k:
            result.append(path[:])
            return
        
        for i in range(start, n + 1):
            path.append(i)
            backtrack(i + 1, path)
            path.pop()
    
    backtrack(1, [])
    return result
```

**3. Subsets**

Generate all possible subsets:

```python
def subsets(nums):
    result = []
    
    def backtrack(start, path):
        result.append(path[:])
        
        for i in range(start, len(nums)):
            path.append(nums[i])
            backtrack(i + 1, path)
            path.pop()
    
    backtrack(0, [])
    return result
```

**4. N-Queens**

Classic backtracking problem:

```python
def solve_n_queens(n):
    result = []
    board = [['.'] * n for _ in range(n)]
    
    def is_safe(row, col):
        # Check column
        for i in range(row):
            if board[i][col] == 'Q':
                return False
        
        # Check diagonal
        i, j = row - 1, col - 1
        while i >= 0 and j >= 0:
            if board[i][j] == 'Q':
                return False
            i -= 1
            j -= 1
        
        # Check anti-diagonal
        i, j = row - 1, col + 1
        while i >= 0 and j < n:
            if board[i][j] == 'Q':
                return False
            i -= 1
            j += 1
        
        return True
    
    def backtrack(row):
        if row == n:
            result.append([''.join(row) for row in board])
            return
        
        for col in range(n):
            if is_safe(row, col):
                board[row][col] = 'Q'
                backtrack(row + 1)
                board[row][col] = '.'
    
    backtrack(0)
    return result
```

**5. Sudoku Solver**

```python
def solve_sudoku(board):
    def is_valid(row, col, num):
        # Check row
        if num in board[row]:
            return False
        
        # Check column
        if num in [board[i][col] for i in range(9)]:
            return False
        
        # Check 3x3 box
        box_row, box_col = 3 * (row // 3), 3 * (col // 3)
        for i in range(box_row, box_row + 3):
            for j in range(box_col, box_col + 3):
                if board[i][j] == num:
                    return False
        
        return True
    
    def backtrack():
        for i in range(9):
            for j in range(9):
                if board[i][j] == '.':
                    for num in '123456789':
                        if is_valid(i, j, num):
                            board[i][j] = num
                            
                            if backtrack():
                                return True
                            
                            board[i][j] = '.'
                    
                    return False
        return True
    
    backtrack()
```

#### Optimization Techniques

**1. Pruning**

Cut off branches early when they can't lead to a solution:

```python
def combine_sum(candidates, target):
    result = []
    candidates.sort()  # Sort for pruning
    
    def backtrack(start, path, total):
        if total == target:
            result.append(path[:])
            return
        
        for i in range(start, len(candidates)):
            # Prune: if current number is too large, stop
            if total + candidates[i] > target:
                break
            
            path.append(candidates[i])
            backtrack(i, path, total + candidates[i])
            path.pop()
    
    backtrack(0, [], 0)
    return result
```

**2. Avoiding Duplicates**

Skip duplicate elements to avoid generating duplicate solutions:

```python
def subsets_with_dup(nums):
    result = []
    nums.sort()  # Sort to group duplicates
    
    def backtrack(start, path):
        result.append(path[:])
        
        for i in range(start, len(nums)):
            # Skip duplicates
            if i > start and nums[i] == nums[i - 1]:
                continue
            
            path.append(nums[i])
            backtrack(i + 1, path)
            path.pop()
    
    backtrack(0, [])
    return result
```

#### Common Backtracking Problems

- **LeetCode 46**: Permutations
- **LeetCode 47**: Permutations II
- **LeetCode 77**: Combinations
- **LeetCode 78**: Subsets
- **LeetCode 90**: Subsets II
- **LeetCode 39**: Combination Sum
- **LeetCode 40**: Combination Sum II
- **LeetCode 51**: N-Queens
- **LeetCode 37**: Sudoku Solver
- **LeetCode 79**: Word Search
- **LeetCode 131**: Palindrome Partitioning

#### Backtracking Tips

1. **State Representation**: Clearly define what constitutes your current state
2. **Base Case**: Know when you've found a complete solution
3. **Choice Making**: Understand what choices you have at each step
4. **Pruning**: Add conditions to avoid exploring invalid paths
5. **Restoration**: Always undo changes when backtracking
6. **Avoid Duplicates**: Sort and skip duplicates when needed
7. **Time Complexity**: Be aware that backtracking can be exponential

### Graph Algorithms

Graph algorithms are essential for solving problems involving networks, relationships, and connections. Understanding graph traversal and various graph algorithms is crucial for technical interviews.

#### Graph Representations

**1. Adjacency List**

Most common representation, efficient for sparse graphs:

```python
# Dictionary of lists
graph = {
    0: [1, 2],
    1: [0, 3, 4],
    2: [0, 4],
    3: [1],
    4: [1, 2]
}

# Or list of lists
graph = [
    [1, 2],      # neighbors of node 0
    [0, 3, 4],   # neighbors of node 1
    [0, 4],      # neighbors of node 2
    [1],         # neighbors of node 3
    [1, 2]       # neighbors of node 4
]
```

**2. Adjacency Matrix**

Good for dense graphs or when you need O(1) edge lookup:

```python
# 2D array where matrix[i][j] = 1 if edge exists
graph = [
    [0, 1, 1, 0, 0],
    [1, 0, 0, 1, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 0, 0]
]
```

**3. Edge List**

Simple representation, useful for some algorithms:

```python
edges = [(0, 1), (0, 2), (1, 3), (1, 4), (2, 4)]
```

#### Depth-First Search (DFS)

DFS explores as far as possible along each branch before backtracking.

**Recursive DFS:**

```python
def dfs_recursive(graph, node, visited=None):
    if visited is None:
        visited = set()
    
    visited.add(node)
    print(node)  # Process node
    
    for neighbor in graph[node]:
        if neighbor not in visited:
            dfs_recursive(graph, neighbor, visited)
    
    return visited
```

**Iterative DFS:**

```python
def dfs_iterative(graph, start):
    visited = set()
    stack = [start]
    
    while stack:
        node = stack.pop()
        
        if node not in visited:
            visited.add(node)
            print(node)  # Process node
            
            # Add neighbors in reverse order for same traversal as recursive
            for neighbor in reversed(graph[node]):
                if neighbor not in visited:
                    stack.append(neighbor)
    
    return visited
```

**DFS for Cycle Detection:**

```python
def has_cycle_directed(graph):
    WHITE, GRAY, BLACK = 0, 1, 2
    color = {node: WHITE for node in graph}
    
    def dfs(node):
        if color[node] == GRAY:
            return True  # Back edge found
        if color[node] == BLACK:
            return False  # Already processed
        
        color[node] = GRAY
        
        for neighbor in graph.get(node, []):
            if dfs(neighbor):
                return True
        
        color[node] = BLACK
        return False
    
    for node in graph:
        if color[node] == WHITE:
            if dfs(node):
                return True
    
    return False
```

#### Breadth-First Search (BFS)

BFS explores all neighbors at the current depth before moving to nodes at the next depth level.

**Basic BFS:**

```python
from collections import deque

def bfs(graph, start):
    visited = set([start])
    queue = deque([start])
    
    while queue:
        node = queue.popleft()
        print(node)  # Process node
        
        for neighbor in graph[node]:
            if neighbor not in visited:
                visited.add(neighbor)
                queue.append(neighbor)
    
    return visited
```

**BFS for Shortest Path (Unweighted):**

```python
def shortest_path_bfs(graph, start, end):
    if start == end:
        return [start]
    
    visited = {start}
    queue = deque([(start, [start])])
    
    while queue:
        node, path = queue.popleft()
        
        for neighbor in graph[node]:
            if neighbor not in visited:
                new_path = path + [neighbor]
                
                if neighbor == end:
                    return new_path
                
                visited.add(neighbor)
                queue.append((neighbor, new_path))
    
    return []  # No path found
```

**Level-Order Traversal:**

```python
def level_order_traversal(graph, start):
    visited = {start}
    queue = deque([start])
    levels = []
    
    while queue:
        level_size = len(queue)
        current_level = []
        
        for _ in range(level_size):
            node = queue.popleft()
            current_level.append(node)
            
            for neighbor in graph[node]:
                if neighbor not in visited:
                    visited.add(neighbor)
                    queue.append(neighbor)
        
        levels.append(current_level)
    
    return levels
```

#### Topological Sort

Ordering of vertices in a directed acyclic graph (DAG) where for every directed edge u -> v, u comes before v.

**DFS-based Topological Sort:**

```python
def topological_sort_dfs(graph):
    visited = set()
    stack = []
    
    def dfs(node):
        visited.add(node)
        
        for neighbor in graph.get(node, []):
            if neighbor not in visited:
                dfs(neighbor)
        
        stack.append(node)
    
    for node in graph:
        if node not in visited:
            dfs(node)
    
    return stack[::-1]  # Reverse for topological order
```

**Kahn's Algorithm (BFS-based):**

```python
from collections import deque

def topological_sort_kahn(graph, num_nodes):
    # Calculate in-degrees
    in_degree = {i: 0 for i in range(num_nodes)}
    for node in graph:
        for neighbor in graph[node]:
            in_degree[neighbor] += 1
    
    # Start with nodes having no incoming edges
    queue = deque([node for node in in_degree if in_degree[node] == 0])
    result = []
    
    while queue:
        node = queue.popleft()
        result.append(node)
        
        for neighbor in graph.get(node, []):
            in_degree[neighbor] -= 1
            if in_degree[neighbor] == 0:
                queue.append(neighbor)
    
    # If result doesn't contain all nodes, there's a cycle
    return result if len(result) == num_nodes else []
```

#### Dijkstra's Algorithm

Find shortest path from source to all other vertices in weighted graph (non-negative weights).

```python
import heapq

def dijkstra(graph, start):
    # graph[node] = [(neighbor, weight), ...]
    distances = {node: float('infinity') for node in graph}
    distances[start] = 0
    
    pq = [(0, start)]  # (distance, node)
    visited = set()
    
    while pq:
        current_dist, current_node = heapq.heappop(pq)
        
        if current_node in visited:
            continue
        
        visited.add(current_node)
        
        for neighbor, weight in graph[current_node]:
            distance = current_dist + weight
            
            if distance < distances[neighbor]:
                distances[neighbor] = distance
                heapq.heappush(pq, (distance, neighbor))
    
    return distances
```

#### Union Find (Disjoint Set)

Efficient data structure for tracking connected components.

```python
class UnionFind:
    def __init__(self, n):
        self.parent = list(range(n))
        self.rank = [0] * n
        self.count = n  # Number of connected components
    
    def find(self, x):
        # Path compression
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]
    
    def union(self, x, y):
        # Union by rank
        root_x = self.find(x)
        root_y = self.find(y)
        
        if root_x == root_y:
            return False
        
        if self.rank[root_x] < self.rank[root_y]:
            self.parent[root_x] = root_y
        elif self.rank[root_x] > self.rank[root_y]:
            self.parent[root_y] = root_x
        else:
            self.parent[root_y] = root_x
            self.rank[root_x] += 1
        
        self.count -= 1
        return True
    
    def connected(self, x, y):
        return self.find(x) == self.find(y)
```

#### Minimum Spanning Tree

**Kruskal's Algorithm:**

```python
def kruskal(edges, num_nodes):
    # edges = [(weight, u, v), ...]
    edges.sort()  # Sort by weight
    
    uf = UnionFind(num_nodes)
    mst = []
    total_weight = 0
    
    for weight, u, v in edges:
        if uf.union(u, v):
            mst.append((u, v, weight))
            total_weight += weight
            
            if len(mst) == num_nodes - 1:
                break
    
    return mst, total_weight
```

#### Common Graph Problems

**BFS/DFS:**
- **LeetCode 200**: Number of Islands
- **LeetCode 133**: Clone Graph
- **LeetCode 994**: Rotting Oranges
- **LeetCode 542**: 01 Matrix

**Topological Sort:**
- **LeetCode 207**: Course Schedule
- **LeetCode 210**: Course Schedule II
- **LeetCode 269**: Alien Dictionary (Premium)

**Shortest Path:**
- **LeetCode 743**: Network Delay Time
- **LeetCode 787**: Cheapest Flights Within K Stops

**Union Find:**
- **LeetCode 547**: Number of Provinces
- **LeetCode 684**: Redundant Connection
- **LeetCode 721**: Accounts Merge

#### Graph Algorithm Tips

1. **Choose Right Representation**: Adjacency list for sparse, matrix for dense graphs
2. **Mark Visited**: Always track visited nodes to avoid infinite loops
3. **BFS for Shortest Path**: Use BFS for unweighted shortest paths
4. **DFS for Exploration**: Use DFS for exhaustive exploration and backtracking
5. **Topological Sort for Dependencies**: Great for scheduling and ordering problems
6. **Union Find for Connectivity**: Efficient for dynamic connectivity queries
7. **Consider Edge Cases**: Disconnected graphs, self-loops, multiple edges

### Tree Traversal

Trees are hierarchical data structures, and understanding different traversal methods is fundamental to solving tree problems.

#### Binary Tree Structure

```python
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
```

#### Depth-First Traversals

**1. Inorder Traversal (Left -> Root -> Right)**

Gives sorted order for BST:

```python
def inorder_recursive(root):
    result = []
    
    def traverse(node):
        if not node:
            return
        traverse(node.left)
        result.append(node.val)
        traverse(node.right)
    
    traverse(root)
    return result

def inorder_iterative(root):
    result = []
    stack = []
    current = root
    
    while current or stack:
        while current:
            stack.append(current)
            current = current.left
        
        current = stack.pop()
        result.append(current.val)
        current = current.right
    
    return result
```

**2. Preorder Traversal (Root -> Left -> Right)**

Used for creating copy of tree:

```python
def preorder_recursive(root):
    result = []
    
    def traverse(node):
        if not node:
            return
        result.append(node.val)
        traverse(node.left)
        traverse(node.right)
    
    traverse(root)
    return result

def preorder_iterative(root):
    if not root:
        return []
    
    result = []
    stack = [root]
    
    while stack:
        node = stack.pop()
        result.append(node.val)
        
        if node.right:
            stack.append(node.right)
        if node.left:
            stack.append(node.left)
    
    return result
```

**3. Postorder Traversal (Left -> Right -> Root)**

Used for deletion:

```python
def postorder_recursive(root):
    result = []
    
    def traverse(node):
        if not node:
            return
        traverse(node.left)
        traverse(node.right)
        result.append(node.val)
    
    traverse(root)
    return result

def postorder_iterative(root):
    if not root:
        return []
    
    result = []
    stack = [root]
    
    while stack:
        node = stack.pop()
        result.append(node.val)
        
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    
    return result[::-1]  # Reverse
```

#### Breadth-First Traversal (Level Order)

```python
from collections import deque

def level_order(root):
    if not root:
        return []
    
    result = []
    queue = deque([root])
    
    while queue:
        level_size = len(queue)
        current_level = []
        
        for _ in range(level_size):
            node = queue.popleft()
            current_level.append(node.val)
            
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)
        
        result.append(current_level)
    
    return result
```

#### Common Tree Patterns

**1. Tree Height/Depth**

```python
def max_depth(root):
    if not root:
        return 0
    return 1 + max(max_depth(root.left), max_depth(root.right))

def min_depth(root):
    if not root:
        return 0
    if not root.left and not root.right:
        return 1
    if not root.left:
        return 1 + min_depth(root.right)
    if not root.right:
        return 1 + min_depth(root.left)
    return 1 + min(min_depth(root.left), min_depth(root.right))
```

**2. Path Sum**

```python
def has_path_sum(root, target_sum):
    if not root:
        return False
    
    if not root.left and not root.right:
        return root.val == target_sum
    
    return (has_path_sum(root.left, target_sum - root.val) or
            has_path_sum(root.right, target_sum - root.val))

def path_sum_all(root, target_sum):
    result = []
    
    def dfs(node, current_path, remaining):
        if not node:
            return
        
        current_path.append(node.val)
        
        if not node.left and not node.right and remaining == node.val:
            result.append(current_path[:])
        
        dfs(node.left, current_path, remaining - node.val)
        dfs(node.right, current_path, remaining - node.val)
        
        current_path.pop()
    
    dfs(root, [], target_sum)
    return result
```

**3. Lowest Common Ancestor**

```python
def lowest_common_ancestor(root, p, q):
    if not root or root == p or root == q:
        return root
    
    left = lowest_common_ancestor(root.left, p, q)
    right = lowest_common_ancestor(root.right, p, q)
    
    if left and right:
        return root
    
    return left if left else right
```

**4. Binary Search Tree Operations**

```python
def search_bst(root, val):
    if not root or root.val == val:
        return root
    
    if val < root.val:
        return search_bst(root.left, val)
    return search_bst(root.right, val)

def insert_bst(root, val):
    if not root:
        return TreeNode(val)
    
    if val < root.val:
        root.left = insert_bst(root.left, val)
    else:
        root.right = insert_bst(root.right, val)
    
    return root

def is_valid_bst(root, min_val=float('-inf'), max_val=float('inf')):
    if not root:
        return True
    
    if root.val <= min_val or root.val >= max_val:
        return False
    
    return (is_valid_bst(root.left, min_val, root.val) and
            is_valid_bst(root.right, root.val, max_val))
```

**5. Serialize and Deserialize**

```python
def serialize(root):
    if not root:
        return "null"
    
    return f"{root.val},{serialize(root.left)},{serialize(root.right)}"

def deserialize(data):
    def helper(values):
        val = next(values)
        if val == "null":
            return None
        
        node = TreeNode(int(val))
        node.left = helper(values)
        node.right = helper(values)
        return node
    
    return helper(iter(data.split(',')))
```

#### Common Tree Problems

**Easy:**
- **LeetCode 94**: Binary Tree Inorder Traversal
- **LeetCode 100**: Same Tree
- **LeetCode 101**: Symmetric Tree
- **LeetCode 104**: Maximum Depth of Binary Tree
- **LeetCode 110**: Balanced Binary Tree

**Medium:**
- **LeetCode 102**: Binary Tree Level Order Traversal
- **LeetCode 103**: Binary Tree Zigzag Level Order Traversal
- **LeetCode 105**: Construct Binary Tree from Preorder and Inorder
- **LeetCode 108**: Convert Sorted Array to Binary Search Tree
- **LeetCode 113**: Path Sum II
- **LeetCode 114**: Flatten Binary Tree to Linked List
- **LeetCode 236**: Lowest Common Ancestor

**Hard:**
- **LeetCode 124**: Binary Tree Maximum Path Sum
- **LeetCode 297**: Serialize and Deserialize Binary Tree

#### Tree Tips

1. **Recursion is Natural**: Most tree problems have elegant recursive solutions
2. **Base Cases**: Always handle null nodes
3. **Think About Return Values**: What should each recursive call return?
4. **Level Order = BFS**: Use a queue for level-order traversal
5. **Inorder for BST**: Remember that inorder gives sorted sequence for BST
6. **Path Tracking**: Use backtracking when you need to track paths
7. **Height vs Depth**: Height is distance to deepest leaf; depth is distance from root

### Sorting Algorithms

Understanding sorting algorithms is fundamental, even though you'll typically use built-in sort functions. Knowledge of different sorting techniques helps in optimization and understanding time-space tradeoffs.

#### Comparison-Based Sorting

**1. Quick Sort**

Average O(n log n), worst O(n²), in-place, not stable:

```python
def quick_sort(arr):
    if len(arr) <= 1:
        return arr
    
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    
    return quick_sort(left) + middle + quick_sort(right)

def quick_sort_inplace(arr, low=0, high=None):
    if high is None:
        high = len(arr) - 1
    
    if low < high:
        pi = partition(arr, low, high)
        quick_sort_inplace(arr, low, pi - 1)
        quick_sort_inplace(arr, pi + 1, high)
    
    return arr

def partition(arr, low, high):
    pivot = arr[high]
    i = low - 1
    
    for j in range(low, high):
        if arr[j] <= pivot:
            i += 1
            arr[i], arr[j] = arr[j], arr[i]
    
    arr[i + 1], arr[high] = arr[high], arr[i + 1]
    return i + 1
```

**2. Merge Sort**

O(n log n), stable, requires O(n) extra space:

```python
def merge_sort(arr):
    if len(arr) <= 1:
        return arr
    
    mid = len(arr) // 2
    left = merge_sort(arr[:mid])
    right = merge_sort(arr[mid:])
    
    return merge(left, right)

def merge(left, right):
    result = []
    i = j = 0
    
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    
    result.extend(left[i:])
    result.extend(right[j:])
    return result
```

**3. Heap Sort**

O(n log n), in-place, not stable:

```python
def heap_sort(arr):
    n = len(arr)
    
    # Build max heap
    for i in range(n // 2 - 1, -1, -1):
        heapify(arr, n, i)
    
    # Extract elements one by one
    for i in range(n - 1, 0, -1):
        arr[0], arr[i] = arr[i], arr[0]
        heapify(arr, i, 0)
    
    return arr

def heapify(arr, n, i):
    largest = i
    left = 2 * i + 1
    right = 2 * i + 2
    
    if left < n and arr[left] > arr[largest]:
        largest = left
    
    if right < n and arr[right] > arr[largest]:
        largest = right
    
    if largest != i:
        arr[i], arr[largest] = arr[largest], arr[i]
        heapify(arr, n, largest)
```

#### Non-Comparison Sorts

**1. Counting Sort**

O(n + k) where k is range of input, stable:

```python
def counting_sort(arr):
    if not arr:
        return arr
    
    min_val = min(arr)
    max_val = max(arr)
    range_size = max_val - min_val + 1
    
    count = [0] * range_size
    output = [0] * len(arr)
    
    # Count occurrences
    for num in arr:
        count[num - min_val] += 1
    
    # Cumulative count
    for i in range(1, len(count)):
        count[i] += count[i - 1]
    
    # Build output array
    for i in range(len(arr) - 1, -1, -1):
        num = arr[i]
        index = count[num - min_val] - 1
        output[index] = num
        count[num - min_val] -= 1
    
    return output
```

**2. Radix Sort**

O(d * (n + k)) where d is number of digits:

```python
def radix_sort(arr):
    if not arr:
        return arr
    
    max_val = max(arr)
    exp = 1
    
    while max_val // exp > 0:
        counting_sort_by_digit(arr, exp)
        exp *= 10
    
    return arr

def counting_sort_by_digit(arr, exp):
    n = len(arr)
    output = [0] * n
    count = [0] * 10
    
    for num in arr:
        index = (num // exp) % 10
        count[index] += 1
    
    for i in range(1, 10):
        count[i] += count[i - 1]
    
    for i in range(n - 1, -1, -1):
        index = (arr[i] // exp) % 10
        output[count[index] - 1] = arr[i]
        count[index] -= 1
    
    for i in range(n):
        arr[i] = output[i]
```

**3. Bucket Sort**

Average O(n + k), good for uniformly distributed data:

```python
def bucket_sort(arr):
    if not arr:
        return arr
    
    # Create buckets
    bucket_count = len(arr)
    buckets = [[] for _ in range(bucket_count)]
    
    # Distribute elements
    min_val = min(arr)
    max_val = max(arr)
    range_size = max_val - min_val
    
    for num in arr:
        if range_size == 0:
            index = 0
        else:
            index = int((num - min_val) * (bucket_count - 1) / range_size)
        buckets[index].append(num)
    
    # Sort individual buckets and concatenate
    result = []
    for bucket in buckets:
        result.extend(sorted(bucket))
    
    return result
```

#### Sorting Algorithm Comparison

| Algorithm | Best | Average | Worst | Space | Stable | Notes |
|-----------|------|---------|-------|-------|--------|-------|
| Quick Sort | O(n log n) | O(n log n) | O(n²) | O(log n) | No | Fast in practice |
| Merge Sort | O(n log n) | O(n log n) | O(n log n) | O(n) | Yes | Predictable |
| Heap Sort | O(n log n) | O(n log n) | O(n log n) | O(1) | No | In-place |
| Counting Sort | O(n + k) | O(n + k) | O(n + k) | O(k) | Yes | Limited range |
| Radix Sort | O(d(n + k)) | O(d(n + k)) | O(d(n + k)) | O(n + k) | Yes | For integers |
| Bucket Sort | O(n + k) | O(n + k) | O(n²) | O(n) | Yes | Uniform distribution |

#### Custom Comparators

In practice, you'll often need to sort with custom criteria:

```python
# Sort by multiple criteria
students = [("Alice", 85), ("Bob", 90), ("Charlie", 85)]

# Sort by score descending, then name ascending
students.sort(key=lambda x: (-x[1], x[0]))

# Sort using custom comparison function
from functools import cmp_to_key

def compare(a, b):
    if a[1] != b[1]:
        return b[1] - a[1]  # Descending score
    return -1 if a[0] < b[0] else 1  # Ascending name

students.sort(key=cmp_to_key(compare))
```

#### Sorting Tips

1. **Use Built-in Sort**: Unless specifically asked, use language built-in sorting
2. **Stable vs Unstable**: Know when stability matters
3. **In-place vs Extra Space**: Consider memory constraints
4. **Custom Comparators**: Master custom sorting criteria
5. **Partial Sorting**: Use heaps for k smallest/largest elements
6. **Nearly Sorted**: Insertion sort performs well on nearly sorted data

### Bit Manipulation

Bit manipulation involves working directly with binary representations of numbers. It's used for optimization and solving specific types of problems efficiently.

#### Basic Bit Operations

```python
# AND (&): Both bits must be 1
5 & 3  # 0101 & 0011 = 0001 = 1

# OR (|): At least one bit must be 1
5 | 3  # 0101 | 0011 = 0111 = 7

# XOR (^): Bits must be different
5 ^ 3  # 0101 ^ 0011 = 0110 = 6

# NOT (~): Flip all bits
~5  # ~0101 = ...11111010 = -6 (two's complement)

# Left Shift (<<): Multiply by 2^n
5 << 1  # 0101 << 1 = 1010 = 10
5 << 2  # 0101 << 2 = 10100 = 20

# Right Shift (>>): Divide by 2^n
5 >> 1  # 0101 >> 1 = 0010 = 2
5 >> 2  # 0101 >> 2 = 0001 = 1
```

#### Common Bit Manipulation Tricks

**1. Check if Number is Power of Two**

```python
def is_power_of_two(n):
    return n > 0 and (n & (n - 1)) == 0

# Why this works:
# Power of 2:     1000 (8)
# Power of 2 - 1: 0111 (7)
# AND:            0000 (0)
```

**2. Get, Set, Clear, Toggle Bit**

```python
def get_bit(num, i):
    return (num & (1 << i)) != 0

def set_bit(num, i):
    return num | (1 << i)

def clear_bit(num, i):
    return num & ~(1 << i)

def toggle_bit(num, i):
    return num ^ (1 << i)
```

**3. Count Number of 1 Bits**

```python
def count_ones(n):
    count = 0
    while n:
        count += 1
        n &= (n - 1)  # Remove rightmost 1
    return count

# Or using Brian Kernighan's algorithm
def count_ones_fast(n):
    count = 0
    while n:
        n &= (n - 1)
        count += 1
    return count
```

**4. Find Single Number (All Others Appear Twice)**

```python
def single_number(nums):
    result = 0
    for num in nums:
        result ^= num
    return result

# XOR properties:
# a ^ a = 0
# a ^ 0 = a
# XOR is commutative and associative
```

**5. Swap Two Numbers Without Temp Variable**

```python
def swap(a, b):
    a ^= b
    b ^= a
    a ^= b
    return a, b
```

**6. Check if Numbers Have Opposite Signs**

```python
def opposite_signs(a, b):
    return (a ^ b) < 0
```

**7. Isolate Rightmost 1 Bit**

```python
def isolate_rightmost_one(n):
    return n & (-n)

# Example: 12 = 1100
# -12 in two's complement = 0100
# 12 & -12 = 0100 = 4
```

**8. Clear All Bits from LSB to i-th Bit**

```python
def clear_bits_lsb_to_i(num, i):
    mask = (-1 << (i + 1))
    return num & mask
```

**9. Generate All Subsets (Power Set)**

```python
def subsets_bitwise(nums):
    n = len(nums)
    result = []
    
    for i in range(1 << n):  # 2^n subsets
        subset = []
        for j in range(n):
            if i & (1 << j):
                subset.append(nums[j])
        result.append(subset)
    
    return result
```

#### Advanced Bit Manipulation

**1. Find Two Numbers That Appear Once (Others Twice)**

```python
def single_numbers(nums):
    # Get XOR of the two unique numbers
    xor = 0
    for num in nums:
        xor ^= num
    
    # Find rightmost set bit
    rightmost_bit = xor & (-xor)
    
    # Divide numbers into two groups
    num1 = num2 = 0
    for num in nums:
        if num & rightmost_bit:
            num1 ^= num
        else:
            num2 ^= num
    
    return [num1, num2]
```

**2. Reverse Bits**

```python
def reverse_bits(n):
    result = 0
    for i in range(32):
        result = (result << 1) | (n & 1)
        n >>= 1
    return result
```

**3. Find Missing Number**

```python
def missing_number(nums):
    n = len(nums)
    missing = n
    
    for i, num in enumerate(nums):
        missing ^= i ^ num
    
    return missing
```

**4. Power Set with Bits**

```python
def power_set_bits(s):
    n = len(s)
    power_set = []
    
    # Generate all 2^n combinations
    for i in range(1 << n):
        subset = []
        for j in range(n):
            if (i >> j) & 1:
                subset.append(s[j])
        power_set.append(subset)
    
    return power_set
```

#### Common Bit Manipulation Problems

- **LeetCode 136**: Single Number
- **LeetCode 137**: Single Number II
- **LeetCode 190**: Reverse Bits
- **LeetCode 191**: Number of 1 Bits
- **LeetCode 231**: Power of Two
- **LeetCode 268**: Missing Number
- **LeetCode 338**: Counting Bits
- **LeetCode 371**: Sum of Two Integers

#### Bit Manipulation Tips

1. **XOR for Uniqueness**: XOR is great for finding unique elements
2. **Powers of 2**: Use `n & (n-1)` to check or remove powers of 2
3. **Masks**: Use masks to isolate or modify specific bits
4. **Shifts for Multiplication/Division**: Faster than arithmetic operations
5. **Sign Bit**: Leftmost bit determines sign in two's complement
6. **Practice**: Bit manipulation requires practice to build intuition

## Study Resources

### Online Learning Platforms

**LeetCode**
- Primary platform for this repository
- Start with Easy problems, progress to Medium, then Hard
- Use the "Explore" section for curated study plans
- Join weekly contests to practice under time pressure
- https://leetcode.com

**NeetCode**
- Excellent curated problem list organized by pattern
- Video explanations for most problems
- Focus on the Blind 75 list for interview prep
- https://neetcode.io

**AlgoExpert**
- Comprehensive video explanations
- Well-organized by difficulty and category
- Good for systematic learning
- https://algoexpert.io (Paid)

**Educative.io**
- Interactive coding environment
- "Grokking" series is highly recommended
- Text-based explanations
- https://educative.io (Paid)

### Books

**Introduction to Algorithms (CLRS)**
- Authors: Cormen, Leiserson, Rivest, Stein
- The definitive algorithms textbook
- Comprehensive but dense
- Best for deep understanding

**Cracking the Coding Interview**
- Author: Gayle Laakmann McDowell
- 189 programming questions and solutions
- Excellent for interview preparation
- Includes behavioral interview tips

**Elements of Programming Interviews**
- Authors: Aziz, Lee, Prakash
- Available in Java, Python, C++
- More challenging than CTCI
- Great problem collection

**Algorithm Design Manual**
- Author: Steven Skiena
- Practical approach to algorithms
- War stories from real applications
- Good balance of theory and practice

### YouTube Channels

**NeetCode**
- Clear problem explanations
- Covers most LeetCode patterns
- Good visualization

**Back To Back SWE**
- In-depth explanations
- Focus on understanding, not just solving

**Tushar Roy - Coding Made Simple**
- Classic algorithm problems
- Good for DP and graph algorithms

**Abdul Bari**
- Excellent algorithm fundamentals
- Great visualizations
- Good for learning theory

### Websites and Blogs

**GeeksforGeeks**
- Huge problem collection
- Multiple approaches explained
- Good for learning basics

**HackerRank**
- Good for practice
- Organized by difficulty and topic
- Certification tests available

**Codeforces**
- Competitive programming platform
- More advanced problems
- Active community

**CP-Algorithms**
- Comprehensive algorithm encyclopedia
- Clear explanations with code
- https://cp-algorithms.com

## Problem-Solving Strategies

### General Approach

**1. Understand the Problem**
- Read carefully, identify inputs and outputs
- Look for constraints (size, value ranges)
- Try small examples manually
- Ask clarifying questions

**2. Plan Your Approach**
- Identify the problem type (array, tree, graph, etc.)
- Recognize patterns you've seen before
- Consider multiple approaches
- Think about edge cases

**3. Start with Brute Force**
- Get a working solution first
- Analyze time and space complexity
- This gives you a baseline to improve upon

**4. Optimize**
- Can you eliminate redundant work?
- Can you use a different data structure?
- Can you apply a specific algorithm pattern?
- Trade space for time if needed

**5. Code Carefully**
- Write clean, readable code
- Use meaningful variable names
- Handle edge cases
- Test with examples

**6. Test Thoroughly**
- Normal cases
- Edge cases (empty, single element, etc.)
- Large inputs
- Special cases from problem description

### Pattern Recognition

Learning to recognize patterns is crucial. Here are the most common:

**Array Problems:**
- Two pointers for sorted arrays or pairs
- Sliding window for subarrays/substrings
- Hash map for frequency/lookup
- Prefix sum for range queries

**String Problems:**
- Two pointers for palindromes
- Hash map for anagrams
- Sliding window for substrings
- Trie for prefix matching

**Linked List Problems:**
- Fast and slow pointers for cycles
- Dummy node for simplification
- Reverse in groups
- Merge sorted lists

**Tree Problems:**
- Recursion for traversal
- BFS for level-order
- DFS for path problems
- BST properties for optimization

**Graph Problems:**
- BFS for shortest path (unweighted)
- DFS for exploration/connectivity
- Topological sort for dependencies
- Union-Find for connectivity

**Dynamic Programming:**
- Identify optimal substructure
- Find overlapping subproblems
- Define state clearly
- Build bottom-up or top-down

### Time Management During Interviews

**Before the Interview:**
- Practice typing code quickly
- Get comfortable with your IDE/editor
- Know your language's standard library
- Practice explaining your thought process

**During the Interview:**
- First 5 minutes: Understand and clarify
- Next 10 minutes: Plan and discuss approach
- Next 20 minutes: Code the solution
- Last 5 minutes: Test and discuss improvements

**Don't:**
- Rush into coding without understanding
- Stay silent - communicate your thoughts
- Give up if stuck - ask for hints
- Ignore edge cases

## Time and Space Complexity Analysis

Understanding complexity analysis is crucial for evaluating solutions.

### Big O Notation

Common time complexities from fastest to slowest:
- **O(1)**: Constant - array access, hash map lookup
- **O(log n)**: Logarithmic - binary search
- **O(n)**: Linear - single array traversal
- **O(n log n)**: Linearithmic - efficient sorting
- **O(n²)**: Quadratic - nested loops
- **O(2^n)**: Exponential - subsets, permutations
- **O(n!)**: Factorial - all permutations

### Analyzing Time Complexity

**Sequential Operations**: Add them
```python
def example(arr):
    for x in arr:  # O(n)
        print(x)
    
    for x in arr:  # O(n)
        print(x)
    
    # Total: O(n) + O(n) = O(2n) = O(n)
```

**Nested Operations**: Multiply them
```python
def example(arr):
    for i in arr:       # O(n)
        for j in arr:   # O(n)
            print(i, j)
    
    # Total: O(n) * O(n) = O(n²)
```

**Recursive Complexity**: Use recurrence relations
```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

# T(n) = T(n-1) + T(n-2) + O(1)
# Time: O(2^n)
# With memoization: O(n)
```

**Master Theorem**: For divide and conquer
```
T(n) = aT(n/b) + f(n)

Examples:
- Merge sort: T(n) = 2T(n/2) + O(n) = O(n log n)
- Binary search: T(n) = T(n/2) + O(1) = O(log n)
```

### Analyzing Space Complexity

**Auxiliary Space**: Extra space used beyond input
```python
def reverse_array(arr):
    return arr[::-1]  # O(n) extra space

def reverse_inplace(arr):
    left, right = 0, len(arr) - 1
    while left < right:
        arr[left], arr[right] = arr[right], arr[left]
        left += 1
        right -= 1
    # O(1) extra space
```

**Recursion Stack**: Depth of recursion
```python
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

# Space: O(n) due to recursion stack
```

### Common Complexity Patterns

| Operation | Time | Space | Example |
|-----------|------|-------|---------|
| Array access | O(1) | O(1) | arr[i] |
| Array search | O(n) | O(1) | Linear search |
| Binary search | O(log n) | O(1) | Sorted array |
| Sorting | O(n log n) | O(1) to O(n) | Merge/Quick sort |
| Hash map lookup | O(1) avg | O(n) | Dictionary |
| Tree traversal | O(n) | O(h) | DFS/BFS |
| Graph BFS/DFS | O(V + E) | O(V) | Adjacency list |

## Tips and Tricks

### General Problem-Solving Tips

**1. Read Constraints Carefully**
- Small n (< 20): Consider exponential solutions (backtracking, DP)
- Medium n (< 1000): O(n²) might work
- Large n (> 10^6): Need O(n) or O(n log n)

**2. Look for Hints in Examples**
- Examples often reveal patterns
- Edge cases are sometimes hidden in examples
- Try to break the examples

**3. Use Helper Functions**
- Keep main logic clean
- Easier to test and debug
- More readable

**4. Don't Optimize Prematurely**
- Get a working solution first
- Then optimize if needed
- But know the optimal solution exists

### Language-Specific Tips

**Python:**
```python
# Use collections module
from collections import defaultdict, Counter, deque

# List comprehensions
squares = [x*x for x in range(10)]

# Enumerate instead of range
for i, val in enumerate(arr):
    pass

# Multiple assignment
a, b = b, a  # Swap

# Slicing
reversed_arr = arr[::-1]

# zip for parallel iteration
for x, y in zip(arr1, arr2):
    pass

# any() and all()
has_positive = any(x > 0 for x in arr)
all_positive = all(x > 0 for x in arr)
```

**Java:**
```java
// Use built-in data structures
Map<Integer, Integer> map = new HashMap<>();
Set<Integer> set = new HashSet<>();
Queue<Integer> queue = new LinkedList<>();
PriorityQueue<Integer> pq = new PriorityQueue<>();

// Arrays utility
Arrays.sort(arr);
Arrays.fill(arr, 0);
int[] copy = Arrays.copyOf(arr, arr.length);

// Collections utility
Collections.sort(list);
Collections.reverse(list);

// StringBuilder for string concatenation
StringBuilder sb = new StringBuilder();
```

**C++:**
```cpp
// STL containers
vector<int> vec;
unordered_map<int, int> map;
unordered_set<int> set;
priority_queue<int> pq;

// Algorithms
sort(vec.begin(), vec.end());
reverse(vec.begin(), vec.end());
auto it = lower_bound(vec.begin(), vec.end(), target);

// Lambda functions
sort(vec.begin(), vec.end(), [](int a, int b) {
    return a > b;  // Descending order
});
```

### Common Mistakes to Avoid

1. **Off-by-One Errors**
   - Check array bounds carefully
   - Remember: 0-indexed vs 1-indexed

2. **Integer Overflow**
   - Use long for large sums
   - Check for multiplication overflow

3. **Modifying While Iterating**
   - Don't modify a collection while iterating
   - Use a copy or iterate backwards

4. **Not Handling Edge Cases**
   - Empty input
   - Single element
   - All same elements
   - Negative numbers

5. **Wrong Data Structure**
   - Array vs LinkedList
   - HashSet vs TreeSet
   - Stack vs Queue

### Debugging Strategies

1. **Print Statements**
   - Print intermediate values
   - Verify assumptions

2. **Small Examples**
   - Test with minimal cases
   - Hand-trace the algorithm

3. **Rubber Duck Debugging**
   - Explain code line by line
   - Often reveals the bug

4. **Binary Search Debugging**
   - Comment out half the code
   - Find which half has the bug

5. **Check Assumptions**
   - Are inputs what you expect?
   - Are indices correct?
   - Is the algorithm logic sound?

## Interview Preparation

### Study Plan

**Week 1-2: Foundations**
- Arrays and Strings (Easy problems)
- Hash Tables
- Two Pointers
- Practice: 20-30 easy problems

**Week 3-4: Core Data Structures**
- Linked Lists
- Stacks and Queues
- Trees and BST
- Practice: 15-20 easy, 10-15 medium

**Week 5-6: Algorithms**
- Binary Search
- Recursion and Backtracking
- Sorting
- Practice: 20-30 medium problems

**Week 7-8: Advanced Topics**
- Dynamic Programming
- Graphs
- Advanced Trees (Trie, Segment Tree)
- Practice: 30-40 medium, 5-10 hard

**Week 9-10: Practice and Review**
- Mock interviews
- Contest participation
- Review weak areas
- Practice: Mix of all difficulties

**Week 11-12: Final Prep**
- Company-specific problems
- System design basics
- Behavioral questions
- Mock interviews

### Problem Selection Strategy

**For Beginners:**
- Start with LeetCode Easy
- Focus on: Arrays, Strings, Hash Maps
- Do 2-3 problems daily
- Review solutions of others

**For Intermediate:**
- LeetCode Medium (70%) + Easy (30%)
- Cover all major patterns
- Do 1-2 problems daily
- Time yourself

**For Advanced:**
- LeetCode Hard (40%) + Medium (60%)
- Focus on optimization
- Participate in contests
- Review multiple approaches

### Company-Specific Preparation

**FAANG Companies:**
- Heavy on algorithms and data structures
- Expect 4-5 rounds of coding
- Study: Dynamic Programming, Graphs, System Design
- Resources: Blind 75, Grind 75

**Startups:**
- More practical problems
- Focus on clean code and testing
- May involve real project questions
- Be ready to discuss trade-offs

**Product Companies:**
- Balance of algorithms and design
- Behavioral questions important
- Focus on communication
- May have take-home assignments

### Behavioral Interview Prep

**STAR Method:**
- **S**ituation: Set the context
- **T**ask: Describe the challenge
- **A**ction: Explain what you did
- **R**esult: Share the outcome

**Common Questions:**
- Tell me about yourself
- Why this company?
- Describe a challenging project
- How do you handle conflict?
- Tell me about a failure
- What are your strengths/weaknesses?

**Prepare Stories About:**
- Technical challenges overcome
- Leadership experiences
- Team conflicts resolved
- Failed projects and learnings
- Innovations or improvements made

### Mock Interview Tips

1. **Practice Out Loud**: Verbalize your thought process
2. **Use a Whiteboard**: Get comfortable with whiteboarding
3. **Time Yourself**: Stick to interview time limits
4. **Get Feedback**: Ask for constructive criticism
5. **Record Yourself**: Review and improve

### Day Before Interview

- Review key algorithms and patterns
- Don't try to learn new concepts
- Prepare questions for the interviewer
- Get good sleep
- Prepare your environment (for virtual interviews)

### During the Interview

**First 5 Minutes:**
- Listen carefully to the question
- Ask clarifying questions
- Discuss constraints and edge cases
- Propose an approach before coding

**Next 20-30 Minutes:**
- Think out loud
- Start with a working solution
- Optimize if time permits
- Write clean, readable code

**Last 5-10 Minutes:**
- Test your code with examples
- Discuss time and space complexity
- Mention possible optimizations
- Ask questions about the team/company

### After the Interview

- Send thank-you email within 24 hours
- Reflect on what went well and what didn't
- Note any topics you struggled with
- Continue practicing regardless of outcome

## Contributing

Contributions are welcome! If you'd like to add solutions or improve existing ones:

1. Fork the repository
2. Create a new branch for your changes
3. Follow the existing structure and formatting
4. Ensure your code is well-commented
5. Submit a pull request with a clear description

Please ensure:
- Solutions are correct and efficient
- Code is clean and readable
- Time and space complexity is documented
- Multiple approaches are provided when applicable

## License

This repository is open source and available under the MIT License. Feel free to use it for learning and interview preparation.

---

## Final Thoughts

Mastering algorithms and data structures is a journey, not a destination. It requires consistent practice, patience, and perseverance. Don't get discouraged by difficult problems. Every problem you solve makes you a better problem solver.

Remember:
- Quality over quantity: Understand deeply rather than solving many problems superficially
- Learn from mistakes: Every wrong approach teaches you something
- Stay consistent: Regular practice is more effective than cramming
- Enjoy the process: Problem-solving can be fun once you get the hang of it

Good luck on your coding journey! Happy coding!

---

### Additional Resources

**Online Judges:**
- LeetCode: https://leetcode.com
- HackerRank: https://hackerrank.com
- CodeForces: https://codeforces.com
- AtCoder: https://atcoder.jp

**Visualization Tools:**
- VisuAlgo: https://visualgo.net
- Algorithm Visualizer: https://algorithm-visualizer.org
- Python Tutor: http://pythontutor.com

**Communities:**
- LeetCode Discuss
- Reddit r/leetcode
- Reddit r/cscareerquestions
- Stack Overflow

**Books (Free Online):**
- Competitive Programming 3 by Steven Halim
- Algorithms by Jeff Erickson
- Problem Solving with Algorithms and Data Structures using Python

**YouTube Playlists:**
- MIT OpenCourseWare - Introduction to Algorithms
- Stanford - Algorithms Specialization
- Princeton - Algorithms Parts I & II

Remember: The key to success is consistent practice and genuine understanding. Don't just memorize solutions—understand the patterns and principles behind them. Good luck!