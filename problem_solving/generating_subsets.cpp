#include <bits/stdc++.h>

using namespace std;

#define print_vector(v) for(auto i: v) cout << "p:" << i << "\n";
#define print_vector_line(v) for (auto i: v) cout << i << " ";

#define loop(i, a, n) for(int i=a;i<a;i++)

// method one go through all 
// vector<vector<int>> search(int k, int n) {
//   if (k == n) {

//   } else {

//   }
// }

int main() {
    vector<int> v = {0, 1, 2};
    vector<vector<int>> subsets = {};
    // search(0, v.size());
    // print_vector(subsets);
    // finding subset using method 2:
    // int n = v.size();
    // cout << n << "\n";
    // for (int i=0; i < (1<<n); i++) {
    //     cout << i << " " << (1<<n) << " ";
    //     vector<int> subset;
    //     for (int j = 0; j<  n;j++) {

    //         if (i&(1<<j)) subset.push_back(j);
    //     }
    //     vector<int> c;
    //     cout << "Set index: ";
    //     for (int a : subset) {
    //         cout << a << " "; 
    //         c.push_back(v[a]);
    //     }
    //     cout << "\n";
    //     subsets.push_back(c);
    // }
    // cout << subsets.size() << "\n";
    // for (auto s : subsets){
    //     cout << "{ ";
    //     print_vector_line(s);
    //     cout << " }";
    //     cout << "\n";
    // }
}

