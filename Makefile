SRC 	= src
RE 		= re
TEST 	= test
BUILD = build
BENCH = benchmark

OPT_LEVEL = 3
FLAGS = --opt-level=$(OPT_LEVEL)

SOURCES = lib.rs compile.rs error.rs exec.rs parse.rs regexp.rs \
					state.rs charclass.rs result.rs
LIBSOURCES = $(addprefix $(SRC)/$(RE)/, $(SOURCES))

TESTS = test_generator.py cases.py
TESTSOURCES = $(addprefix $(SRC)/$(TEST)/, $(TESTS))


all: $(BUILD)/lib $(BUILD)/run $(BUILD)/test $(BUILD)/bench

test: test_correctness

run: $(BUILD)/run
	./build/run

bench: $(BUILD)/bench

test_all: $(BUILD)/test
	./build/test

test_correctness: $(BUILD)/test
	./build/test python_tests

check: $(LIBSOURCES)
	rustc $(FLAGS) --no-trans $(SRC)/$(RE)/lib.rs

$(BUILD)/lib: $(LIBSOURCES)
	test -d $(BUILD) || mkdir $(BUILD)
	rustc $(FLAGS) --crate-type=lib --out-dir $(BUILD) $(SRC)/$(RE)/lib.rs

$(BUILD)/test: $(LIBSOURCES) $(TESTSOURCES)
	test -d $(BUILD) || mkdir $(BUILD)
	python $(SRC)/$(TEST)/test_generator.py
	rustc $(FLAGS) --test -o $(BUILD)/test $(SRC)/$(RE)/lib.rs

$(BUILD)/run: $(BUILD) $(LIBSOURCES)
	test -d $(BUILD) || mkdir $(BUILD)
	rustc $(FLAGS) -o $(BUILD)/run  $(SRC)/$(RE)/lib.rs

$(BUILD)/bench: $(BUILD)
	test -d $(BUILD) || mkdir $(BUILD)
	test -d $(BENCH)/benches || mkdir $(BENCH)/benches
	test -d $(BUILD)/benches || mkdir $(BUILD)/benches

	python $(BENCH)/generators/rust_bench_generator.py

	rustc $(FLAGS) --out-dir $(BUILD)/benches $(BENCH)/benches/rust_bench.rs  -L ./$(BUILD)

	clang++ -std=c++11 -stdlib=libc++ -o $(BUILD)/run_benchmark -Weverything $(BENCH)/benchmark.cpp


clean:
	rm -r build/
