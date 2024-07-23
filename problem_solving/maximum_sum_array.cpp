#include <bits/stdc++.h>

#define loop(i, a, n) for (int i=a;i<n;i++)

using namespace std;

int maximum_subarray_n3(int a[], int n) {
    int best = 0;
    for (int i = 0;i<n; i++) {
        for (int j=i; j < n; j++) {
            int sum = 0;
            for (int k = i; k <= j; k++) {
                sum += a[k];
            }

            best = max(best, sum);
        }
    }
    return best;
}

int maximum_subarray_n2(int a[], int n) {
    int best = 0;
    for (int i = 0; i < n; i ++) {
        int sum = 0;
        for (int b = i; b < n; b++) {
            sum += a[b];
            best = max(best, sum);
        }
    }
    return best;
}

int maximum_subarray_n1(int a[], int n) {
    int best = 0, sum = 0;
    for (int i = 0; i < n; i++) {
        sum = max(a[i], sum+a[i]);
        best = max(best, sum);
    }
    return best;
}

int main() {
    int ar[10] = {2, -2, 1, 4, 7, 4, 9, 10, 6, 99};
    cout << maximum_subarray_n3(ar, 10) << "\n";
    cout << maximum_subarray_n2(ar, 10) << "\n";
    cout << maximum_subarray_n1(ar, 10) << "\n";

    // sorting
    vector<int> v = {4, 2, 1, -1, 0, 9, 3, 6};
    int size = v.size();
    

    for (int i=0;i<size;i++) {
        cout << v[i] << ", ";
    }
    cout << "\n";
    sort(v.begin(), v.end());
    for (int i=0;i<size;i++) {
        cout << v[i] << ", ";
    }
    cout << "\n";
    sort(v.rbegin(), v.rend());

    for (int i=0;i<size;i++) {
        cout << v[i] << ", ";
    }

    cout << "\n";

    string s = "monkey";

    cout << s << '\n';

    sort(s.begin(), s.end());

    cout << s << '\n';

    vector<pair<int, int>> v2;
    v2.push_back({2, 5});
    v2.push_back({8, 9});
    v2.push_back({2, 4});
    v2.push_back({4, 9});
    int len = v2.size();

    loop(i, 0, len) {
        cout << v2[i].first << " " << v2[i].second << "\n";
    }
    sort(v2.begin(), v2.end());
    printf("\n");
    loop(i, 0, len) {
         cout << v2[i].first << " " << v2[i].second << "\n";
    }

    vector<tuple<int, int, int>> v3;
    v3.push_back({2, 1, 5});
    v3.push_back({1, 0, 9});
    v3.push_back({3, 8, 10});
    v3.push_back({4, 9, 11});
    v3.push_back({8, 10, 90});

    int len2 = v3.size();
  
    tuple<int, int> year = {237, 560};
    
}


