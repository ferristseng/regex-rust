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
