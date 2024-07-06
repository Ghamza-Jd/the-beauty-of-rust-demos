#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

using namespace std;

mutex counterMtx;
int counter = 0;

void incrementCounter() {
    // locking mutex using lock_guard, new in C++17 and above
    lock_guard<mutex> lock(counterMtx);
    for (int i = 0; i < 1000000; ++i) {
        ++counter;
    }

    // mutex unlocked after going out of scope
}

int main() {
    vector<thread> threads;
    for (int i = 0; i < 10; i++) {
        threads.push_back(thread(incrementCounter));
    }

    for (int i = 0; i < 10; i++) {
        threads[i].join();
    }

    cout << "Counter = " << counter << endl;
    return 0;
}
