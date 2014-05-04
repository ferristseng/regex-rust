#include <iostream>
#include <cstdlib>
#include <chrono>

using namespace std;

int main(){

  chrono::duration<double> rust_gen, rustBS_gen, cpp_gen, rust_search, rustBS_search, cpp_search, python_gen, python_search;


  for (int i = 0; i < 10; i++) {
    chrono::steady_clock::time_point start = chrono::steady_clock::now();

    system("./benches/rust_gen_bench");
    chrono::steady_clock::time_point end = chrono::steady_clock::now();

    rust_gen += chrono::duration_cast<chrono::duration<double>>(end - start);

    start = chrono::steady_clock::now();

    system("./benches/cpp_gen_bench");
    end = chrono::steady_clock::now();

    cpp_gen += chrono::duration_cast<chrono::duration<double>>(end - start);

    start = chrono::steady_clock::now();

    system("./benches/rust_BS_gen_bench");
    end = chrono::steady_clock::now();

    rustBS_gen += chrono::duration_cast<chrono::duration<double>>(end - start);

    start = chrono::steady_clock::now();

    system("python ./benches/python_gen_bench.py");
    end = chrono::steady_clock::now();

    python_gen += chrono::duration_cast<chrono::duration<double>>(end - start);


    start = chrono::steady_clock::now();

    system("./benches/rust_search_bench");
    end = chrono::steady_clock::now();

    rust_search += chrono::duration_cast<chrono::duration<double>>(end - start);

    start = chrono::steady_clock::now();

    system("./benches/cpp_search_bench");
    end = chrono::steady_clock::now();

    cpp_search += chrono::duration_cast<chrono::duration<double>>(end - start);

    start = chrono::steady_clock::now();

    system("./benches/rust_BS_search_bench");
    end = chrono::steady_clock::now();

    rustBS_search += chrono::duration_cast<chrono::duration<double>>(end - start);

    start = chrono::steady_clock::now();

    system("python ./benches/python_search_bench.py");
    end = chrono::steady_clock::now();

    python_search += chrono::duration_cast<chrono::duration<double>>(end - start);
  }

  cout << "RUST GEN TEST: " << chrono::duration<double> (rust_gen).count() << " s" << endl;
  cout << "C++ GEN TEST: " << chrono::duration<double> (cpp_gen).count() << " s" << endl;
  cout << "RUST BS GEN TEST: " << chrono::duration <double> (rustBS_gen).count() << " s" << endl;
  cout << "PYTHON GEN TEST: " << chrono::duration <double> (python_gen).count() << " s" << endl;
  cout << "RUST SEARCH TEST: " << chrono::duration <double> (rust_search).count() << " s" << endl;
  cout << "C++ SEARCH TEST: " << chrono::duration <double> (cpp_search).count() << " s" << endl;
  cout << "RUST BS SEARCH TEST: " << chrono::duration <double> (rustBS_search).count() << " s" << endl;
  cout << "PYTHON SEARCH TEST: " << chrono::duration <double> (python_search).count() << " s" << endl;


}
