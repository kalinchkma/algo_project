#include <bits/stdc++.h>

using namespace std;

#define print_vector(v) for(auto i: v) cout << i << "\n";



int main() {
    // shorer way to iterate vector
    vector<string> names = {"Hunter", "Mommy", "Nobaria", "Sasuke"};
    
    print_vector(names);
    cout << "=============\n";
    names.push_back("The primate");
    print_vector(names);
    cout << "==============\n";
    names[0] = "Coder";
    print_vector(names);
}


