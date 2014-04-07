#!/usr/bin/env python
#
# Copyright 2011-2013 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

# This digests UnicodeData.txt and DerivedCoreProperties.txt and emits rust
# code covering the core properties. Since this is a pretty rare event we
# just store this out-of-line and check the unicode.rs file into git.
#
# The emitted code is "the minimum we think is necessary for libstd", that
# is, to support basic operations of the compiler and "most nontrivial rust
# programs". It is not meant to be a complete implementation of unicode.
# For that we recommend you use a proper binding to libicu.

import fileinput, re, os, sys


def fetch(f):
    if not os.path.exists(f):
        os.system("curl -O http://www.unicode.org/Public/6.3.0/ucd/%s"
                  % f)

    if not os.path.exists(f):
        sys.stderr.write("cannot load %s" % f)
        exit(1)

def load_unicode_data(f):
    fetch(f)
    gencats = {}
    upperlower = {}
    lowerupper = {}
    combines = []
    canon_decomp = {}
    compat_decomp = {}
    curr_cat = ""
    curr_combine = ""
    c_lo = 0
    c_hi = 0
    com_lo = 0
    com_hi = 0

    for line in fileinput.input(f):
        fields = line.split(";")
        if len(fields) != 15:
            continue
        [code, name, gencat, combine, bidi,
         decomp, deci, digit, num, mirror,
         old, iso, upcase, lowcase, titlecase] = fields

        if gencat == "Cs":
            continue

        code = int(code, 16)

        if curr_cat == "":
            curr_cat = gencat
            c_lo = code
            c_hi = code

        if curr_cat == gencat:
            c_hi = code
        else:
            if curr_cat not in gencats:
                gencats[curr_cat] = []

            gencats[curr_cat].append((c_lo, c_hi))
            curr_cat = gencat
            c_lo = code
            c_hi = code

        if curr_combine == "":
            curr_combine = combine
            com_lo = code
            com_hi = code

        if curr_combine == combine:
            com_hi = code
        else:
            if curr_combine != "0":
                combines.append((com_lo, com_hi, curr_combine))
            curr_combine = combine
            com_lo = code
            com_hi = code

    return gencats

def load_properties(f, interestingprops):
    fetch(f)
    props = {}
    re1 = re.compile("^([0-9A-F]+) +; (\w+)")
    re2 = re.compile("^([0-9A-F]+)\.\.([0-9A-F]+) +; (\w+)")

    for line in fileinput.input(f):
        prop = None
        d_lo = 0
        d_hi = 0
        m = re1.match(line)
        if m:
            d_lo = m.group(1)
            d_hi = m.group(1)
            prop = m.group(2)
        else:
            m = re2.match(line)
            if m:
                d_lo = m.group(1)
                d_hi = m.group(2)
                prop = m.group(3)
            else:
                continue
        if prop not in interestingprops:
            continue
        d_lo = int(d_lo, 16)
        d_hi = int(d_hi, 16)
        if prop not in props:
            props[prop] = []
        props[prop].append((d_lo, d_hi))
    return props

def escape_char(c):
    if c <= 0xff:
        return "'\\x%2.2x'" % c
    if c <= 0xffff:
        return "'\\u%4.4x'" % c
    return "'\\U%8.8x'" % c

def ch_prefix(ix):
    if ix == 0:
        return "        "
    if ix % 2 == 0:
        return ",\n        "
    else:
        return ", "

def emit_bsearch_range_table(f):
    f.write("""
pub fn bsearch_range_table(c: char, r: &'static [(char,char)]) -> bool {
    use std::cmp::{Equal, Less, Greater};
    use std::slice::ImmutableVector;
    use std::option::None;
    r.bsearch(|&(lo,hi)| {
        if lo <= c && c <= hi { Equal }
        else if hi < c { Less }
        else { Greater }
    }) != None
}\n\n
""");

def emit_property_module(f, mod, tbl):
    f.write("pub mod %s {\n" % mod)
    keys = tbl.keys()
    keys.sort()

    f.write("    pub fn get_prop_table(prop: &str) -> Option<&'static [(char,char)]> {\n")
    f.write("        match prop {\n")
    for cat in keys:
        f.write("            &\"%s\" => Some(%s_table),\n" % (cat, cat))
    f.write("            _ => None\n")
    f.write("        }\n")
    f.write("    }\n\n")

    for cat in keys:
        f.write("    pub static %s_table : &'static [(char,char)] = &[\n" % cat)
        ix = 0
        for pair in tbl[cat]:
            f.write(ch_prefix(ix))
            f.write("(%s, %s)" % (escape_char(pair[0]), escape_char(pair[1])))
            ix += 1
        f.write("\n    ];\n\n")
    f.write("}\n")

r = "../re/unicode.rs"
for i in [r]:
    if os.path.exists(i):
        os.remove(i);
rf = open(r, "w")

# Preamble
rf.write('''// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// The following code was generated by "src/unicode/unicode.py"

#[allow(missing_doc)];
#[allow(non_uppercase_statics)];

''')

emit_bsearch_range_table(rf)

gencats = load_unicode_data("UnicodeData.txt")
emit_property_module(rf, "general_category", gencats)

script = load_properties("Scripts.txt",
    ["Arabic", "Armenian", "Balinese", "Bengali", "Bopomofo", "Braille", "Buginese",
     "Buhid", "Canadian_Aboriginal", "Carian", "Cham", "Cherokee", "Common", "Coptic",
     "Cuneiform", "Cypriot", "Cyrillic", "Deseret", "Devanagari", "Ethiopic", "Georgian",
     "Glagolitic", "Gothic", "Greek", "Gujarati", "Gurmukhi", "Han", "Hangul", "Hanunoo",
     "Hebrew", "Hiragana", "Inherited", "Kannada", "Katakana", "Kayah_Li", "Kharoshthi",
     "Khmer", "Lao", "Latin", "Lepcha", "Limbu", "Linear_B", "Lycian", "Lydian", "Malayalam",
     "Mongolian", "Myanmar", "New_Tai_Lue", "Nko", "Ogham", "Ol_Chiki", "Old_Italic", "Old_Persian",
     "Oriya", "Osmanya", "Phags_Pa", "Phoenician", "Rejang", "Runic", "Saurashtra", "Shavian",
     "Sinhala", "Sundanese", "Syloti_Nagri", "Syriac", "Tagalog", "Tagbanwa", "Tai_Le",
     "Tamil", "Telugu", "Thaana", "Thai", "Tibetan", "Tifinagh", "Ugaritic", "Vai", "Yi"]);
emit_property_module(rf, "script", script)

rf.write('''
#[cfg(test)]
mod unicode_tests {
    use super::*;

    #[test]
    fn test_general_property_contains() {
        assert!(bsearch_range_table('\uabf8', general_category::get_prop_table(&"Nd").unwrap()));
    }

    #[test]
    fn test_general_property_doesnt_contain() {
        assert!(!bsearch_range_table('\uabfa', general_category::get_prop_table(&"Nd").unwrap()));
    }

    #[test]
    fn test_general_property_doesnt_exist() {
        assert_eq!(general_category::get_prop_table(&"A"), None);
    }

    #[test]
    fn test_script_contains() {
        assert!(bsearch_range_table('\u1f39', script::get_prop_table(&"Greek").unwrap()));
    }

    #[test]
    fn test_script_doesnt_contain() {
        assert!(!bsearch_range_table('\u1f58', script::get_prop_table(&"Greek").unwrap()));
    }

    #[test]
    fn test_script_doesnt_exist() {
      assert_eq!(general_category::get_prop_table(&"A"), None);
    }
}
''')
