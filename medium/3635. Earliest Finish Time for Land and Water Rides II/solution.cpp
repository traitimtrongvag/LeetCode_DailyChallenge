class Solution {
public:
    int earliestFinishTime(
        vector<int>& landStartTime,
        vector<int>& landDuration,
        vector<int>& waterStartTime,
        vector<int>& waterDuration
    ) {
        int earliestLandFinish = INT_MAX;

        // earliest possible finish time from any land ride
        for (int i = 0; i < landStartTime.size(); i++) {
            earliestLandFinish = min(
                earliestLandFinish,
                landStartTime[i] + landDuration[i]
            );
        }

        int earliestWaterFinish = INT_MAX;

        // earliest possible finish time from any water ride
        for (int i = 0; i < waterStartTime.size(); i++) {
            earliestWaterFinish = min(
                earliestWaterFinish,
                waterStartTime[i] + waterDuration[i]
            );
        }

        int answer = INT_MAX;

        // land -> water
        for (int i = 0; i < waterStartTime.size(); i++) {
            int startWaterRide = max(
                waterStartTime[i],
                earliestLandFinish
            );

            int finishTime =
                startWaterRide + waterDuration[i];

            answer = min(answer, finishTime);
        }

        // water -> land
        for (int i = 0; i < landStartTime.size(); i++) {
            int startLandRide = max(
                landStartTime[i],
                earliestWaterFinish
            );

            int finishTime =
                startLandRide + landDuration[i];

            answer = min(answer, finishTime);
        }

        return answer;
    }
};