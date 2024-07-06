#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

using namespace std;

int counter = 0;

void incrementCounter() {
    for (int i = 0; i < 1000000; ++i) {
        ++counter;
    }
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
