all:
	rustc --opt-level=3 regexp.rs

test: 
	rust test regexp.rs

run: all
	./regexp

clean:
	rm -r *~* regexp 
