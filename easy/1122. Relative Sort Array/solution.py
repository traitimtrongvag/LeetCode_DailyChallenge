class Solution(object):
    def relativeSortArray(self, arr1, arr2):
        counts = {}
        for num in arr1:
            counts[num] = counts.get(num, 0) + 1

        result = []
        for num in arr2:
            result.extend([num] * counts.pop(num, 0))

        for num in sorted(counts):
            result.extend([num] * counts[num])

        return result
