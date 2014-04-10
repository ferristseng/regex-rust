#include <iostream>
#include <cstdlib>
#include <chrono>

using namespace std;

int main(){

  auto start = chrono::steady_clock::now();

  system("./benches/rust_gen_bench");
  auto end = chrono::steady_clock::now();

  auto diff = end - start;
  cout << "RUST GEN TEST: " << chrono::duration <double, nano> (diff).count() << " ns" << endl;

  start = chrono::steady_clock::now();

  system("./benches/cpp_gen_bench");
  end = chrono::steady_clock::now();

  diff = end - start;
  cout << "C++ GEN TEST: " << chrono::duration <double, nano> (diff).count() << " ns" << endl;

  start = chrono::steady_clock::now();

  system("./benches/rust_search_bench");
  end = chrono::steady_clock::now();

  diff = end - start;
  cout << "RUST SEARCH TEST: " << chrono::duration <double, nano> (diff).count() << " ns" << endl;

  start = chrono::steady_clock::now();

  system("./benches/cpp_search_bench");
  end = chrono::steady_clock::now();

  diff = end - start;
  cout << "C++ SEARCH TEST: " << chrono::duration <double, nano> (diff).count() << " ns" << endl;



}
