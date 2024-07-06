#include <iostream>
#include <vector>

using namespace std;

int main() {
    vector<int> v;
    v.assign(4, 10);

    int* x = &v[0];

    cout << "v[0] = " << *x << endl;
    return 0;
}
