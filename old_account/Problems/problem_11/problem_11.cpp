

// 11. Container With Most Water

// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

// Find two lines that together with the x-axis form a container, such that the container contains the most water.

// Return the maximum amount of water a container can store.

// Notice that you may not slant the container. burh!!

#include <iostream>
#include <vector>

using namespace std;

class Solution{
    public:
    int maxArea(vector<int>& height){
        int leftPointer = 0;
        int rightPointer = height.size()-1;
        int maxCapacity = 0;
        while (rightPointer>leftPointer)
        {
            int smallerWall;
            int currentCapacity;
            if (height[leftPointer]<height[rightPointer])
            {
                smallerWall = height[leftPointer];
                currentCapacity= smallerWall*(rightPointer-leftPointer);
                leftPointer++;
            }else{
                smallerWall = height[rightPointer];
                currentCapacity= smallerWall*(rightPointer-leftPointer);
                rightPointer--;
            }
             

            if (currentCapacity>maxCapacity)
            {
                maxCapacity=currentCapacity;
            }
        }
        return maxCapacity;
    }
};

int main (){
    Solution s = Solution();

    vector<int> heights = {1,8,6,2,5,4,8,3,7};

    cout<<"Output: "<<s.maxArea(heights);


    return 0;
}