#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

using namespace std;

mutex counterMtx;
int* counter = new int(0);

void incrementCounter() {
    lock_guard<mutex> lock(counterMtx);
    for (int i = 0; i < 1000000; ++i) {
        *counter += 1;
    }
    free(counter);
}

int main() {
    vector<thread> threads;
    for (int i = 0; i < 10; i++) {
        threads.push_back(thread(incrementCounter));
    }

    for (int i = 0; i < 10; i++) {
        threads[i].join();
    }

    cout << "Counter = " << *counter << endl;
    return 0;
}
