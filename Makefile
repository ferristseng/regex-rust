SRC = src/re
SOURCES = lib.rs compile.rs error.rs exec.rs parse.rs regexp.rs \
					state.rs
LIBSOURCES = $(addprefix $(SRC)/, $(SOURCES))
OPT_LEVEL = 3

all: lib

lib: $(LIBSOURCES)
	rustc --opt-level=$(OPT_LEVEL) --lib $(SRC)/lib.rs
	echo "~ Compiled lib.rs ~"

test:
	rust test src/re/lib.rs

run: $(LIBSOURCES) 
	rustc --opt-level=$(OPT_LEVEL) $(SRC)/lib.rs
	./$(SRC)/lib

