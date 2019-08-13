# Ruby FFI Example

## `libffi` Library and `ffi` Gem

Install (or upgrade) the `libffi` library and the `ffi` gem.

To install the `libffi` library on macOS, run:

```bash
$ brew install libffi
```

## Generate Rust project

Create a new cargo package using a library template:

```bash
$ cargo new ruby_ffi --lib
```

## Cargo Configuration

Add the library name and crate type in Cargo.toml:

```
[lib]
name = "rubyffi"
crate-type = ["dylib"]
```

Also, add `libc` as a dependency:

```
[dependencies]
libc = "0.2.61"
```

## Rust Function

The `no_mangle` attribute turns off Rust's [name mangling][mangling] so that
it's easier to link to.

In the context of FFI, Rust's `extern` keyword can be used to declare function
interfaces that Rust code can call foreign code by, and the mirror use case of
making a Rust code available as a function interface for another programming
language to call.

The following Rust code is callable from Ruby:

```rust
#[no_mangle]
pub extern fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

## Using FFI in Ruby

The `ffi_lib` method accepts as argument the path to the compiled library. In
the following example, the Ruby script is located in the Rust project directory
and uses relative path to point to the compiled Rust library `librubyffi` as
named in Cargo.toml:

```ruby
require 'ffi'

module Main
  extend FFI::Library
  ffi_lib 'target/debug/librubyffi.' + FFI::Platform::LIBSUFFIX
  attach_function :add, [:int, :int], :int
end
```

The `attach_function` above accepts the following parameters:

* name of Rust function to attach
* array of the types of the arguments of the foreign function
* type of the return value of the foreign function

To call the function, simply:

```ruby
sum = Main.add(2, 2)
```

[mangling]: https://en.wikipedia.org/wiki/Name_mangling "Name Mangling on Wikipedia"
