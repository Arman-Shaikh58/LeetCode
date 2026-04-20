// 5. Longest Palindromic Substring
// Given a string s, return the longest palindromic substring in s.
// Example 1:

// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.
// Example 2:

// Input: s = "cbbd"
// Output: "bb"
 

// Constraints:
// 1 <= s.length <= 1000
// s consist of only digits and English letters.

#include <iostream>

using namespace std;

class Solution{
    public:
     string longestPalindrome(string s){
        string res="";
        int resLen=0;
        for (int i = 0; i < s.length(); i++)
        {
            int l=i,r=i;
            while(l>=0 && r<s.length() && s[l]==s[r]){
                if (r-l+1>resLen)
                {
                    res=s.substr(l,r-l+1);
                    resLen=r-l+1;
                }
                l-=1;
                r+=1;
            }
            l=i;r=i+1;
            while(l>=0 && r<s.length() && s[l]==s[r]){
                if (r-l+1>resLen)
                {
                    res=s.substr(l,r-l+1);
                    resLen=r-l+1;
                }
                l-=1;
                r+=1;
            }
        }
        return res;
     }
};

int main(){
    Solution s = Solution();
    string str1;
    cout<<"Enter a string containing a palidrome in it"<<endl;
    cin>>str1;
    cout<<"The Longest Palidorme is: "<<s.longestPalindrome(str1)<<endl;
    return 0;
}
