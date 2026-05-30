class SegmentTree {
    int size;
    vector<int> tree;

public:
    SegmentTree(int n) {
        size = 1;

        while (size <= n) {
            size *= 2;
        }

        tree.assign(size * 2, 0);
    }

    void update(int position, int value) {
        position += size;
        tree[position] = value;

        position /= 2;

        while (position > 0) {
            tree[position] = max(tree[position * 2],
                                 tree[position * 2 + 1]);
            position /= 2;
        }
    }

    int query(int right) {
        int left = size;
        int currentRight = right + size + 1;

        int maximumGap = 0;

        while (left < currentRight) {
            if (left & 1) {
                maximumGap = max(maximumGap, tree[left]);
                left++;
            }

            if (currentRight & 1) {
                currentRight--;
                maximumGap = max(maximumGap, tree[currentRight]);
            }

            left /= 2;
            currentRight /= 2;
        }

        return maximumGap;
    }
};

class Solution {
public:
    vector<bool> getResults(vector<vector<int>>& queries) {
        int maxPosition = 0;

        for (auto& query : queries) {
            maxPosition = max(maxPosition, query[1]);
        }

        SegmentTree segTree(maxPosition + 1);

        // store obstacle positions
        set<int> obstacles;
        obstacles.insert(0);

        vector<bool> answer;

        for (auto& query : queries) {
            if (query[0] == 1) {
                int position = query[1];

                auto rightIt = obstacles.lower_bound(position);

                int rightObstacle = -1;
                if (rightIt != obstacles.end()) {
                    rightObstacle = *rightIt;
                }

                int leftObstacle = *prev(rightIt);

                // new gap ending at current obstacle
                segTree.update(position, position - leftObstacle);

                // split old gap
                if (rightObstacle != -1) {
                    segTree.update(rightObstacle,
                                   rightObstacle - position);
                }

                obstacles.insert(position);
            } else {
                int x = query[1];
                int blockSize = query[2];

                auto it = obstacles.upper_bound(x);

                int leftObstacle = *prev(it);

                // gap that contains x
                int currentGap = x - leftObstacle;

                // largest completed gap before leftObstacle
                int bestPreviousGap = segTree.query(leftObstacle);

                int largestAvailableGap =
                    max(currentGap, bestPreviousGap);

                answer.push_back(largestAvailableGap >= blockSize);
            }
        }

        return answer;
    }
};