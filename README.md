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

In order for regular expressions to be used, they must first be parsed into expressions, then compiled into instructions that can be executed by the underlying ```PikeVM``` virtual machine. This is done by executing the following code (example using regexp ```a+b+```):

```rust
let regexp = UncompiledRegexp::new("a+b+");
```

*Note: The somewhat confusing use of ```UncompiledRegexp``` as a class name for a regular expression that has been compiled results from the manner in which the regular expression is compiled. Without regular expression compilation built into the Rust compiler, regular expressions can only be compiled at runtime. The ```UncompiledRegexp``` class represents a regular expression that has been compiled at runtime. In the future, compiler support for regular expression compilation can limit the use of ```UncompiledRegexp``` to only those regular expressions whose regular expression string is not known until runtime. See [this page](https://github.com/mozilla/rust/wiki/Lib-re#4-module-writing) for more information on this topic.*

The compilation of regular expressions includes the following stages:

  * Parsing
  * Compilation

### Parsing (```parse.rs```)

The first thing that happens when a regular expression string is converted into a usable regular expression is the parsing of the regular expression string. This is invoked by calling ```parse()``` and passing in the regular expression string, which returns a [Result](http://static.rust-lang.org/doc/0.9/std/result/enum.Result.html) that contains either the recursive definition of the regular expression (using the ```Expr``` enum type) or the ```ParseCode``` associated with the error encoutered in compiling the regular expression.

Parsing is an iterative function looping through the symbols in the input string as stored in a ```State``` object. This process includes several subroutines that handle the parsing of characters with specific meaning in regular expressions. ```Expr``` objects are built up in a stack. Subexpressions within the regular expression are parsed recursively using the same function as the root level (```_parse_recursive()```).

### Compilation (```compile.rs```)

Once the parse tree has been constructed for the regular expression, it can be turned into the [Pike VM](http://swtch.com/~rsc/regexp/regexp2.html) instructions to execute when running on an input string. This is accomplished by passing the ```Expr``` returned by ```parse()``` into ```compile_recursive()```, which returns an array of ```Instruction``` objects.

The algorithm proceeds recursively, matching each ```Expr``` by its type and compiling any subexpressions recursively as necessary. Like ```Expr```, ```Instruction``` is an enumerated type that contains types for each of the possible instructions for the [Pike VM](http://swtch.com/~rsc/regexp/regexp2.html) that ultimately matches the regular expressions. Unlike expressions, instructions are not recursively defined.

## Regular Expression Matching

As mentioned earlier, the regular expression algorithm used in this implementation is the Pike VM algorithm, in which a regular expression string is compiled into a set of instructions that tell the VM how to process an arbitrary input string. The following code will create a regular expression and check it against the beginning of an input string:

```rust
let regexp = UncompiledRegexp::new("a+b+");
regexp.exec("my test input"); // returns an Option<Match>
```

Under the hood, a new ```PikeVM``` object is created from the instruction list generated during regexp compilation. Next, ```run()``` is called on the resulting object and passed the input string. The Pike VM algorithm runs, generating new tasks for each split. As of right now, certain patterns that are not handled by the standard Pike VM algorithm are not handled properly (such as ```(a*)*```, which causes an infinite loop).

## Library Functions (API)

The current API for the ```UncompiledRegexp``` class consists of two functions, ```exec()``` and ```search()```, which perform a single match searching from the start of the string and an arbitraryposition in the string, respectively. Ultimately, we would like to implment all of the functions that are a part of the [Python re library](http://docs.python.org/2/library/re.html). *More information on specific functions coming soon.*