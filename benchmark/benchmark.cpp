#include <iostream>
#include <cstdlib>
#include <chrono>

using namespace std;

int main(){

  auto start = chrono::steady_clock::now();

  system("./benches/rust_bench");
  auto end = chrono::steady_clock::now();

  auto diff = end - start;
  cout << chrono::duration <double, nano> (diff).count() << " ns" << endl;


}
