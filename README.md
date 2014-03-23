A regular expression library implemented natively in Rust, that is inspired by this [series of articles](http://swtch.com/~rsc/regexp/).

The library aims to cover a subset of the ones available for PCRE, namely:

(currently, not all of these are fully implemented)

  * Consistent escaping rules
  * Extended character classes 
  * Minimal matching (a.k.a. “ungreedy”)
  * Unicode character properties 
  * Multiline matching 
  * Newline/linebreak options 
  * Named subpatterns 
  * Backreferences 
  * Look-ahead and look-behind assertions 
  * Comments 

The library provides an interface and suite of functions similar to the one available in the Python regular expression library. 

## Compiling Regular Expressions

The compilation of regular expressions includes the following stages:

  * Parsing
  * Compilation

### Parsing (```parse.rs```)

The first thing that happens when a regular expression string is converted into a usable regular expression is the parsing of the regular expression string. This is invoked by calling ```parse()``` and passing in the regular expression string, which returns a [Result](http://static.rust-lang.org/doc/0.9/std/result/enum.Result.html) that contains either the recursive definition of the regular expression (using the ```Expr``` enum type) or the ```ParseCode``` associated with the error encoutered in compiling the regular expression.

Parsing is an iterative function looping through the symbols in the input string as stored in a ```State``` object. This process includes several subroutines that handle the parsing of characters with specific meaning in regular expressions. ```Expr``` objects are built up in a stack. Subexpressions within the regular expression are parsed recursively using the same function as the root level (```_parse_recursive()```).

### Compilation (```compile.rs```)

Once the parse tree has been constructed for the regular expression, it can be turned into the [Pike VM](http://swtch.com/~rsc/regexp/regexp2.html) instructions to execute when running on an input string. This is accomplished by passing the ```Expr``` returned by ```parse()``` into ```compile_recursive()```, which returns an array of ```Instruction``` objects.

The algorithm proceeds recursively, matching each ```Expr``` by its type and compiling any subexpressions recursively as necessary. Like ```Expr```, ```Instruction``` is an enumerated type that contains types for each of the possible instructions for the [Pike VM](http://swtch.com/~rsc/regexp/regexp2.html) that ultimately matches the regular expressions. Unlike expressions, instructions are not recursively defined.