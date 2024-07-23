#include <bits/stdc++.h>

#define REF(i, a, b) for (int i = a; i <= b; i++)
#define SQ(a) (a)*(a)

using namespace std;

int main() {
    int i = INT_MAX;
    long l = LONG_MAX;
    long long ll = LONG_LONG_MAX;

    cout << i << "\t" << l << "\t" << ll << "\n";

    long long x = 1;
    int n = 5;
    int m = 3;
    for (int i = 1; i <= n; i++) {
        cout << x * i << "\n";
        x = (x*i)%m;
    }
    cout << x << "\n";

    // floating point number
    double d = 0.3*3+0.1;
    printf("%.20f\n", d);
    double a = 1.0000234230024;
    double b = 1.0000423242342;
    if (abs(a-b) < 1e-9) {
        cout << "a and b are equal\n";
    } else {
        cout << "a and b are not equal\n";
    }

    typedef long double ld;
    ld test = 23.2323;

    cout << test << "\n";
    REF(i,1, 10) {
        cout << i << "\n";
    }
    cout << SQ(3+3) << "\n";
}