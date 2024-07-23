#include <bits/stdc++.h>

using namespace std;

// sum of polynomial consecutive number
int sum_con(int a[], int n) {
    // procedural
    // int s = 0;
    // for (int i = 0; i < n; i++) {
    //     s += a[i];
    // }
    // return s;
    return (n*(n+1))/2;
}

// sum of polynomial square
int sum_poly(int a[], int n) {
    return (n*(n+1)*(2*n+1))/6;
}

// sum of arithmetic progression
int sum_arithmetic_progression(int a[], int n) {
    return (n*(a[0] + a[n-1]))/2;
}

// sum of geometric progression
int sum_geometric_progression(int a[], int n, int d) {
    return (a[n-1]*d - a[0])/(d-1);
}

// fectorial number calculating
int fectorial(int n) {
    if (n == 0) return 1;
    return n*fectorial(n-1);
}

    
int main() {
    // sum of sequence
    // int n[11] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11};
    // cout << sum_con(n, 11) << "\n";

    // sum of polynopmial 
    // int c_s[4] = {1, 4, 9, 16};
    // cout << sum_poly(c_s, 4) << "\n"; 

    // arithmetic progression
    // int p[11] = {1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21};
    // cout << sum_arithmetic_progression(p, 11) << "\n";

    // geometric progression
    // int g[5] = {3, 6, 12, 24, 48};
    // cout << "geometric progression: " << sum_geometric_progression(g, 5, 2) << "\n";
    // cout << pow(2, 4) << "\n";

    // fectorial number
    // cout << "Fect 0: " << fectorial(0) << "\n";
    // cout << "Fect 1: " << fectorial(1) << "\n";
    // cout << "Fect 2: " << fectorial(2) << "\n";
    // cout << "Fect 5: " << fectorial(5) << "\n";
    // cout << "Fect 7: " << fectorial(7) << "\n";


}
