This crate provides an alternative syntax for string formatting which is more friendly to syntax highlighting and allows interpolation by full expressions.

This is primarily provided via the [`f`] macro.  This macro returns a value of type [`std::fmt::Arguments`], and so utility functions to use this type instead of [`std`] functions such as
[`println`] are also provided.

# Usage
---

The behavior of the [`f`] macro is similar to, and ultimately backed by, the [`std`] library's [`std::format_args`].  This is different from [`format`] in that rather than producing a [`String`]
it meerly borrows the arguments into a [`std::fmt::Arguments`].  This type implements [`Display`], and so it can be converted into a [`String`] using [`to_string`], however, the borrowed form works
for functions taking `impl Display`.

Here are some examples of the [`f`] macro:
```
f!("Hello");                         // => "Hello"
f!("Hello, " "world");               // => "Hello, world!"
f!("The number is" 1);               // => "The number is 1"
f!((3, 4):?);                        // => "(3, 4)"
let people = "Rustaceans";
f!("Hello" people "!");              // => "Hello Rustaceans!"
f!(1" "2);                           // => "1 2"
f!(42:0 4);                          // => "0042" with leading zeroes
f!((100, 200):#?);                   // => "(
                                     //        100,
                                     //        200,
                                     //    )"
f!("{look ma, no double braces}");   // => "{look ma, no double braces}"

```

You can see that the syntax resembles the usual rust formatting syntax, except that interpolated expressions are outside the string literals.
This allows arbitrary expressions to be interpolated inline, and also makes it more clear what is actually string text, and what is an interpolated expression.


# Interpolation
---

The arguments to [`f`] are a sequence of string literals and interpolating expressions.  These interpolation expressions look like any valid rust expression followed optionally by a colon, and then
a list of formatting arguments, which are for the most part the same as you would use in a normal Rust format string.  The exceptions to this are the `$` and `*` arguments, which are replaced by
interpolating another expression to serve as the value these would find.

# Formatting Parameters
---

Just like with the usual format strings, formatting arguments are supported using a colon after an interpolated expression:

```
let a = 5;
let b = &a;
println(f!( a:e " " b:p )); // => 5e0 0x20e9b4f76c
```

# Width
---

```
// All of these produce "Hello x    !"
f!("Hello " {"x"}:5 "!");
f!("Hello " {"x"}:{5} "!");
let width = 5;
f!("Hello " {"x"}:{width});
```

If an integer or block expression is placed after the colon, it will be used as the width parameter for formatting.  Here's
a recap of its behavior from the standard library:
> This is a parameter for the “minimum width” that the format should take up. If the value’s string does not fill up this many characters, then the padding specified by fill/alignment will be used to take up the required space (see below).


# Fill/Alignment
---
```
assert_eq!(f!("Hello " {"x"}:<5 "!").to_string(),    "Hello x    !");
assert_eq!(f!("Hello " {"x"}:'-'<5 "!").to_string(), "Hello x----!");
assert_eq!(f!("Hello " {"x"}:^5 "!").to_string(),    "Hello   x  !");
assert_eq!(f!("Hello " {"x"}:>5 "!").to_string(),    "Hello     x!");
```

The alignment within the extra space produced by width can be determined using the characters `<`, `^`, and `>` for left-aligned, centered, and right-aligned respectively.  A character literal can be
placed before the alignment to specifiy what character is used to fill the empty space.  This fill and alignment control behaves the same as the standard library formatter.

# Sign / # / 0
---
```
assert_eq!(f!("Hello " 5:+ "!").to_string(), "Hello +5!");
assert_eq!(f!(27:#x "!"), "0x1b!").to_string();
assert_eq!(f!("Hello " 5:0 5 "!").to_string(),  "Hello 00005!");
assert_eq!(f!("Hello " -5:0 5 "!").to_string(), "Hello -0005!");
assert_eq!(f!(27:#0 10x "!").to_string(), "0x0000001b!");
```

These flags alter the behavior of the formatter.

Here is the explanation from the standard library documentation:
> - `+` - This is intended for numeric types and indicates that the sign should always be printed. By default only the negative sign of signed values is printed, and the sign of positive or unsigned values is omitted. This flag indicates that the correct sign (+ or -) should always be printed.
> - `-` - Currently not used
> - `#` - This flag indicates that the "alternate" form of printing should be used.  The alternate forms are:
>   - `#?` - pretty-print the [`Debug`] formatting (adds linebreaks and indentation)
>   - `#x` - precedes the argument with a `0x`
>   - `#X` - precedes the argument with a `0x`
>   - `#b` - precedes the argument with a `0b`
>   - `#o` - precedes the argument with a `0o`
>   See [`Formatting traits`] for a description of what the `?`, `x`, `X`, `b`, and `o`, flags do.
> - `0` - This is used to indicate for integer formats that the padding to `width` should both be done with a `0` character as well as be sign-aware. A format like `:0 8` would yeild `00000001` for the
> integer `1`, while the same format would yeild `-0000001` for the integer `-1`.  Notice that the negative version has one fewer zero than the positive version. Note that padding zeros are always placed
> after the sign (if any) and before the digits. When used together with the `#` flag, a similar rule applies: padding zeros are inserted after the prefix but before the digits. The prefix is included in
> the total width. This flag overrides the fill character alignment flag.

Notably, the `0` flag must have a space after it before the `width` flag, so `:08` will not work, and it must be written as `:0 8` instead.  This is because using the current implementation, each flag must
be a separate token, and `08` will combine together into a single integer token.  This limitation may be removed in the future.

# Precision
---

The precision can be specified using the syntax `:.N` where `N` is an integer literal, or using `:.{ /* some expression */ }` to use an arbitrary expression as the precision.  This defines how precise
floating point numbers will be displayed as, and represents a maximum width for non-numeric types.  Notably, all formatting arguments must be separate tokens, so to specify the `0` flag followed by the `precision`
flag, you have to add a space, like `:0 .3`, not `:0.3`.  If the width is specified as well, all three must be separate tokens: `:0 4 .3`.

# Traits
---

Formatting ultimately falls to a set of specific traits, and which trait to use is decided by the final format argument.  These are:
- *nothing* => [`Display`]
- `?` => [`Debug`]
- `x?` => [`Debug`] with lower-case hexadecimal integers
- `X?` => [`Debug`] with upper-case hexadecimal integers
- `o` => [`Octal`]
- `x` => [`LowerHex`]
- `X` => [`UpperHex`]
- `p` => [`Pointer`]
- `b` => [`Binary`]
- `e` => [`LowerExp`]
- `E` => [`UpperExp`]


# Functions
---

`strf` also implements several functions for working with the output of [`f`].  These parallel `std` macros that take format strings as arguments.  The implemented functions are:
- [`print`]
- [`println`]
- [`eprint`]
- [`eprintln`]
- [`panic`]
- [`todo`]
- [`unimplemented`]
- [`unreachable`]

These take an argument which is `impl Display` and pass it to the corresponding macro.  This means that the output of [`f`] can be directly passed to these.
