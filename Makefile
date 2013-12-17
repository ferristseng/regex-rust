SRC 	= src
RE 		= re
TEST 	= test

OPT_LEVEL = 3
FLAGS = --opt-level=$(OPT_LEVEL)

SOURCES = lib.rs compile.rs error.rs exec.rs parse.rs regexp.rs \
					state.rs
LIBSOURCES = $(addprefix $(SRC)/$(RE)/, $(SOURCES))

all: build

build: $(LIBSOURCES)
	mkdir build/
	rustc $(FLAGS) --lib --out-dir build $(SRC)/$(RE)/lib.rs

test: $(SRC)/$(RE)/test
	./build/test

$(SRC)/$(RE)/test: build $(SRC)/$(TEST)/test_generator.py
	python $(SRC)/$(TEST)/test_generator.py
	rustc $(FLAGS) --test  -L build/ --out-dir build $(SRC)/$(RE)/test.rs

run: build
	rustc $(FLAGS) --out-dir build $(SRC)/$(RE)/lib.rs
	./build/lib	

clean:
	rm -r build/

