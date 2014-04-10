# regex-rust

A regular expression library implemented natively in Rust, that is inspired by this [series of articles](http://swtch.com/~rsc/regexp/). Specifically this library employs the [Pike VM](http://swtch.com/~rsc/regexp/regexp2.html) algorithm to compile and process regular expressions.

The library aims to cover a subset of the ones available for the subset of PCRE implemented in the [C++ RE2 library](https://re2.googlecode.com/hg/doc/syntax.html), namely:

  * Consistent escaping rules
  * Extended character classes
  * Minimal matching (a.k.a. "ungreedy")
  * Unicode character properties
  * Full character folding
  * Multiline matching
  * Newline/linebreak options
  * Named subpatterns
  * Comments

The library aims to provide an interface and suite of functions similar to the one available in the [Python regular expression library](http://docs.python.org/2/library/re.html#module-contents).

## Compiling Regular Expressions

In order for regular expressions to be used, they must first be parsed into expressions, then compiled into instructions that can be executed by the underlying ```PikeVM``` virtual machine. This is done by executing the following code (example using regexp ```a+b+```):

```rust
let regexp = match UncompiledRegexp::new("a+b+") {
		Ok(regex) => regex,
		Err(e) => fail!(e)
	};
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
let regexp = match UncompiledRegexp::new("a+b+") {
		Ok(regex) => regex,
		Err(e) => fail!(e)
	};
regexp.exec("my test input"); // returns an Option<Match>
```

Under the hood, a new ```PikeVM``` object is created from the instruction list generated during regexp compilation. Next, ```run()``` is called on the resulting object and passed the input string. The Pike VM algorithm runs, generating new tasks for each split.

## Library Functions (API)

The current API for the ```UncompiledRegexp``` class consists of two functions, ```exec()``` and ```search()```, which perform a single match searching from the start of the string and an arbitrary position in the string, respectively. Ultimately, we would like to implment all of the functions that are a part of the [Python re library](http://docs.python.org/2/library/re.html#regular-expression-objects).

Below is a listing of the functions we would like to implement and the progress on each:

  * [```match()```](http://docs.python.org/2/library/re.html#re.RegexObject.match) - *implemented in ```exec()```*
  * [```search()```](http://docs.python.org/2/library/re.html#re.RegexObject.search) - *implemented*
  * [```split()```](http://docs.python.org/2/library/re.html#re.RegexObject.split) - *not implemented*
  * [```find_all()```](http://docs.python.org/2/library/re.html#re.RegexObject.findall) - *not implemented*
  * [```find_iter()```](http://docs.python.org/2/library/re.html#re.RegexObject.finditer) - *not implemented*
  * [```replace()```](http://docs.python.org/2/library/re.html#re.RegexObject.sub) - *not implemented*
  * [```replacen()```](http://docs.python.org/2/library/re.html#re.RegexObject.subn) - *not implemented*

## Testing

The most reliable way to determine whether specific features of this implementation are working is by running the testcases associated with the module. The testcases are autogenerated from the ```cases.py``` file by ```test_generator.py``` in ```src/test```. The resulting testcases are located in ```src/re/test.rs```.

To run the tests, simply execute the following command while in the root repository directory

```bash
make test
```

## Other Repositories

We are not the only game in town. In particular, take a look at the following repositories that are also working to implement regular expressions in Rust:

  * [rose](https://github.com/lfairy/rose)
  * [rust-re](https://github.com/glennsl/rust-re)

There are also a couple of bindings to regular expression libraries from other languages available. A couple that we have come across are listed below:

  * [rust-re2](https://github.com/nickdesaulniers/rust-re2)
  * [rust-pcre](https://github.com/uasi/rust-pcre)

## Codebase upgrade to support Rust Version 0.10 & 0.11-Nightly
There is a huge paradigm shift in the Rust API in the new release. Below you will find my notes on what I had to change and what are issues for us moving forward. Those marked with **[APPARENT]** are unconfirmed changes. AKA, we are not Rust Devs and no public discussion of this change was made, or I haven't discovered the new way to do it. Either is likely.

Understand that you **must** now be on rustc version 0.10 or newer. Otherwise your compiler will have a *field day*....

  * **Overriding implementations is now forbidden**. We have several To_Str overrides which now throw compile errors. All references to the to_str method have now been rewritten to fmts which have an implementation for to_str. **[APPARENT]**
  * **All Struct fields are by default private**. Before we declared certain fields to be private, and others public. Now its reversed, there are pub qualifiers next to those that we need. The rest are private **[APPARENT]**
  * **Extra is now depreceated**. Yes you read right. Please check the 0.10 [Docs](http://static.rust-lang.org/doc/master/index.html). You will discover that much of std has been refactored. About a dozen other libraries have now apparantly been created from the pieces of Extra.
  * **Vector has been overhauled**. The old class is no more, most of the inherited vector classes are now split between str::vec and vec::Vec. Please be careful when using vectors and ensure you are using the right one. The general one is std::vec::Vec now.
  * **Most functions return Options now in std**. This is especially important. Vector for instance will return an Option now for functions like shift() or pop(). There used to be shift_opt() that did the same thing but now its the only way to get a value back. So you have to check to make sure you handle the Option. **Note** Brian finds this extremely annoying.

## External Library Compilation
Now that the codebase is running on 0.10 and 0.11, using our library externally is a breeze. Not that documentation made it easy to find, but below you will find all that you need!

To turn our codebase into a Rust Library .rlib, execute the following. This is already done in our makefile. This is for your reference only.

```bash
rustc --crate-type=lib path/to/lib.rs
```

Please note that lib.rs is incredibly important. It names our library for other rust files to include.

To use our newly compiled library, execute the following:

```bash
rustc /path/to/file_compiling.rs -L ./path/to/our_library.rlib
```

In your file_compiling.rs file, indicate use by extern crate rustre;

## Benchmarking
Our benchmarking suite is designed to be user flexible. There are two compilations essentially. There is a cases.py file in the benchmark directory that is similar to the one found in the test directory. Benchmarking tests performance, so there is no checking if its correct or not. The format of the file will change over the writing of this document...

The first stage of compilation will compile all of the test cases into each of the benchmarks to be run. Then the second stage will compile each benchmark into the build directory. In the second stage, the benchmark C++ application will be compiled and placed in the build directory as **run_benchmark**.

###Benchmarking Languages Supported:

  * Rust
  * C++11 Built in Regex library

###Benchmarks Performed:
  1. Generic Parse/Execute Loop
    * In this first benchmark, each program is compiled with all test cases and will loop a certain number of times as decided by the cases file. Each test case in each loop will create a new Regex and thus will compile each and every time. This is a generic first forray to test general performance for worst cases/bad programmers.
