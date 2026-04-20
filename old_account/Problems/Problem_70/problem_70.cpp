// Problem 70 : Climbing stairs
//You are climbing a staircase. It takes n steps to reach the top.
// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

// Example 1:
// Input: n = 2
// Output: 2
// Explanation: There are two ways to climb to the top.
// 1. 1 step + 1 step
// 2. 2 steps

// Example 2:
// Input: n = 3
// Output: 3
// Explanation: There are three ways to climb to the top.
// 1. 1 step + 1 step + 1 step
// 2. 1 step + 2 steps
// 3. 2 steps + 1 step


#include <iostream>

using namespace std;

class Solution{
    public:
    int climbStairs(int n){
        int step1=1;
        int step2=0;
        int total_ways=0;
        for (int i = 0; i < n; i++)
        {
            total_ways=step1+step2;
            step2=step1;
            step1=total_ways;
        }
        return total_ways;
    }
};

int main(){
    Solution s = Solution();
    int n;
    cout<<"Enter a Number"<<endl;
    cin>>n;
    cout<<"The Total ways to climd the stairs is: "<<s.climbStairs(n)<<endl;
    return 0;
}