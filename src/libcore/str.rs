/*
Module: str

String manipulation

Strings are a packed UTF-8 representation of text, stored as null terminated
buffers of u8 bytes.  Strings should be indexed in bytes, for efficiency,
but UTF-8 unsafe operations should be avoided.
For some heavy-duty uses, try std::rope.
*/

export
   // Creating a string
   from_bytes,
   from_byte,
   push_char,
   from_char,
   from_chars,
   from_cstr,
   from_cstr_len,
   concat,
   connect,

   // Adding things to and removing things from a string
   push_char,
   pop_char,
   shift_char,
   unshift_char,
   trim_left,
   trim_right,
   trim,

   // Transforming strings
   bytes,
   chars,
   substr,
   slice,
   split, splitn, split_nonempty,
   split_char, splitn_char, split_char_nonempty,
   split_str, split_str_nonempty,
   lines,
   lines_any,
   words,
   to_lower,
   to_upper,
   replace,

   // Comparing strings
   eq,
   le,
   hash,

   // Iterating through strings
   all, any,
   all_between, any_between,
   map,
   bytes_iter,
   chars_iter,
   split_char_iter,
   splitn_char_iter,
   words_iter,
   lines_iter,

   // Searching
   find, find_from, find_between,
   rfind, rfind_from, rfind_between,
   find_char, find_char_from, find_char_between,
   rfind_char, rfind_char_from, rfind_char_between,
   find_str, find_str_from, find_str_between,
   contains,
   starts_with,
   ends_with,

   // String properties
   is_ascii,
   is_empty,
   is_not_empty,
   is_whitespace,
   len,
   char_len,

   // Misc
   is_utf8,
   is_utf16,
   to_utf16,
   from_utf16,
   utf16_chars,
   count_chars, count_bytes,
   utf8_char_width,
   char_range_at,
   is_char_boundary,
   char_at,
   as_bytes,
   as_buf,
   sbuf,
   reserve,

   unsafe;


#[abi = "cdecl"]
native mod rustrt {
    fn rust_str_push(&s: str, ch: u8);
    fn str_reserve_shared(&ss: str, nn: ctypes::size_t);
}

// FIXME: add pure to a lot of functions

/*
Section: Creating a string
*/

/*
Function: from_bytes

Convert a vector of bytes to a UTF-8 string.  Fails if invalid UTF-8.
*/
fn from_bytes(vv: [u8]) -> str unsafe {
   assert is_utf8(vv);
   ret unsafe::from_bytes(vv);
}

/*
Function: from_byte

Convert a byte to a UTF-8 string.  Fails if invalid UTF-8.
*/
fn from_byte(b: u8) -> str unsafe {
    assert b < 128u8;
    let mut v = [b, 0u8];
    let s: str = ::unsafe::reinterpret_cast(v);
    ::unsafe::leak(v);
    s
}

/*
Function: push_char

Appends a character at the end of a string.
*/
fn push_char(&s: str, ch: char) unsafe {
    let code = ch as uint;
    if code < max_one_b {
        rustrt::rust_str_push(s, code as u8);
    } else if code < max_two_b {
        rustrt::rust_str_push(s, (code >> 6u & 31u | tag_two_b) as u8);
        rustrt::rust_str_push(s, (code & 63u | tag_cont) as u8);
    } else if code < max_three_b {
        rustrt::rust_str_push(s, (code >> 12u & 15u | tag_three_b) as u8);
        rustrt::rust_str_push(s, (code >> 6u & 63u | tag_cont) as u8);
        rustrt::rust_str_push(s, (code & 63u | tag_cont) as u8);
    } else if code < max_four_b {
        rustrt::rust_str_push(s, (code >> 18u & 7u | tag_four_b) as u8);
        rustrt::rust_str_push(s, (code >> 12u & 63u | tag_cont) as u8);
        rustrt::rust_str_push(s, (code >> 6u & 63u | tag_cont) as u8);
        rustrt::rust_str_push(s, (code & 63u | tag_cont) as u8);
    } else if code < max_five_b {
        rustrt::rust_str_push(s, (code >> 24u & 3u | tag_five_b) as u8);
        rustrt::rust_str_push(s, (code >> 18u & 63u | tag_cont) as u8);
        rustrt::rust_str_push(s, (code >> 12u & 63u | tag_cont) as u8);
        rustrt::rust_str_push(s, (code >> 6u & 63u | tag_cont) as u8);
        rustrt::rust_str_push(s, (code & 63u | tag_cont) as u8);
    } else {
        rustrt::rust_str_push(s, (code >> 30u & 1u | tag_six_b) as u8);
        rustrt::rust_str_push(s, (code >> 24u & 63u | tag_cont) as u8);
        rustrt::rust_str_push(s, (code >> 18u & 63u | tag_cont) as u8);
        rustrt::rust_str_push(s, (code >> 12u & 63u | tag_cont) as u8);
        rustrt::rust_str_push(s, (code >> 6u & 63u | tag_cont) as u8);
        rustrt::rust_str_push(s, (code & 63u | tag_cont) as u8);
    }
}

/*
Function: from_char

Convert a char to a string
*/
fn from_char(ch: char) -> str {
    let mut buf = "";
    push_char(buf, ch);
    ret buf;
}

/*
Function: from_chars

Convert a vector of chars to a string
*/
fn from_chars(chs: [char]) -> str {
    let mut buf = "";
    reserve(buf, chs.len());
    for ch in chs { push_char(buf, ch); }
    ret buf;
}

/*
Function: from_cstr

Create a Rust string from a null-terminated C string
*/
fn from_cstr(cstr: sbuf) -> str unsafe {
    let mut curr = cstr, i = 0u;
    while *curr != 0u8 {
        i += 1u;
        curr = ptr::offset(cstr, i);
    }
    ret from_cstr_len(cstr, i);
}

/*
Function: from_cstr_len

Create a Rust string from a C string of the given length
*/
fn from_cstr_len(cstr: sbuf, len: uint) -> str unsafe {
    let mut buf: [u8] = [];
    vec::reserve(buf, len + 1u);
    vec::as_buf(buf) {|b| ptr::memcpy(b, cstr, len); }
    vec::unsafe::set_len(buf, len);
    buf += [0u8];

    assert is_utf8(buf);
    let s: str = ::unsafe::reinterpret_cast(buf);
    ::unsafe::leak(buf);
    ret s;
}

/*
Function: concat

Concatenate a vector of strings
*/
fn concat(v: [str]) -> str {
    let mut s: str = "";
    for ss: str in v { s += ss; }
    ret s;
}

/*
Function: connect

Concatenate a vector of strings, placing a given separator between each
*/
fn connect(v: [str], sep: str) -> str {
    let mut s = "", first = true;
    for ss: str in v {
        if first { first = false; } else { s += sep; }
        s += ss;
    }
    ret s;
}

/*
Section: Adding to and removing from a string
*/

/*
Function: pop_char

Remove the final character from a string and return it.

Failure:
If the string does not contain any characters.
*/
fn pop_char(&s: str) -> char {
    let end = len(s);
    assert end > 0u;
    let {ch, prev} = char_range_at_reverse(s, end);
    unsafe { unsafe::set_len(s, prev); }
    ret ch;
}

/*
Function: shift_char

Remove the first character from a string and return it.

Failure:

If the string does not contain any characters.
*/
fn shift_char(&s: str) -> char unsafe {
    let {ch, next} = char_range_at(s, 0u);
    s = unsafe::slice_bytes(s, next, len(s));
    ret ch;
}

/*
Function: unshift_char

Prepend a char to a string
*/
fn unshift_char(&s: str, ch: char) { s = from_char(ch) + s; }

/*
Function: trim_left

Returns a string with leading whitespace removed.
*/
fn trim_left(+s: str) -> str {
    alt find(s, {|c| !char::is_whitespace(c)}) {
      none { "" }
      some(first) {
        if first == 0u { s }
        else unsafe { unsafe::slice_bytes(s, first, len(s)) }
      }
    }
}

/*
Function: trim_right

Returns a string with trailing whitespace removed.
*/
fn trim_right(+s: str) -> str {
    alt rfind(s, {|c| !char::is_whitespace(c)}) {
      none { "" }
      some(last) {
        let {next, _} = char_range_at(s, last);
        if next == len(s) { s }
        else unsafe { unsafe::slice_bytes(s, 0u, next) }
      }
    }
}

/*
Function: trim

Returns a string with leading and trailing whitespace removed
*/
fn trim(+s: str) -> str { trim_left(trim_right(s)) }

/*
Section: Transforming strings
*/

/*
Function: bytes

Converts a string to a vector of bytes. The result vector is not
null-terminated.
*/
fn bytes(s: str) -> [u8] unsafe {
    as_bytes(s) { |v| vec::slice(v, 0u, vec::len(v) - 1u) }
}

/*
Function: chars

Convert a string to a vector of characters
*/
fn chars(s: str) -> [char] {
    let mut buf = [], i = 0u;
    let len = len(s);
    while i < len {
        let {ch, next} = char_range_at(s, i);
        buf += [ch];
        i = next;
    }
    ret buf;
}

/*
Function: substr

Take a substring of another. Returns a string containing `n`
characters starting at byte offset `begin`.
*/
fn substr(s: str, begin: uint, n: uint) -> str {
    slice(s, begin, begin + count_bytes(s, begin, n))
}

// Function: slice
//
// Return a slice of the given string from the byte range [`begin`..`end`)
// or else fail when `begin` and `end` do not point to valid characters or
// beyond the last character of the string
fn slice(s: str, begin: uint, end: uint) -> str unsafe {
    assert is_char_boundary(s, begin);
    assert is_char_boundary(s, end);
    unsafe::slice_bytes(s, begin, end)
}

// Function: split_char
//
// Splits a string into substrings at each occurrence of a given
// character
fn split_char(s: str, sep: char) -> [str] {
    split_char_inner(s, sep, len(s), true)
}

// Function: splitn_char
//
// Splits a string into substrings at each occurrence of a given
// character up to 'count' times
//
// The byte must be a valid UTF-8/ASCII byte
fn splitn_char(s: str, sep: char, count: uint) -> [str] {
    split_char_inner(s, sep, count, true)
}

// Function: split_char_nonempty
//
// Like `split_char`, but omits empty strings from the returned vector.
fn split_char_nonempty(s: str, sep: char) -> [str] {
    split_char_inner(s, sep, len(s), false)
}

fn split_char_inner(s: str, sep: char, count: uint, allow_empty: bool)
    -> [str] unsafe {
    if sep < 128u as char {
        let b = sep as u8, l = len(s);
        let mut result = [], done = 0u;
        let mut i = 0u, start = 0u;
        while i < l && done < count {
            if s[i] == b {
                if allow_empty || start < i {
                    result += [unsafe::slice_bytes(s, start, i)];
                }
                start = i + 1u;
                done += 1u;
            }
            i += 1u;
        }
        if allow_empty || start < l {
            result += [unsafe::slice_bytes(s, start, l)];
        }
        result
    } else {
        splitn(s, {|cur| cur == sep}, count)
    }
}


/*
Function: split

Splits a string into substrings using a character function
*/
fn split(s: str, sepfn: fn(char) -> bool) -> [str] {
    split_inner(s, sepfn, len(s), true)
}

/*
Function: splitn

Splits a string into substrings using a character function, cutting at
most [count] times.
*/
fn splitn(s: str, sepfn: fn(char) -> bool, count: uint) -> [str] {
    split_inner(s, sepfn, count, true)
}

// Function: split_nonempty
//
// Like `split`, but omits empty strings from the returned vector.
fn split_nonempty(s: str, sepfn: fn(char) -> bool) -> [str] {
    split_inner(s, sepfn, len(s), false)
}

fn split_inner(s: str, sepfn: fn(cc: char) -> bool, count: uint,
               allow_empty: bool) -> [str] unsafe {
    let l = len(s);
    let mut result = [], i = 0u, start = 0u, done = 0u;
    while i < l && done < count {
        let {ch, next} = char_range_at(s, i);
        if sepfn(ch) {
            if allow_empty || start < i {
                result += [unsafe::slice_bytes(s, start, i)];
            }
            start = next;
            done += 1u;
        }
        i = next;
    }
    if allow_empty || start < l {
        result += [unsafe::slice_bytes(s, start, l)];
    }
    result
}

// FIXME use Boyer-Moore
fn iter_matches(s: str, sep: str, f: fn(uint, uint)) {
    let sep_len = len(sep), l = len(s);
    assert sep_len > 0u;
    let mut i = 0u, match_start = 0u, match_i = 0u;

    while i < l {
        if s[i] == sep[match_i] {
            if match_i == 0u { match_start = i; }
            match_i += 1u;
            // Found a match
            if match_i == sep_len {
                f(match_start, i + 1u);
                match_i = 0u;
            }
            i += 1u;
        } else {
            // Failed match, backtrack
            if match_i > 0u {
                match_i = 0u;
                i = match_start + 1u;
            } else {
                i += 1u;
            }
        }
    }
}

fn iter_between_matches(s: str, sep: str, f: fn(uint, uint)) {
    let mut last_end = 0u;
    iter_matches(s, sep) {|from, to|
        f(last_end, from);
        last_end = to;
    }
    f(last_end, len(s));
}

/*
Function: split_str

Splits a string into a vector of the substrings separated by a given string

Note that this has recently been changed.  For example:
>  assert ["", "XXX", "YYY", ""] == split_str(".XXX.YYY.", ".")
*/
fn split_str(s: str, sep: str) -> [str] {
    let mut result = [];
    iter_between_matches(s, sep) {|from, to|
        unsafe { result += [unsafe::slice_bytes(s, from, to)]; }
    }
    result
}

fn split_str_nonempty(s: str, sep: str) -> [str] {
    let mut result = [];
    iter_between_matches(s, sep) {|from, to|
        if to > from {
            unsafe { result += [unsafe::slice_bytes(s, from, to)]; }
        }
    }
    result
}

/*
Function: lines

Splits a string into a vector of the substrings
separated by LF ('\n')
*/
fn lines(s: str) -> [str] { split_char(s, '\n') }

/*
Function: lines_any

Splits a string into a vector of the substrings
separated by LF ('\n') and/or CR LF ('\r\n')
*/
fn lines_any(s: str) -> [str] {
    vec::map(lines(s), {|s|
        let l = len(s);
        let mut cp = s;
        if l > 0u && s[l - 1u] == '\r' as u8 {
            unsafe { unsafe::set_len(cp, l - 1u); }
        }
        cp
    })
}

/*
Function: words

Splits a string into a vector of the substrings
separated by whitespace
*/
fn words(s: str) -> [str] {
    split_nonempty(s, {|c| char::is_whitespace(c)})
}

/*
Function: to_lower

Convert a string to lowercase
*/
fn to_lower(s: str) -> str {
    map(s, char::to_lower)
}

/*
Function: to_upper

Convert a string to uppercase
*/
fn to_upper(s: str) -> str {
    map(s, char::to_upper)
}

/*
Function: replace

Replace all occurances of one string with another

Parameters:

s - The string containing substrings to replace
from - The string to replace
to - The replacement string

Returns:

The original string with all occurances of `from` replaced with `to`
*/
fn replace(s: str, from: str, to: str) -> str unsafe {
    let mut result = "", first = true;
    iter_between_matches(s, from) {|start, end|
        if first { first = false; } else { result += to; }
        unsafe { result += unsafe::slice_bytes(s, start, end); }
    }
    result
}

/*
Section: Comparing strings
*/

/*
Function: eq

Bytewise string equality
*/
pure fn eq(&&a: str, &&b: str) -> bool { a == b }

/*
Function: le

Bytewise less than or equal
*/
pure fn le(&&a: str, &&b: str) -> bool { a <= b }

/*
Function: hash

String hash function
*/
fn hash(&&s: str) -> uint {
    // djb hash.
    // FIXME: replace with murmur.
    let mut u: uint = 5381u;
    for c: u8 in s { u *= 33u; u += c as uint; }
    ret u;
}

/*
Section: Iterating through strings
*/

/*
Function: all

Return true if a predicate matches all characters or
if the string contains no characters
*/
fn all(s: str, it: fn(char) -> bool) -> bool {
    all_between(s, 0u, len(s), it)
}

/*
Function: any

Return true if a predicate matches any character
(and false if it matches none or there are no characters)
*/
fn any(ss: str, pred: fn(char) -> bool) -> bool {
    !all(ss, {|cc| !pred(cc)})
}

/*
Function: map

Apply a function to each character
*/
fn map(ss: str, ff: fn(char) -> char) -> str {
    let mut result = "";
    reserve(result, len(ss));
    chars_iter(ss) {|cc| str::push_char(result, ff(cc));}
    result
}

/*
Function: bytes_iter

Iterate over the bytes in a string
*/
fn bytes_iter(ss: str, it: fn(u8)) {
    let mut pos = 0u;
    let len = len(ss);

    while (pos < len) {
        it(ss[pos]);
        pos += 1u;
    }
}

/*
Function: chars_iter

Iterate over the characters in a string
*/
fn chars_iter(s: str, it: fn(char)) {
    let mut pos = 0u;
    let len = len(s);
    while (pos < len) {
        let {ch, next} = char_range_at(s, pos);
        pos = next;
        it(ch);
    }
}

/*
Function: split_char_iter

Apply a function to each substring after splitting
by character
*/
fn split_char_iter(ss: str, cc: char, ff: fn(&&str)) {
   vec::iter(split_char(ss, cc), ff)
}

/*
Function: splitn_char_iter

Apply a function to each substring after splitting
by character, up to `count` times
*/
fn splitn_char_iter(ss: str, sep: char, count: uint, ff: fn(&&str)) unsafe {
   vec::iter(splitn_char(ss, sep, count), ff)
}

/*
Function: words_iter

Apply a function to each word
*/
fn words_iter(ss: str, ff: fn(&&str)) {
    vec::iter(words(ss), ff)
}

/*
Function: lines_iter

Apply a function to each lines (by '\n')
*/
fn lines_iter(ss: str, ff: fn(&&str)) {
    vec::iter(lines(ss), ff)
}

/*
Section: Searching
*/

// Function: find_char
//
// Returns the byte index of the first matching char
// (as option some/none)
fn find_char(s: str, c: char) -> option<uint> {
    find_char_between(s, c, 0u, len(s))
}

// Function: find_char_from
//
// Returns the byte index of the first matching char
// (as option some/none), starting from `start`
fn find_char_from(s: str, c: char, from: uint) -> option<uint> {
    find_char_between(s, c, from, len(s))
}

// Function: find_char_between
//
// Returns the byte index of the first matching char
// (as option some/none), between `start` and `end`
fn find_char_between(s: str, c: char, start: uint, end: uint)
    -> option<uint> {
    if c < 128u as char {
        assert start <= end;
        assert end <= len(s);
        let mut i = start;
        let b = c as u8;
        while i < end {
            if s[i] == b { ret some(i); }
            i += 1u;
        }
        ret none;
    } else {
        find_between(s, start, end, {|x| x == c})
    }
}

// Function: rfind_char
//
// Returns the byte index of the last matching char
// (as option some/none)
fn rfind_char(s: str, c: char) -> option<uint> {
    rfind_char_between(s, c, len(s), 0u)
}

// Function: rfind_char_from
//
// Returns the byte index of the last matching char
// (as option some/none), starting from `start`
fn rfind_char_from(s: str, c: char, start: uint) -> option<uint> {
    rfind_char_between(s, c, start, 0u)
}

// Function: rfind_char_between
//
// Returns the byte index of the last matching char (as option
// some/none), between from `start` and `end` (start must be greater
// than or equal to end)
fn rfind_char_between(s: str, c: char, start: uint, end: uint)
    -> option<uint> {
    if c < 128u as char {
        assert start >= end;
        assert start <= len(s);
        let mut i = start;
        let b = c as u8;
        while i > end {
            i -= 1u;
            if s[i] == b { ret some(i); }
        }
        ret none;
    } else {
        rfind_between(s, start, end, {|x| x == c})
    }
}

// Function: find
//
// Returns, as an option, the first character that passes the given
// predicate
fn find(s: str, f: fn(char) -> bool) -> option<uint> {
    find_between(s, 0u, len(s), f)
}

// Function: find_from
//
// Returns, as an option, the first character that passes the given
// predicate, starting at byte offset `start`
fn find_from(s: str, start: uint, f: fn(char) -> bool) -> option<uint> {
    find_between(s, start, len(s), f)
}

// Function: find_between
//
// Returns, as an option, the first character that passes the given
// predicate, between byte offsets `start` and `end`
fn find_between(s: str, start: uint, end: uint, f: fn(char) -> bool)
    -> option<uint> {
    assert start <= end;
    assert end <= len(s);
    assert is_char_boundary(s, start);
    let mut i = start;
    while i < end {
        let {ch, next} = char_range_at(s, i);
        if f(ch) { ret some(i); }
        i = next;
    }
    ret none;
}

// Function: rfind
//
// Returns, as an option, the last character in the string that passes
// the given predicate
fn rfind(s: str, f: fn(char) -> bool) -> option<uint> {
    rfind_between(s, len(s), 0u, f)
}

// Function: rfind_from
//
// Returns, as an option, the last character that passes the given
// predicate, up until byte offset `start`
fn rfind_from(s: str, start: uint, f: fn(char) -> bool) -> option<uint> {
    rfind_between(s, start, 0u, f)
}

// Function: rfind_between
//
// Returns, as an option, the last character that passes the given
// predicate, between byte offsets `start` and `end` (`start` must be
// greater than or equal to `end`)
fn rfind_between(s: str, start: uint, end: uint, f: fn(char) -> bool)
    -> option<uint> {
    assert start >= end;
    assert start <= len(s);
    assert is_char_boundary(s, start);
    let mut i = start;
    while i > end {
        let {ch, prev} = char_range_at_reverse(s, i);
        if f(ch) { ret some(prev); }
        i = prev;
    }
    ret none;
}

// Utility used by various searching functions
fn match_at(haystack: str, needle: str, at: uint) -> bool {
    let mut i = at;
    for c in needle { if haystack[i] != c { ret false; } i += 1u; }
    ret true;
}

//Function: find_str
//
// Find the byte position of the first instance of one string
// within another, or return option::none
fn find_str(haystack: str, needle: str) -> option<uint> {
    find_str_between(haystack, needle, 0u, len(haystack))
}

//Function: find_str_from
//
// Find the byte position of the first instance of one string
// within another, or return option::none
fn find_str_from(haystack: str, needle: str, start: uint)
  -> option<uint> {
    find_str_between(haystack, needle, start, len(haystack))
}

//Function: find_str_between
//
// Find the byte position of the first instance of one string
// within another, or return option::none
fn find_str_between(haystack: str, needle: str, start: uint, end:uint)
  -> option<uint> {
    // FIXME: Boyer-Moore should be significantly faster
    assert end <= len(haystack);
    let needle_len = len(needle);
    if needle_len == 0u { ret some(start); }
    if needle_len > end { ret none; }

    let mut i = start;
    let e = end - needle_len;
    while i <= e {
        if match_at(haystack, needle, i) { ret some(i); }
        i += 1u;
    }
    ret none;
}

/*
Function: contains

Returns true if one string contains another

Parameters:

haystack - The string to look in
needle - The string to look for
*/
fn contains(haystack: str, needle: str) -> bool {
    option::is_some(find_str(haystack, needle))
}

/*
Function: starts_with

Returns true if one string starts with another

Parameters:

haystack - The string to look in
needle - The string to look for
*/
fn starts_with(haystack: str, needle: str) -> bool unsafe {
    let haystack_len = len(haystack), needle_len = len(needle);
    if needle_len == 0u { true }
    else if needle_len > haystack_len { false }
    else { match_at(haystack, needle, 0u) }
}

/*
Function: ends_with

Returns true if one string ends with another

haystack - The string to look in
needle - The string to look for
*/
fn ends_with(haystack: str, needle: str) -> bool {
    let haystack_len = len(haystack), needle_len = len(needle);
    if needle_len == 0u { true }
    else if needle_len > haystack_len { false }
    else { match_at(haystack, needle, haystack_len - needle_len) }
}

/*
Section: String properties
*/

/*
Function: is_ascii

Determines if a string contains only ASCII characters
*/
fn is_ascii(s: str) -> bool {
    let mut i: uint = len(s);
    while i > 0u { i -= 1u; if !u8::is_ascii(s[i]) { ret false; } }
    ret true;
}

/*
Predicate: is_empty

Returns true if the string has length 0
*/
pure fn is_empty(s: str) -> bool { for c: u8 in s { ret false; } ret true; }

/*
Predicate: is_not_empty

Returns true if the string has length greater than 0
*/
pure fn is_not_empty(s: str) -> bool { !is_empty(s) }

/*
Function: is_whitespace

Returns true if the string contains only whitespace
*/
fn is_whitespace(s: str) -> bool {
    ret all(s, char::is_whitespace);
}


// Function: len
//
// Returns the string length/size in bytes
// not counting the null terminator
pure fn len(s: str) -> uint unsafe {
    let repr: *vec::unsafe::vec_repr = ::unsafe::reinterpret_cast(s);
    (*repr).fill - 1u
}

// Function: char_len
//
// Returns the number of characters that a string holds
fn char_len(s: str) -> uint { count_chars(s, 0u, len(s)) }

/*
Section: Misc
*/

/*
Function: is_utf8

Determines if a vector of bytes contains valid UTF-8
*/
fn is_utf8(v: [const u8]) -> bool {
    let mut i = 0u;
    let total = vec::len::<u8>(v);
    while i < total {
        let mut chsize = utf8_char_width(v[i]);
        if chsize == 0u { ret false; }
        if i + chsize > total { ret false; }
        i += 1u;
        while chsize > 1u {
            if v[i] & 192u8 != tag_cont_u8 { ret false; }
            i += 1u;
            chsize -= 1u;
        }
    }
    ret true;
}


fn is_utf16(v: [const u16]) -> bool {
    let len = v.len();
    let mut i = 0u;
    while (i < len) {
        let u = v[i];

        if  u <= 0xD7FF_u16 || u >= 0xE000_u16 {
            i += 1u;

        } else {
            if i+1u < len { ret false; }
            let u2 = v[i+1u];
            if u < 0xD7FF_u16 || u > 0xDBFF_u16 { ret false; }
            if u2 < 0xDC00_u16 || u2 > 0xDFFF_u16 { ret false; }
            i += 2u;
        }
    }
    ret true;
}

fn to_utf16(s: str) -> [u16] {
    let mut u = [];
    chars_iter(s) {|cch|
        // Arithmetic with u32 literals is easier on the eyes than chars.
        let mut ch = cch as u32;

        if (ch & 0xFFFF_u32) == ch {
            // The BMP falls through (assuming non-surrogate, as it should)
            assert ch <= 0xD7FF_u32 || ch >= 0xE000_u32;
            u += [ch as u16]
        } else {
            // Supplementary planes break into surrogates.
            assert ch >= 0x1_0000_u32 && ch <= 0x10_FFFF_u32;
            ch -= 0x1_0000_u32;
            let w1 = 0xD800_u16 | ((ch >> 10) as u16);
            let w2 = 0xDC00_u16 | ((ch as u16) & 0x3FF_u16);
            u += [w1, w2]
        }
    }
    ret u;
}

fn utf16_chars(v: [const u16], f: fn(char)) {
    let len = v.len();
    let mut i = 0u;
    while (i < len && v[i] != 0u16) {
        let mut u = v[i];

        if  u <= 0xD7FF_u16 || u >= 0xE000_u16 {
            f(u as char);
            i += 1u;

        } else {
            let u2 = v[i+1u];
            assert u >= 0xD800_u16 && u <= 0xDBFF_u16;
            assert u2 >= 0xDC00_u16 && u2 <= 0xDFFF_u16;
            let mut c = (u - 0xD800_u16) as char;
            c = c << 10;
            c |= (u2 - 0xDC00_u16) as char;
            c |= 0x1_0000_u32 as char;
            f(c);
            i += 2u;
        }
    }
}


fn from_utf16(v: [const u16]) -> str {
    let mut buf = "";
    reserve(buf, v.len());
    utf16_chars(v) {|ch| push_char(buf, ch); }
    ret buf;
}


/*
Function: count_chars

As char_len but for a slice of a string

Parameters:
 s           - A valid string
 start       - The position inside `s` where to start counting in bytes.
 end         - The position where to stop counting

Returns:
 The number of Unicode characters in `s` between the given indices.
*/
fn count_chars(s: str, start: uint, end: uint) -> uint {
    assert is_char_boundary(s, start);
    assert is_char_boundary(s, end);
    let mut i = start, len = 0u;
    while i < end {
        let {next, _} = char_range_at(s, i);
        len += 1u;
        i = next;
    }
    ret len;
}

// Function count_bytes
//
// Counts the number of bytes taken by the `n` in `s` starting from
// `start`.
fn count_bytes(s: str, start: uint, n: uint) -> uint {
    assert is_char_boundary(s, start);
    let mut end = start, cnt = n;
    let l = len(s);
    while cnt > 0u {
        assert end < l;
        let {next, _} = char_range_at(s, end);
        cnt -= 1u;
        end = next;
    }
    end - start
}

/*
Function: utf8_char_width

Given a first byte, determine how many bytes are in this UTF-8 character

*/
pure fn utf8_char_width(b: u8) -> uint {
    let byte: uint = b as uint;
    if byte < 128u { ret 1u; }
    // Not a valid start byte
    if byte < 192u { ret 0u; }
    if byte < 224u { ret 2u; }
    if byte < 240u { ret 3u; }
    if byte < 248u { ret 4u; }
    if byte < 252u { ret 5u; }
    ret 6u;
}

/*
Function is_char_boundary

Returns false if the index points into the middle of a multi-byte
character sequence.
*/
pure fn is_char_boundary(s: str, index: uint) -> bool {
    if index == len(s) { ret true; }
    let b = s[index];
    ret b < 128u8 || b >= 192u8;
}

/*
Function: char_range_at

Pluck a character out of a string and return the index of the next character.
This function can be used to iterate over the unicode characters of a string.

Example:
> let s = "中华Việt Nam";
> let i = 0u;
> while i < str::len(s) {
>    let {ch, next} = str::char_range_at(s, i);
>    std::io::println(#fmt("%u: %c",i,ch));
>    i = next;
> }

Example output:

      0: 中
      3: 华
      6: V
      7: i
      8: ệ
      11: t
      12:
      13: N
      14: a
      15: m

Parameters:

s - The string
i - The byte offset of the char to extract

Returns:

A record {ch: char, next: uint} containing the char value and the byte
index of the next unicode character.

Failure:

If `i` is greater than or equal to the length of the string.
If `i` is not the index of the beginning of a valid UTF-8 character.
*/
fn char_range_at(s: str, i: uint) -> {ch: char, next: uint} {
    let b0 = s[i];
    let w = utf8_char_width(b0);
    assert (w != 0u);
    if w == 1u { ret {ch: b0 as char, next: i + 1u}; }
    let mut val = 0u;
    let end = i + w;
    let mut i = i + 1u;
    while i < end {
        let byte = s[i];
        assert (byte & 192u8 == tag_cont_u8);
        val <<= 6u;
        val += (byte & 63u8) as uint;
        i += 1u;
    }
    // Clunky way to get the right bits from the first byte. Uses two shifts,
    // the first to clip off the marker bits at the left of the byte, and then
    // a second (as uint) to get it to the right position.
    val += ((b0 << ((w + 1u) as u8)) as uint) << ((w - 1u) * 6u - w - 1u);
    ret {ch: val as char, next: i};
}

/*
Function: char_at

Pluck a character out of a string
*/
fn char_at(s: str, i: uint) -> char { ret char_range_at(s, i).ch; }

// Function: char_range_at_reverse
//
// Given a byte position and a str, return the previous char and its position
// This function can be used to iterate over a unicode string in reverse.
fn char_range_at_reverse(ss: str, start: uint) -> {ch: char, prev: uint} {
    let mut prev = start;

    // while there is a previous byte == 10......
    while prev > 0u && ss[prev - 1u] & 192u8 == tag_cont_u8 {
        prev -= 1u;
    }

    // now refer to the initial byte of previous char
    prev -= 1u;

    let ch = char_at(ss, prev);
    ret {ch:ch, prev:prev};
}

/*
Function: all_between

Loop through a substring, char by char

Parameters:
s           - A string to traverse. It may be empty.
start       - The byte offset at which to start in the string.
end         - The end of the range to traverse
it          - A block to execute with each consecutive character of `s`.
Return `true` to continue, `false` to stop.

Returns:

`true` If execution proceeded correctly, `false` if it was interrupted,
that is if `it` returned `false` at any point.

Safety note:
- This function does not check whether the substring is valid.
- This function fails if `byte_offset` or `byte_len` do not
 represent valid positions inside `s`
 */
fn all_between(s: str, start: uint, end: uint, it: fn(char) -> bool) -> bool {
    assert is_char_boundary(s, start);
    let mut i = start;
    while i < end {
        let {ch, next} = char_range_at(s, i);
        if !it(ch) { ret false; }
        i = next;
    }
    ret true;
}

fn any_between(s: str, start: uint, end: uint, it: fn(char) -> bool) -> bool {
    !all_between(s, start, end, {|c| !it(c)})
}

// UTF-8 tags and ranges
const tag_cont_u8: u8 = 128u8;
const tag_cont: uint = 128u;
const max_one_b: uint = 128u;
const tag_two_b: uint = 192u;
const max_two_b: uint = 2048u;
const tag_three_b: uint = 224u;
const max_three_b: uint = 65536u;
const tag_four_b: uint = 240u;
const max_four_b: uint = 2097152u;
const tag_five_b: uint = 248u;
const max_five_b: uint = 67108864u;
const tag_six_b: uint = 252u;

/*
Function: as_bytes

Work with the byte buffer of a string. Allows for unsafe manipulation
of strings, which is useful for native interop.

Example:

> let i = str::as_bytes("Hello World") { |bytes| vec::len(bytes) };

*/
fn as_bytes<T>(s: str, f: fn([u8]) -> T) -> T unsafe {
    let mut v: [u8] = ::unsafe::reinterpret_cast(s);
    let r = f(v);
    ::unsafe::leak(v);
    r
}

/*
Function: as_buf

Work with the byte buffer of a string. Allows for unsafe manipulation
of strings, which is useful for native interop.

Example:

> let s = str::as_buf("PATH", { |path_buf| libc::getenv(path_buf) });

*/
fn as_buf<T>(s: str, f: fn(sbuf) -> T) -> T unsafe {
    as_bytes(s) { |v| vec::as_buf(v, f) }
}

/*
Type: sbuf

An unsafe buffer of bytes.
*/
type sbuf = *u8;

// Function: reserve
//
// Allocate more memory for a string, up to `nn` + 1 bytes
fn reserve(&ss: str, nn: uint) {
    rustrt::str_reserve_shared(ss, nn);
}

// Module: unsafe
//
// These functions may create invalid UTF-8 strings and eat your baby.
mod unsafe {
   export
      // FIXME: stop exporting several of these
      from_bytes,
      from_byte,
      slice_bytes,
      push_byte,
      push_bytes,
      pop_byte,
      shift_byte,
      set_len;

   // Function: unsafe::from_bytes
   //
   // Converts a vector of bytes to a string. Does not verify that the
   // vector contains valid UTF-8.
   unsafe fn from_bytes(v: [const u8]) -> str unsafe {
       let mut vcopy: [u8] = v + [0u8];
       let scopy: str = ::unsafe::reinterpret_cast(vcopy);
       ::unsafe::leak(vcopy);
       ret scopy;
   }

   // Function: unsafe::from_byte
   //
   // Converts a byte to a string. Does not verify that the byte is
   // valid UTF-8.
   unsafe fn from_byte(u: u8) -> str { unsafe::from_bytes([u]) }

   /*
   Function: slice_bytes

   Takes a bytewise (not UTF-8) slice from a string.
   Returns the substring from [`begin`..`end`).

   Failure:

   - If begin is greater than end.
   - If end is greater than the length of the string.
   */
   unsafe fn slice_bytes(s: str, begin: uint, end: uint) -> str unsafe {
       assert (begin <= end);
       assert (end <= len(s));

       let mut v = as_bytes(s) { |v| vec::slice(v, begin, end) };
       v += [0u8];
       let s: str = ::unsafe::reinterpret_cast(v);
       ::unsafe::leak(v);
       ret s;
   }

   // Function: push_byte
   //
   // Appends a byte to a string. (Not UTF-8 safe).
   unsafe fn push_byte(&s: str, b: u8) {
       rustrt::rust_str_push(s, b);
   }

   // Function: push_bytes
   //
   // Appends a vector of bytes to a string. (Not UTF-8 safe).
   unsafe fn push_bytes(&s: str, bytes: [u8]) {
       for byte in bytes { rustrt::rust_str_push(s, byte); }
   }

   // Function: pop_byte
   //
   // Removes the last byte from a string and returns it.  (Not UTF-8 safe).
   unsafe fn pop_byte(&s: str) -> u8 unsafe {
       let len = len(s);
       assert (len > 0u);
       let b = s[len - 1u];
       set_len(s, len - 1u);
       ret b;
   }

   // Function: shift_byte
   //
   // Removes the first byte from a string and returns it. (Not UTF-8 safe).
   unsafe fn shift_byte(&s: str) -> u8 unsafe {
       let len = len(s);
       assert (len > 0u);
       let b = s[0];
       s = unsafe::slice_bytes(s, 1u, len);
       ret b;
   }

    unsafe fn set_len(&v: str, new_len: uint) {
        let repr: *vec::unsafe::vec_repr = ::unsafe::reinterpret_cast(v);
        (*repr).fill = new_len + 1u;
        let null = ptr::mut_offset(ptr::mut_addr_of((*repr).data), new_len);
        *null = 0u8;
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_eq() {
        assert (eq("", ""));
        assert (eq("foo", "foo"));
        assert (!eq("foo", "bar"));
    }

    #[test]
    fn test_le() {
        assert (le("", ""));
        assert (le("", "foo"));
        assert (le("foo", "foo"));
        assert (!eq("foo", "bar"));
    }

    #[test]
    fn test_len() {
        assert (len("") == 0u);
        assert (len("hello world") == 11u);
        assert (len("\x63") == 1u);
        assert (len("\xa2") == 2u);
        assert (len("\u03c0") == 2u);
        assert (len("\u2620") == 3u);
        assert (len("\U0001d11e") == 4u);

        assert (char_len("") == 0u);
        assert (char_len("hello world") == 11u);
        assert (char_len("\x63") == 1u);
        assert (char_len("\xa2") == 1u);
        assert (char_len("\u03c0") == 1u);
        assert (char_len("\u2620") == 1u);
        assert (char_len("\U0001d11e") == 1u);
        assert (char_len("ประเทศไทย中华Việt Nam") == 19u);
    }

    #[test]
    fn test_rfind_char() {
        assert rfind_char("hello", 'l') == some(3u);
        assert rfind_char("hello", 'o') == some(4u);
        assert rfind_char("hello", 'h') == some(0u);
        assert rfind_char("hello", 'z') == none;
        assert rfind_char("ประเทศไทย中华Việt Nam", '华') == some(30u);
    }

    #[test]
    fn test_pop_char() {
        let data = "ประเทศไทย中华";
        let cc = pop_char(data);
        assert "ประเทศไทย中" == data;
        assert '华' == cc;
    }

    #[test]
    fn test_pop_char_2() {
        let data2 = "华";
        let cc2 = pop_char(data2);
        assert "" == data2;
        assert '华' == cc2;
    }

    #[test]
    #[should_fail]
    #[ignore(cfg(target_os = "win32"))]
    fn test_pop_char_fail() {
        let data = "";
        let _cc3 = pop_char(data);
    }

    #[test]
    fn test_split_char() {
        fn t(s: str, c: char, u: [str]) {
            log(debug, "split_byte: " + s);
            let v = split_char(s, c);
            #debug("split_byte to: %?", v);
            assert vec::all2(v, u, { |a,b| a == b });
        }
        t("abc.hello.there", '.', ["abc", "hello", "there"]);
        t(".hello.there", '.', ["", "hello", "there"]);
        t("...hello.there.", '.', ["", "", "", "hello", "there", ""]);

        assert ["", "", "", "hello", "there", ""]
            == split_char("...hello.there.", '.');

        assert [""] == split_char("", 'z');
        assert ["",""] == split_char("z", 'z');
        assert ["ok"] == split_char("ok", 'z');
    }

    #[test]
    fn test_split_char_2() {
        let data = "ประเทศไทย中华Việt Nam";
        assert ["ประเทศไทย中华", "iệt Nam"]
            == split_char(data, 'V');
        assert ["ประเ", "ศไ", "ย中华Việt Nam"]
            == split_char(data, 'ท');
    }

    #[test]
    fn test_splitn_char() {
        fn t(s: str, c: char, n: uint, u: [str]) {
            log(debug, "splitn_byte: " + s);
            let v = splitn_char(s, c, n);
            #debug("split_byte to: %?", v);
            #debug("comparing vs. %?", u);
            assert vec::all2(v, u, { |a,b| a == b });
        }
        t("abc.hello.there", '.', 0u, ["abc.hello.there"]);
        t("abc.hello.there", '.', 1u, ["abc", "hello.there"]);
        t("abc.hello.there", '.', 2u, ["abc", "hello", "there"]);
        t("abc.hello.there", '.', 3u, ["abc", "hello", "there"]);
        t(".hello.there", '.', 0u, [".hello.there"]);
        t(".hello.there", '.', 1u, ["", "hello.there"]);
        t("...hello.there.", '.', 3u, ["", "", "", "hello.there."]);
        t("...hello.there.", '.', 5u, ["", "", "", "hello", "there", ""]);

        assert [""] == splitn_char("", 'z', 5u);
        assert ["",""] == splitn_char("z", 'z', 5u);
        assert ["ok"] == splitn_char("ok", 'z', 5u);
        assert ["z"] == splitn_char("z", 'z', 0u);
        assert ["w.x.y"] == splitn_char("w.x.y", '.', 0u);
        assert ["w","x.y"] == splitn_char("w.x.y", '.', 1u);
    }

    #[test]
    fn test_splitn_char_2 () {
        let data = "ประเทศไทย中华Việt Nam";
        assert ["ประเทศไทย中", "Việt Nam"]
            == splitn_char(data, '华', 1u);

        assert ["", "", "XXX", "YYYzWWWz"]
            == splitn_char("zzXXXzYYYzWWWz", 'z', 3u);
        assert ["",""] == splitn_char("z", 'z', 5u);
        assert [""] == splitn_char("", 'z', 5u);
        assert ["ok"] == splitn_char("ok", 'z', 5u);
    }


    #[test]
    fn test_splitn_char_3() {
        let data = "ประเทศไทย中华Việt Nam";
        assert ["ประเทศไทย中华", "iệt Nam"]
            == splitn_char(data, 'V', 1u);
        assert ["ประเ", "ศไทย中华Việt Nam"]
            == splitn_char(data, 'ท', 1u);

    }

    #[test]
    fn test_split_str() {
        fn t(s: str, sep: str, i: int, k: str) {
            let v = split_str(s, sep);
            assert eq(v[i], k);
        }

        t("--1233345--", "12345", 0, "--1233345--");
        t("abc::hello::there", "::", 0, "abc");
        t("abc::hello::there", "::", 1, "hello");
        t("abc::hello::there", "::", 2, "there");
        t("::hello::there", "::", 0, "");
        t("hello::there::", "::", 2, "");
        t("::hello::there::", "::", 3, "");

        let data = "ประเทศไทย中华Việt Nam";
        assert ["ประเทศไทย", "Việt Nam"]
            == split_str (data, "中华");

        assert ["", "XXX", "YYY", ""]
            == split_str("zzXXXzzYYYzz", "zz");

        assert ["zz", "zYYYz"]
            == split_str("zzXXXzYYYz", "XXX");


        assert ["", "XXX", "YYY", ""] == split_str(".XXX.YYY.", ".");
        assert [""] == split_str("", ".");
        assert ["",""] == split_str("zz", "zz");
        assert ["ok"] == split_str("ok", "z");
        assert ["","z"] == split_str("zzz", "zz");
        assert ["","","z"] == split_str("zzzzz", "zz");
    }


    #[test]
    fn test_split() {
        let data = "ประเทศไทย中华Việt Nam";
        assert ["ประเทศไทย中", "Việt Nam"]
            == split (data, {|cc| cc == '华'});

        assert ["", "", "XXX", "YYY", ""]
            == split("zzXXXzYYYz", char::is_lowercase);

        assert ["zz", "", "", "z", "", "", "z"]
            == split("zzXXXzYYYz", char::is_uppercase);

        assert ["",""] == split("z", {|cc| cc == 'z'});
        assert [""] == split("", {|cc| cc == 'z'});
        assert ["ok"] == split("ok", {|cc| cc == 'z'});
    }

    #[test]
    fn test_lines() {
        let lf = "\nMary had a little lamb\nLittle lamb\n";
        let crlf = "\r\nMary had a little lamb\r\nLittle lamb\r\n";

        assert ["", "Mary had a little lamb", "Little lamb", ""]
            == lines(lf);

        assert ["", "Mary had a little lamb", "Little lamb", ""]
            == lines_any(lf);

        assert ["\r", "Mary had a little lamb\r", "Little lamb\r", ""]
            == lines(crlf);

        assert ["", "Mary had a little lamb", "Little lamb", ""]
            == lines_any(crlf);

        assert [""] == lines    ("");
        assert [""] == lines_any("");
        assert ["",""] == lines    ("\n");
        assert ["",""] == lines_any("\n");
        assert ["banana"] == lines    ("banana");
        assert ["banana"] == lines_any("banana");
    }

    #[test]
    fn test_words () {
        let data = "\nMary had a little lamb\nLittle lamb\n";
        assert ["Mary","had","a","little","lamb","Little","lamb"]
            == words(data);

        assert ["ok"] == words("ok");
        assert [] == words("");
    }

    #[test]
    fn test_find_str() {
        // byte positions
        assert find_str("banana", "apple pie") == none;
        assert find_str("", "") == some(0u);

        let data = "ประเทศไทย中华Việt Nam";
        assert find_str(data, "")     == some(0u);
        assert find_str(data, "ประเ") == some( 0u);
        assert find_str(data, "ะเ")   == some( 6u);
        assert find_str(data, "中华") == some(27u);
        assert find_str(data, "ไท华") == none;
    }

    #[test]
    fn test_find_str_between() {
        // byte positions
        assert find_str_between("", "", 0u, 0u) == some(0u);

        let data = "abcabc";
        assert find_str_between(data, "ab", 0u, 6u) == some(0u);
        assert find_str_between(data, "ab", 2u, 6u) == some(3u);
        assert find_str_between(data, "ab", 2u, 4u) == none;

        let data = "ประเทศไทย中华Việt Nam";
        data += data;
        assert find_str_between(data, "", 0u, 43u) == some(0u);
        assert find_str_between(data, "", 6u, 43u) == some(6u);

        assert find_str_between(data, "ประ", 0u, 43u) == some( 0u);
        assert find_str_between(data, "ทศไ", 0u, 43u) == some(12u);
        assert find_str_between(data, "ย中", 0u, 43u) == some(24u);
        assert find_str_between(data, "iệt", 0u, 43u) == some(34u);
        assert find_str_between(data, "Nam", 0u, 43u) == some(40u);

        assert find_str_between(data, "ประ", 43u, 86u) == some(43u);
        assert find_str_between(data, "ทศไ", 43u, 86u) == some(55u);
        assert find_str_between(data, "ย中", 43u, 86u) == some(67u);
        assert find_str_between(data, "iệt", 43u, 86u) == some(77u);
        assert find_str_between(data, "Nam", 43u, 86u) == some(83u);
    }

    #[test]
    fn test_substr() {
        fn t(a: str, b: str, start: int) {
            assert (eq(substr(a, start as uint, len(b)), b));
        }
        t("hello", "llo", 2);
        t("hello", "el", 1);
        assert "ะเทศไท" == substr("ประเทศไทย中华Việt Nam", 6u, 6u);
    }

    #[test]
    fn test_concat() {
        fn t(v: [str], s: str) { assert (eq(concat(v), s)); }
        t(["you", "know", "I'm", "no", "good"], "youknowI'mnogood");
        let v: [str] = [];
        t(v, "");
        t(["hi"], "hi");
    }

    #[test]
    fn test_connect() {
        fn t(v: [str], sep: str, s: str) {
            assert (eq(connect(v, sep), s));
        }
        t(["you", "know", "I'm", "no", "good"], " ", "you know I'm no good");
        let v: [str] = [];
        t(v, " ", "");
        t(["hi"], " ", "hi");
    }

    #[test]
    fn test_to_upper() {
        // char::to_upper, and hence str::to_upper
        // are culturally insensitive: I'm not sure they
        // really work for anything but English ASCII, but YMMV

        let unicode = "\u65e5\u672c";
        let input = "abcDEF" + unicode + "xyz:.;";
        let expected = "ABCDEF" + unicode + "XYZ:.;";
        let actual = to_upper(input);
        assert (eq(expected, actual));
    }

    #[test]
    fn test_to_lower() {
        assert "" == map("", char::to_lower);
        assert "ymca" == map("YMCA", char::to_lower);
    }

    #[test]
    fn test_unsafe_slice() unsafe {
        assert (eq("ab", unsafe::slice_bytes("abc", 0u, 2u)));
        assert (eq("bc", unsafe::slice_bytes("abc", 1u, 3u)));
        assert (eq("", unsafe::slice_bytes("abc", 1u, 1u)));
        fn a_million_letter_a() -> str {
            let i = 0;
            let rs = "";
            while i < 100000 { rs += "aaaaaaaaaa"; i += 1; }
            ret rs;
        }
        fn half_a_million_letter_a() -> str {
            let i = 0;
            let rs = "";
            while i < 100000 { rs += "aaaaa"; i += 1; }
            ret rs;
        }
        assert (eq(half_a_million_letter_a(),
               unsafe::slice_bytes(a_million_letter_a(), 0u, 500000u)));
    }

    #[test]
    fn test_starts_with() {
        assert (starts_with("", ""));
        assert (starts_with("abc", ""));
        assert (starts_with("abc", "a"));
        assert (!starts_with("a", "abc"));
        assert (!starts_with("", "abc"));
    }

    #[test]
    fn test_ends_with() {
        assert (ends_with("", ""));
        assert (ends_with("abc", ""));
        assert (ends_with("abc", "c"));
        assert (!ends_with("a", "abc"));
        assert (!ends_with("", "abc"));
    }

    #[test]
    fn test_is_empty() {
        assert (is_empty(""));
        assert (!is_empty("a"));
    }

    #[test]
    fn test_is_not_empty() {
        assert (is_not_empty("a"));
        assert (!is_not_empty(""));
    }

    #[test]
    fn test_replace() {
        let a = "a";
        assert replace("", a, "b") == "";
        assert replace("a", a, "b") == "b";
        assert replace("ab", a, "b") == "bb";
        let test = "test";
        assert replace(" test test ", test, "toast") == " toast toast ";
        assert replace(" test test ", test, "") == "   ";
    }

    #[test]
    fn test_replace_2a() {
        let data = "ประเทศไทย中华";
        let repl = "دولة الكويت";

        let a = "ประเ";
        let A = "دولة الكويتทศไทย中华";
        assert (replace(data, a, repl) ==  A);
    }

    #[test]
    fn test_replace_2b() {
        let data = "ประเทศไทย中华";
        let repl = "دولة الكويت";

        let b = "ะเ";
        let B = "ปรدولة الكويتทศไทย中华";
        assert (replace(data, b,   repl) ==  B);
    }

    #[test]
    fn test_replace_2c() {
        let data = "ประเทศไทย中华";
        let repl = "دولة الكويت";

        let c = "中华";
        let C = "ประเทศไทยدولة الكويت";
        assert (replace(data, c, repl) ==  C);
    }

    #[test]
    fn test_replace_2d() {
        let data = "ประเทศไทย中华";
        let repl = "دولة الكويت";

        let d = "ไท华";
        assert (replace(data, d, repl) == data);
    }

    #[test]
    fn test_slice() {
        assert (eq("ab", slice("abc", 0u, 2u)));
        assert (eq("bc", slice("abc", 1u, 3u)));
        assert (eq("", slice("abc", 1u, 1u)));
        assert (eq("\u65e5", slice("\u65e5\u672c", 0u, 3u)));

        let data = "ประเทศไทย中华";
        assert "ป" == slice(data, 0u, 3u);
        assert "ร" == slice(data, 3u, 6u);
        assert "" == slice(data, 3u, 3u);
        assert "华" == slice(data, 30u, 33u);

        fn a_million_letter_X() -> str {
            let i = 0;
            let rs = "";
            while i < 100000 { rs += "华华华华华华华华华华"; i += 1; }
            ret rs;
        }
        fn half_a_million_letter_X() -> str {
            let i = 0;
            let rs = "";
            while i < 100000 { rs += "华华华华华"; i += 1; }
            ret rs;
        }
        assert eq(half_a_million_letter_X(),
                  slice(a_million_letter_X(), 0u, 3u * 500000u));
    }

    #[test]
    fn test_slice_2() {
        let ss = "中华Việt Nam";

        assert "华" == slice(ss, 3u, 6u);
        assert "Việt Nam" == slice(ss, 6u, 16u);

        assert "ab" == slice("abc", 0u, 2u);
        assert "bc" == slice("abc", 1u, 3u);
        assert "" == slice("abc", 1u, 1u);

        assert "中" == slice(ss, 0u, 3u);
        assert "华V" == slice(ss, 3u, 7u);
        assert "" == slice(ss, 3u, 3u);
        /*0: 中
          3: 华
          6: V
          7: i
          8: ệ
         11: t
         12:
         13: N
         14: a
         15: m */
    }

    #[test]
    #[should_fail]
    #[ignore(cfg(target_os = "win32"))]
    fn test_slice_fail() {
        slice("中华Việt Nam", 0u, 2u);
    }

    #[test]
    fn test_trim_left() {
        assert (trim_left("") == "");
        assert (trim_left("a") == "a");
        assert (trim_left("    ") == "");
        assert (trim_left("     blah") == "blah");
        assert (trim_left("   \u3000  wut") == "wut");
        assert (trim_left("hey ") == "hey ");
    }

    #[test]
    fn test_trim_right() {
        assert (trim_right("") == "");
        assert (trim_right("a") == "a");
        assert (trim_right("    ") == "");
        assert (trim_right("blah     ") == "blah");
        assert (trim_right("wut   \u3000  ") == "wut");
        assert (trim_right(" hey") == " hey");
    }

    #[test]
    fn test_trim() {
        assert (trim("") == "");
        assert (trim("a") == "a");
        assert (trim("    ") == "");
        assert (trim("    blah     ") == "blah");
        assert (trim("\nwut   \u3000  ") == "wut");
        assert (trim(" hey dude ") == "hey dude");
    }

    #[test]
    fn test_is_whitespace() {
        assert (is_whitespace(""));
        assert (is_whitespace(" "));
        assert (is_whitespace("\u2009")); // Thin space
        assert (is_whitespace("  \n\t   "));
        assert (!is_whitespace("   _   "));
    }

    #[test]
    fn test_is_ascii() {
        assert (is_ascii(""));
        assert (is_ascii("a"));
        assert (!is_ascii("\u2009"));
    }

    #[test]
    fn test_shift_byte() unsafe {
        let s = "ABC";
        let b = unsafe::shift_byte(s);
        assert (s == "BC");
        assert (b == 65u8);
    }

    #[test]
    fn test_pop_byte() unsafe {
        let s = "ABC";
        let b = unsafe::pop_byte(s);
        assert (s == "AB");
        assert (b == 67u8);
    }

    #[test]
    fn test_unsafe_from_bytes() unsafe {
        let a = [65u8, 65u8, 65u8, 65u8, 65u8, 65u8, 65u8];
        let b = unsafe::from_bytes(a);
        assert (b == "AAAAAAA");
    }

    #[test]
    fn test_from_bytes() {
        let ss = "ศไทย中华Việt Nam";
        let bb = [0xe0_u8, 0xb8_u8, 0xa8_u8,
                  0xe0_u8, 0xb9_u8, 0x84_u8,
                  0xe0_u8, 0xb8_u8, 0x97_u8,
                  0xe0_u8, 0xb8_u8, 0xa2_u8,
                  0xe4_u8, 0xb8_u8, 0xad_u8,
                  0xe5_u8, 0x8d_u8, 0x8e_u8,
                  0x56_u8, 0x69_u8, 0xe1_u8,
                  0xbb_u8, 0x87_u8, 0x74_u8,
                  0x20_u8, 0x4e_u8, 0x61_u8,
                  0x6d_u8];

         assert ss == from_bytes(bb);
    }

    #[test]
    #[should_fail]
    #[ignore(cfg(target_os = "win32"))]
    fn test_from_bytes_fail() {
        let bb = [0xff_u8, 0xb8_u8, 0xa8_u8,
                  0xe0_u8, 0xb9_u8, 0x84_u8,
                  0xe0_u8, 0xb8_u8, 0x97_u8,
                  0xe0_u8, 0xb8_u8, 0xa2_u8,
                  0xe4_u8, 0xb8_u8, 0xad_u8,
                  0xe5_u8, 0x8d_u8, 0x8e_u8,
                  0x56_u8, 0x69_u8, 0xe1_u8,
                  0xbb_u8, 0x87_u8, 0x74_u8,
                  0x20_u8, 0x4e_u8, 0x61_u8,
                  0x6d_u8];

         let _x = from_bytes(bb);
    }

    #[test]
    fn test_from_cstr() unsafe {
        let a = [65u8, 65u8, 65u8, 65u8, 65u8, 65u8, 65u8, 0u8];
        let b = vec::unsafe::to_ptr(a);
        let c = from_cstr(b);
        assert (c == "AAAAAAA");
    }

    #[test]
    fn test_from_cstr_len() unsafe {
        let a = [65u8, 65u8, 65u8, 65u8, 65u8, 65u8, 65u8, 0u8];
        let b = vec::unsafe::to_ptr(a);
        let c = from_cstr_len(b, 3u);
        assert (c == "AAA");
    }

    #[test]
    fn test_as_buf() unsafe {
        let a = "Abcdefg";
        let b = as_buf(a, {|buf| assert (*buf == 65u8); 100 });
        assert (b == 100);
    }

    #[test]
    fn test_as_buf_small() unsafe {
        let a = "A";
        let b = as_buf(a, {|buf| assert (*buf == 65u8); 100 });
        assert (b == 100);
    }

    #[test]
    fn test_as_buf2() unsafe {
        let s = "hello";
        let sb = as_buf(s, {|b| b });
        let s_cstr = from_cstr(sb);
        assert (eq(s_cstr, s));
    }

    #[test]
    fn vec_str_conversions() {
        let s1: str = "All mimsy were the borogoves";

        let v: [u8] = bytes(s1);
        let s2: str = from_bytes(v);
        let i: uint = 0u;
        let n1: uint = len(s1);
        let n2: uint = vec::len::<u8>(v);
        assert (n1 == n2);
        while i < n1 {
            let a: u8 = s1[i];
            let b: u8 = s2[i];
            log(debug, a);
            log(debug, b);
            assert (a == b);
            i += 1u;
        }
    }

    #[test]
    fn test_contains() {
        assert contains("abcde", "bcd");
        assert contains("abcde", "abcd");
        assert contains("abcde", "bcde");
        assert contains("abcde", "");
        assert contains("", "");
        assert !contains("abcde", "def");
        assert !contains("", "a");

        let data = "ประเทศไทย中华Việt Nam";
        assert  contains(data, "ประเ");
        assert  contains(data, "ะเ");
        assert  contains(data, "中华");
        assert !contains(data, "ไท华");
    }

    #[test]
    fn test_chars_iter() {
        let i = 0;
        chars_iter("x\u03c0y") {|ch|
            alt check i {
              0 { assert ch == 'x'; }
              1 { assert ch == '\u03c0'; }
              2 { assert ch == 'y'; }
            }
            i += 1;
        }

        chars_iter("") {|_ch| fail; } // should not fail
    }

    #[test]
    fn test_bytes_iter() {
        let i = 0;

        bytes_iter("xyz") {|bb|
            alt check i {
              0 { assert bb == 'x' as u8; }
              1 { assert bb == 'y' as u8; }
              2 { assert bb == 'z' as u8; }
            }
            i += 1;
        }

        bytes_iter("") {|bb| assert bb == 0u8; }
    }

    #[test]
    fn test_split_char_iter() {
        let data = "\nMary had a little lamb\nLittle lamb\n";

        let ii = 0;

        split_char_iter(data, ' ') {|xx|
            alt ii {
              0 { assert "\nMary" == xx; }
              1 { assert "had"    == xx; }
              2 { assert "a"      == xx; }
              3 { assert "little" == xx; }
              _ { () }
            }
            ii += 1;
        }
    }

    #[test]
    fn test_splitn_char_iter() {
        let data = "\nMary had a little lamb\nLittle lamb\n";

        let ii = 0;

        splitn_char_iter(data, ' ', 2u) {|xx|
            alt ii {
              0 { assert "\nMary" == xx; }
              1 { assert "had"    == xx; }
              2 { assert "a little lamb\nLittle lamb\n" == xx; }
              _ { () }
            }
            ii += 1;
        }
    }

    #[test]
    fn test_words_iter() {
        let data = "\nMary had a little lamb\nLittle lamb\n";

        let ii = 0;

        words_iter(data) {|ww|
            alt ii {
              0 { assert "Mary"   == ww; }
              1 { assert "had"    == ww; }
              2 { assert "a"      == ww; }
              3 { assert "little" == ww; }
              _ { () }
            }
            ii += 1;
        }

        words_iter("") {|_x| fail; } // should not fail
    }

    #[test]
    fn test_lines_iter () {
        let lf = "\nMary had a little lamb\nLittle lamb\n";

        let ii = 0;

        lines_iter(lf) {|x|
            alt ii {
                0 { assert "" == x; }
                1 { assert "Mary had a little lamb" == x; }
                2 { assert "Little lamb" == x; }
                3 { assert "" == x; }
                _ { () }
            }
            ii += 1;
        }
    }

    #[test]
    fn test_map() {
        assert "" == map("", char::to_upper);
        assert "YMCA" == map("ymca", char::to_upper);
    }

    #[test]
    fn test_all() {
        assert true  == all("", char::is_uppercase);
        assert false == all("ymca", char::is_uppercase);
        assert true  == all("YMCA", char::is_uppercase);
        assert false == all("yMCA", char::is_uppercase);
        assert false == all("YMCy", char::is_uppercase);
    }

    #[test]
    fn test_any() {
        assert false  == any("", char::is_uppercase);
        assert false == any("ymca", char::is_uppercase);
        assert true  == any("YMCA", char::is_uppercase);
        assert true == any("yMCA", char::is_uppercase);
        assert true == any("Ymcy", char::is_uppercase);
    }

    #[test]
    fn test_chars() {
        let ss = "ศไทย中华Việt Nam";
        assert ['ศ','ไ','ท','ย','中','华','V','i','ệ','t',' ','N','a','m']
            == chars(ss);
    }

    #[test]
    fn test_utf16() {
        let pairs =
            [("𐍅𐌿𐌻𐍆𐌹𐌻𐌰\n",
              [0xd800_u16, 0xdf45_u16, 0xd800_u16, 0xdf3f_u16,
               0xd800_u16, 0xdf3b_u16, 0xd800_u16, 0xdf46_u16,
               0xd800_u16, 0xdf39_u16, 0xd800_u16, 0xdf3b_u16,
               0xd800_u16, 0xdf30_u16, 0x000a_u16]),

             ("𐐒𐑉𐐮𐑀𐐲𐑋 𐐏𐐲𐑍\n",
              [0xd801_u16, 0xdc12_u16, 0xd801_u16,
               0xdc49_u16, 0xd801_u16, 0xdc2e_u16, 0xd801_u16,
               0xdc40_u16, 0xd801_u16, 0xdc32_u16, 0xd801_u16,
               0xdc4b_u16, 0x0020_u16, 0xd801_u16, 0xdc0f_u16,
               0xd801_u16, 0xdc32_u16, 0xd801_u16, 0xdc4d_u16,
               0x000a_u16]),

             ("𐌀𐌖𐌋𐌄𐌑𐌉·𐌌𐌄𐌕𐌄𐌋𐌉𐌑\n",
              [0xd800_u16, 0xdf00_u16, 0xd800_u16, 0xdf16_u16,
               0xd800_u16, 0xdf0b_u16, 0xd800_u16, 0xdf04_u16,
               0xd800_u16, 0xdf11_u16, 0xd800_u16, 0xdf09_u16,
               0x00b7_u16, 0xd800_u16, 0xdf0c_u16, 0xd800_u16,
               0xdf04_u16, 0xd800_u16, 0xdf15_u16, 0xd800_u16,
               0xdf04_u16, 0xd800_u16, 0xdf0b_u16, 0xd800_u16,
               0xdf09_u16, 0xd800_u16, 0xdf11_u16, 0x000a_u16 ]),

             ("𐒋𐒘𐒈𐒑𐒛𐒒 𐒕𐒓 𐒈𐒚𐒍 𐒏𐒜𐒒𐒖𐒆 𐒕𐒆\n",
              [0xd801_u16, 0xdc8b_u16, 0xd801_u16, 0xdc98_u16,
               0xd801_u16, 0xdc88_u16, 0xd801_u16, 0xdc91_u16,
               0xd801_u16, 0xdc9b_u16, 0xd801_u16, 0xdc92_u16,
               0x0020_u16, 0xd801_u16, 0xdc95_u16, 0xd801_u16,
               0xdc93_u16, 0x0020_u16, 0xd801_u16, 0xdc88_u16,
               0xd801_u16, 0xdc9a_u16, 0xd801_u16, 0xdc8d_u16,
               0x0020_u16, 0xd801_u16, 0xdc8f_u16, 0xd801_u16,
               0xdc9c_u16, 0xd801_u16, 0xdc92_u16, 0xd801_u16,
               0xdc96_u16, 0xd801_u16, 0xdc86_u16, 0x0020_u16,
               0xd801_u16, 0xdc95_u16, 0xd801_u16, 0xdc86_u16,
               0x000a_u16 ]) ];

        for (s, u) in pairs {
            assert to_utf16(s) == u;
            assert from_utf16(u) == s;
            assert from_utf16(to_utf16(s)) == s;
            assert to_utf16(from_utf16(u)) == u;
        }
    }
}
