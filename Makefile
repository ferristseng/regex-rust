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

main: all src/re/main.rs
	rustc --opt-level=3 -L $(SRC) $(SRC)/main.rs
	echo "~ Compiled main.rs ~"

run: lib main 
	./src/re/main


