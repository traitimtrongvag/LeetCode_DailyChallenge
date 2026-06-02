/*
Time: O(n * m)
Space: O(1)

Idea:
Try every pair:
- ride i from land
- ride j from water

Do land first:
finishLand = landStart[i] + landDuration[i]

Water ride can start when:
- it becomes available
- or after land ride finishes

Then compute total finish time.

Also try water first -> land.

Take the minimum answer.
*/

class Solution {
public:
    int earliestFinishTime(vector<int>& landStartTime,
                           vector<int>& landDuration,
                           vector<int>& waterStartTime,
                           vector<int>& waterDuration) {

        long long answer = LLONG_MAX;

        int landCount = landStartTime.size();
        int waterCount = waterStartTime.size();

        for (int i = 0; i < landCount; i++) {
            for (int j = 0; j < waterCount; j++) {

                // land -> water
                long long finishLand =
                    (long long)landStartTime[i] + landDuration[i];

                long long startWater =
                    max(finishLand,
                        (long long)waterStartTime[j]);

                answer = min(
                    answer,
                    startWater + waterDuration[j]
                );

                // water -> land
                long long finishWater =
                    (long long)waterStartTime[j] + waterDuration[j];

                long long startLand =
                    max(finishWater,
                        (long long)landStartTime[i]);

                answer = min(
                    answer,
                    startLand + landDuration[i]
                );
            }
        }

        return (int)answer;
    }
};