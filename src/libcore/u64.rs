#[doc = "Operations and constants for `u64`"];

/*
Module: u64
*/

/*
Const: min_value

Return the minimal value for a u64
*/
const min_value: u64 = 0u64;

/*
Const: max_value

Return the maximal value for a u64
*/
const max_value: u64 = 0u64 - 1u64;

pure fn min(x: u64, y: u64) -> u64 { if x < y { x } else { y } }
pure fn max(x: u64, y: u64) -> u64 { if x > y { x } else { y } }

/* Function: add */
pure fn add(x: u64, y: u64) -> u64 { ret x + y; }

/* Function: sub */
pure fn sub(x: u64, y: u64) -> u64 { ret x - y; }

/* Function: mul */
pure fn mul(x: u64, y: u64) -> u64 { ret x * y; }

/* Function: div */
pure fn div(x: u64, y: u64) -> u64 { ret x / y; }

/* Function: rem */
pure fn rem(x: u64, y: u64) -> u64 { ret x % y; }

/* Predicate: lt */
pure fn lt(x: u64, y: u64) -> bool { ret x < y; }

/* Predicate: le */
pure fn le(x: u64, y: u64) -> bool { ret x <= y; }

/* Predicate: eq */
pure fn eq(x: u64, y: u64) -> bool { ret x == y; }

/* Predicate: ne */
pure fn ne(x: u64, y: u64) -> bool { ret x != y; }

/* Predicate: ge */
pure fn ge(x: u64, y: u64) -> bool { ret x >= y; }

/* Predicate: gt */
pure fn gt(x: u64, y: u64) -> bool { ret x > y; }

/*
Function: range

Iterate over the range [`lo`..`hi`)
*/
fn range(lo: u64, hi: u64, it: fn(u64)) {
    let mut i = lo;
    while i < hi { it(i); i += 1u64; }
}

/*
Function: to_str

Convert to a string in a given base
*/
fn to_str(n: u64, radix: uint) -> str {
    assert (0u < radix && radix <= 16u);

    let r64 = radix as u64;

    fn digit(n: u64) -> str {
        ret alt n {
              0u64 { "0" }
              1u64 { "1" }
              2u64 { "2" }
              3u64 { "3" }
              4u64 { "4" }
              5u64 { "5" }
              6u64 { "6" }
              7u64 { "7" }
              8u64 { "8" }
              9u64 { "9" }
              10u64 { "a" }
              11u64 { "b" }
              12u64 { "c" }
              13u64 { "d" }
              14u64 { "e" }
              15u64 { "f" }
              _ { fail }
            };
    }

    if n == 0u64 { ret "0"; }

    let mut s = "";

    let mut n = n;
    while n > 0u64 { s = digit(n % r64) + s; n /= r64; }
    ret s;
}

/*
Function: str

Convert to a string
*/
fn str(n: u64) -> str { ret to_str(n, 10u); }

/*
Function: from_str

Parse a string as an unsigned integer.
*/
fn from_str(buf: str, radix: u64) -> option<u64> {
    if str::len(buf) == 0u { ret none; }
    let mut i = str::len(buf) - 1u;
    let mut power = 1u64, n = 0u64;
    while true {
        alt char::to_digit(buf[i] as char, radix as uint) {
          some(d) { n += d as u64 * power; }
          none { ret none; }
        }
        power *= radix;
        if i == 0u { ret some(n); }
        i -= 1u;
    }
    fail;
}

#[doc = "Computes the bitwise complement"]
fn compl(i: u64) -> u64 {
    max_value ^ i
}
