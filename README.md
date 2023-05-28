# rs-word-nums
Using the handy `num!` macro from this crate, you can write all of the numbers in your Rust source code in plain English.

The crate depends on **nothing** else and never will, so you can live out all of your minimalist fantasies.

Obviously, all of the parsing and macro substitution is done at compile time.

## Example usage
Add the crate to your project using:

    cargo add --git https://github.com/lennyerik/rs-word-nums.git

Afterwards, you can start to rewrite every number in your Rust project in plain, understandable English:

```rust
use word_nums::num;

const MY_NUMBER: i32 = num!(one hundred fifty five thousand three hundred seventy two);

fn main() {
    println!("{}", MY_NUMBER + num!(minus hundred fifty four thousand thirty five));
}
```

The `num!` macro will automatically return an evaluated literal of the smallest integer type the number fits into.
By default, all numbers from `num` will be signed.
If you want an unsigned int, please explicitly specify a positive sign at the start of the number:
```rust
num!(plus one thousand three hundred thirty seven)

// Or

num!(positive one thousand three hundred and thirty seven)
```

The library will do its best to understand you.
It will even reciprocate your lazyness to write thousands as multiples of hundreds:
```rust
num!(fifty seven hundred)
```

## Why?
The library was inspired by this [very cursed Reddit post](https://www.reddit.com/r/programminghorror/comments/13r7c2w/using_macros_to_write_123_as_one_hundred_twenty/).
As any sane person would, I marvelled at the sheer genius of this one very simple header file and came to the logical
conclusion that every Rust programmer also needs the ability to write their numbers this way.
And in the spirit of reimplementing every C library, but better, this library uses Rust's [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) for enhanced user experience.

In all seriousness, this library is probably not useful in any real project, though I am happy to be proven wrong on that.

