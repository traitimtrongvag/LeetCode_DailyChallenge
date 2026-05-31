/*
Time: O(n log n)
Space: O(1) excluding sort

Idea:
Always destroy the smallest asteroid first.

If current mass >= asteroid mass:
- destroy it
- add its mass to ours

Otherwise it is impossible.
*/

class Solution {
public:
    bool asteroidsDestroyed(int mass, vector<int>& asteroids) {
        sort(asteroids.begin(), asteroids.end());

        long long currentMass = mass;

        for (int asteroid : asteroids) {
            // cannot destroy this asteroid
            if (currentMass < asteroid) {
                return false;
            }

            // gain its mass after destroying it
            currentMass += asteroid;
        }

        return true;
    }
};