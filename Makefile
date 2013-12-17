SRC 	= src
RE 		= re
TEST 	= test

OPT_LEVEL = 3
FLAGS = --opt-level=$(OPT_LEVEL)

SOURCES = lib.rs compile.rs error.rs exec.rs parse.rs regexp.rs \
					state.rs
LIBSOURCES = $(addprefix $(SRC)/$(RE)/, $(SOURCES))

all: build

build: 
	mkdir build/

lib: build $(LIBSOURCES)
	rustc $(FLAGS) --lib --out-dir build $(SRC)/$(RE)/lib.rs

libtest: build $(LIBSOURCES)
	rustc $(FLAGS) --test --out-dir build $(SRC)/$(RE)/lib.rs
	./build/lib

test: $(SRC)/$(RE)/test
	./build/test

$(SRC)/$(RE)/test: lib $(SRC)/$(TEST)/test_generator.py
	python $(SRC)/$(TEST)/test_generator.py
	rustc $(FLAGS) --test  -L build/ --out-dir build $(SRC)/$(RE)/test.rs

run: build
	rustc $(FLAGS) --out-dir build $(SRC)/$(RE)/lib.rs
	./build/lib	

clean:
	rm -r build/

