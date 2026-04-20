#include <iostream>
#include <vector>
#include <string>
using namespace std;

class Solution {
public:
    vector<string> result;

    void backtrack(int index, string &digits, string &current, vector<string> &mapping) {
        if (index == digits.size()) {
            result.push_back(current);
            return;
        }

        int digit = digits[index] - '0';

        for (char ch : mapping[digit]) {
            current.push_back(ch);               
            backtrack(index + 1, digits, current, mapping); 
            current.pop_back();                   
        }
    }

    vector<string> letterCombinations(string digits) {
        if (digits.empty()) return {};

        vector<string> mapping = {
            "", "", "abc", "def", "ghi",
            "jkl", "mno", "pqrs", "tuv", "wxyz"
        };

        string current = "";
        backtrack(0, digits, current, mapping);
        return result;
    }
};

int main() {
    Solution sol;

    string digits="234";

    vector<string> combinations = sol.letterCombinations(digits);

    cout << "Letter combinations:\n";
    for (string s : combinations) {
        cout << s << " ";
    }

    return 0;
}
