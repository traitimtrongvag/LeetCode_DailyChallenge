#define ll long long

class Solution {
public:
    

    
    ll cal(string limit){



        // pos , last1 , last2 , touchthelimit (e.g.  21?? , limit = 2137)
        ll dp[20][11][11][2];

        //how many ways can arrive gere
        ll ways[20][11][11][2];

        memset(dp, 0, sizeof(dp));
        memset(ways, 0, sizeof(ways));

        //empty
        ways[0][10][10][1]=1;

        //how many digits
        int n = limit.size();
        
        // for every digit , last1/last2 for 0~10 (10 means not yet) , and whether it touch
        for(int pos=0;pos<n; pos++){
            for(int last1 = 0;last1<=10;last1++){
                for(int last2 = 0; last2 <=10 ; last2 ++){
                    for(int touchlimit = 0 ; touchlimit<=1 ; touchlimit++){


                        // how many ways can achieve this state
                        ll waynow = ways[pos][last1][last2][touchlimit];
                        ll dpnow = dp[pos][last1][last2][touchlimit];


                        //some implement of start / end
                        int start = 0;
                        int end = 9;

                        if (pos == 0) start = 1;
                        if (touchlimit) end = limit[pos] - '0';


                        // try every possible number for the new digit
                        for(int j=start ; j<=end;j++){

                            int nowlast1, nowlast2;
                            bool touchnow = touchlimit && (j == limit[pos] - '0');
                            int peak = 0;


                            // first digit
                            if (last1 == 10 && last2 == 10) {
                                nowlast1 = j;
                                nowlast2 = 10;
                            }
                            // second digit
                            else if (last1 != 10 && last2 == 10) {
                                nowlast2 = last1;
                                nowlast1 = j;
                            }
                            // already have two previous digits
                            else {
                                if (last1 > last2 && last1 > j) peak = 1;
                                if (last1 < last2 && last1 < j) peak = 1;

                                nowlast2 = last1;
                                nowlast1 = j;
                            }

    
                            // the new state
                            ways[pos+1][nowlast1][nowlast2][touchnow] += waynow ;

                            // all the ways that go into this state can benefit from peak
                            // e.g. : 9723 1 , 2423 1 (1 is the j) , both of them can achieve this state , and since 231 is peak , get 2 plus
                            dp[pos+1][nowlast1][nowlast2][touchnow] += waynow * peak + dpnow;


                        }
                    }

                    

                }
            }
        }
        ll ans = 0;

        for (int last1 = 0; last1 <= 10; last1++) {
            for (int last2 = 0; last2 <= 10; last2++) {
                for (int tight = 0; tight <= 1; tight++) {
                    // the final state
                    ans += dp[n][last1][last2][tight];
                }
            }
        }

        return ans;

    }



    ll solve(ll x) {
        if (x <= 0) return 0;

        string s = to_string(x);
        int n = s.size();

        ll ans = 0;
        string now = "";

        // shorter lengths: 9, 99, 999, ...
        for (int len = 1; len < n; len++) {
            now += '9';
            ans += cal(now);
        }

        // same length
        ans += cal(s);

        return ans;
    }

    long long totalWaviness(long long num1, long long num2) {
        return solve(num2) - solve(num1 - 1);
    }
};