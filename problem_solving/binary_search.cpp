#include <bits/stdc++.h>

using namespace std;


// binary search method 1
void search_01(vector<int> v, int item) {

    int start=0,end=v.size()-1;
    cout << start << " " << end << " " << (start + end)/2 << " " << v[start] << "\n";
    while (start <= end)
    {
        /* code */
        int i = (start+end)/2;

        if (v[i] == item) {
            // found the searching item and return
            cout << "search found at " << i << "\n";
            break;
        }
        // check searching item is greather or less to middle item
        if (v[i] > item) end = i-1;
        else start = i+1;
        // cout << i << start << end <<"\n";
    }
    // if (v[start] == item) {
    //     cout << "search fount at " << start << "\n";
    // } else if (v[end] == item) {
    //     cout << "seach found at" << end << "\n";
    // } else {
    //     cout << "search item not found";
    // }
}

// binary search method 2
void search_method_02(vector<int> v, int item) {
    int k = 0;
    int n = v.size();
    for (int i = n/2; i >=1; i /=2) {
        while (k+i < n && v[k+i] <= item) k += i;
    }
    if (v[k] == item) {
        cout << "Item found at: " << k << "\n";
    } else {
        cout << "Item not found\n";
    }
}

  bool ok(int index, int smallest, vector<int> v) {
       
        if (v[index] < smallest) {
            return false;
        }
        return true;
    }

// find the smallest solution with binary search
int find_smallest_solution(vector<int> v, int smallest) {
    int x = -1;
    int n = v.size()-1;
    for (int i=n/2;i >= 1;i/=2) {
        while (!ok(x+i, smallest, v)) x += i;
    }
    return x+1;
}

int main() {
    vector<int> v = {2, 44, 8, 90, 2,1, 9, 5, 11, 12, 345, 87};
    sort(v.begin(), v.end());
    // search_01(v, 9);
    // search_method_02(v, 9);
    // auto k = lower_bound(v.begin(), v.end(), 99) - v.begin();
    // cout << v[k] << "\n";
    cout << "smallest solutions: " << find_smallest_solution(v, 88) <<  " " << v[find_smallest_solution(v, 88)] << "\n";

}

