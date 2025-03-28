# Scriptum

**Scriptum** is a language designed for writing notes and blogs that compile to clean, portable HTML. Powered by compiler called `sptc`, Scriptum makes it easy to create self-contained documents - perfect for personal notes.

## Features

- Explicit syntax
- Base64 image embedding
- Shipped with inline CSS for pretty looks
- Fast CLI compiler written in Rust

## Building the Compiler

You need [Rust](https://rust-lang.org) installed on your system.

```bash
git clone https://github.com/golferjoe/scriptum
cd scriptum
cargo build --release
```

After running these commands, the compiler can be found at `target/release/sptc`.

## Usage

```bash
sptc -s <file name>.scr
```

### Options:

| Flag        | Description                              | Optional |
| ----------- | ---------------------------------------- | -------- |
| `-s <file>` | Specify source file to be compiled       | No       |
| `-o <file>` | Set file to which output will be written | Yes      |

> Run `sptc --help` for full CLI usage details

## Language Syntax

```
[h:lg] Laaaaarge Heading
[h:md] Medium-sized Heading
[h:sm] Smol Heading
plain text is treated as paragraph
so this is another paragraph
[img:images/tree.png]
[b] very bold text
[i] italiiiic text
# lines starting with '#' are ignored
```

> You can find more examples in the `examples` directory

## Features to be Implemented

- [ ] Math syntax
- [ ] Ordered and unordered lists
- [ ] Set embedded image size
- [ ] Sections
- [ ] Separator
- [ ] External links

## License

Project is under MIT license.
