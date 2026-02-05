# Problem 175: Combine Two Tables

**Difficulty:** Easy  
**Topics:** Database

---

## Problem Description

Table: `Person`

```text
+-------------+---------+
| Column Name | Type |
+-------------+---------+
| personId | int |
```
| lastName | varchar |
| firstName | varchar |
```text
+-------------+---------+
personId is the primary key (column with unique values) for this table.
```
This table contains information about the ID of some persons and their first and last names.

Table: `Address`

```text
+-------------+---------+
| Column Name | Type |
+-------------+---------+
| addressId | int |
```
| personId | int |
| city | varchar |
| state | varchar |
```text
+-------------+---------+
addressId is the primary key (column with unique values) for this table.
```
Each row of this table contains information about the city and state of one person with ID = PersonId.

Write a solution to report the first name, last name, city, and state of each person in the `Person` table. If the address of a `personId` is not present in the `Address` table, report `null` instead.

Return the result table in 
any order.

The result format is in the following example.

### Example
Example 1
**Input:** Person table
```text
+----------+----------+-----------+
| personId | lastName | firstName |
+----------+----------+-----------+
| 1 | Wang | Allen |
```
| 2 | Alice | Bob |
```text
+----------+----------+-----------+
Address table
+-----------+----------+---------------+------------+
| addressId | personId | city | state |
+-----------+----------+---------------+------------+
| 1 | 2 | New York City | New York |
```
| 2 | 3 | Leetcode | California |
```text
+-----------+----------+---------------+------------+
```

**Output:** ```text
+-----------+----------+---------------+----------+
| firstName | lastName | city | state |
+-----------+----------+---------------+----------+
| Allen | Wang | Null | Null |
```
| Bob | Alice | New York City | New York |
```text
+-----------+----------+---------------+----------+
```

**Explanation:** There is no address in the address table for the personId = 1 so we return null in their city and state.
addressId = 1 contains information about the address of personId = 2.
