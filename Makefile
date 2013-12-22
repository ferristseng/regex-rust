SRC 	= src
RE 		= re
TEST 	= test
BUILD = build

OPT_LEVEL = 3
FLAGS = --opt-level=$(OPT_LEVEL)

DYLIB = libre-bdb08f4b4768859d-0.1.1.dylib

SOURCES = lib.rs compile.rs error.rs exec.rs parse.rs regexp.rs \
					state.rs parsable.rs charclass.rs result.rs
LIBSOURCES = $(addprefix $(SRC)/$(RE)/, $(SOURCES))

TESTS = test_generator.py cases.py
TESTSOURCES = $(addprefix $(SRC)/$(TEST)/, $(TESTS))

all: $(BUILD)/$(DYLIB) $(BUILD)/run $(BUILD)/test

test: test_correctness
    
run: $(BUILD)/run
	./build/run

test_all: $(BUILD)/test
	./build/test

test_correctness: $(BUILD)/test
	./build/test python_tests

check: $(LIBSOURCES)
	rustc $(FLAGS) --no-trans $(SRC)/$(RE)/lib.rs

$(BUILD)/$(DYLIB): $(LIBSOURCES)
	test -d $(BUILD) || mkdir $(BUILD)
	rustc $(FLAGS) --lib --out-dir $(BUILD) $(SRC)/$(RE)/lib.rs

$(BUILD)/test: $(LIBSOURCES) $(TESTSOURCES) 
	test -d $(BUILD) || mkdir $(BUILD)
	python $(SRC)/$(TEST)/test_generator.py
	rustc $(FLAGS) --test -o $(BUILD)/test $(SRC)/$(RE)/lib.rs

$(BUILD)/run: $(BUILD) $(LIBSOURCES)
	test -d $(BUILD) || mkdir $(BUILD)
	rustc $(FLAGS) -o $(BUILD)/run  $(SRC)/$(RE)/lib.rs

clean:
	rm -r build/

