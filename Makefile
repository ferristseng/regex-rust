all:
	rustc regexp.rs

test: 
	rust test regexp.rs

clean:
	rm -r *~* regexp 
