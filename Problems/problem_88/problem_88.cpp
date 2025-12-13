#include <iostream>
#include <vector>

using namespace std;

class Solution{
    public:
    void merge(vector<int>& nums1, int m, vector<int>& nums2, int n) {
        vector<int> nums1_cpy=nums1;
        int i=0,j=0,z=0;
        nums1.clear();
        nums1.resize(m+n);
        while (i<m && j<n)
        {
            if (nums1_cpy[i]>nums2[j])
            {
                nums1[z++]=nums2[j++];
            }else
                nums1[z++]=nums1_cpy[i++];
        }
        while (i<m)
        {
            nums1[z++]=nums1_cpy[i++];
        }
        while (j<n)
        {
            nums1[z++]=nums2[j++];
        }
    }
};

int main(){

    Solution s = Solution();
    vector<int> nums1 = {1,2,3,0,0,0};
    int m= 3;
    vector<int> nums2 = {2,5,6};
    int n = 3;

    s.merge(nums1,m,nums2,n);

    for (int i = 0; i < nums1.size(); i++)
    {
        cout<<nums1[i]<<" ";
    }
    

    return 0;
}