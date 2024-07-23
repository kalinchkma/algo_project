#include <bits/stdc++.h>

using namespace std;

int main() {
    // This two line for input and output from file
    freopen("input.txt", "r", stdin);
    freopen("output.txt", "w", stdout);
    // making input output efficient
    ios::sync_with_stdio(0);
    cin.tie(0);

    int a, b;
    string x;

    cin >> a >> b;
    getline(cin, x);
    cout << "Your inputs:\n" << a << "\t" << b << "\t" << x << "\n";    

    // The C function scanf and printf is faster but difficult to use
    // scanf("%d %d", &a, &b);

    // getline(cin, x);

    // printf("%d %d\n", a, b);
    // cout << x << "\n";

}