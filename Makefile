SRC 	= src
RE 		= re
TEST 	= test
BUILD = build

OPT_LEVEL = 3
FLAGS = --opt-level=$(OPT_LEVEL)

DYLIB = libre-bdb08f4b4768859d-0.1.1.dylib

SOURCES = lib.rs compile.rs error.rs exec.rs parse.rs regexp.rs \
					state.rs
LIBSOURCES = $(addprefix $(SRC)/$(RE)/, $(SOURCES))

all: $(BUILD) $(BUILD)/$(DYLIB) $(BUILD)/test $(BUILD)/librun $(BUILD)/libtest

test: test_correctness

test_all: $(BUILD)/libtest
	./build/libtest

test_correctness: $(BUILD)/libtest
	./build/libtest python_tests

$(BUILD)/$(DYLIB): $(LIBSOURCES)
	test -d $(BUILD) || mkdir $(BUILD)
	rustc $(FLAGS) --lib --out-dir $(BUILD) $(SRC)/$(RE)/lib.rs

$(BUILD)/libtest: $(BUILD)/$(DYLIB) $(LIBSOURCES) $(SRC)/$(TEST)/test_generator.py
	test -d $(BUILD) || mkdir $(BUILD)
	python $(SRC)/$(TEST)/test_generator.py
	rustc $(FLAGS) --test -o $(BUILD)/libtest $(SRC)/$(RE)/lib.rs

$(BUILD)/librun: $(BUILD) $(LIBSOURCES)
	test -d $(BUILD) || mkdir $(BUILD)
	rustc $(FLAGS) -o $(BUILD)/librun  $(SRC)/$(RE)/lib.rs

clean:
	rm -r build/

