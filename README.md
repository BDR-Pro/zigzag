# Custom Code Translator to Rust

This project translates custom scripting language code into Rust. It's designed to process specific constructs, including variable declarations, conditional statements, loops, and custom structures, and convert them into equivalent Rust code.

## Features

- Parses custom syntax for variable declarations, conditionals, and loops.
- Supports custom data structures like lists and dictionaries.
- Generates Rust code ready for further testing and compilation.

## Getting Started

### Prerequisites

- Rust (latest stable version recommended). Visit [The Rust Programming Language](https://www.rust-lang.org/tools/install) website for installation instructions.

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/bdr-pro/zigzag.git
   ```

2. Navigate to the project directory:

   ```sh
   cd zigzag
   ```

3. Build the project:

   ```sh
   cargo build --release
   ```

### Usage

To use this translator, run the executable with the path to your custom script file:

```sh
cargo run -- /path/to/your/custom/script.zz
```

The translated Rust code will be printed to stdout. Redirect the output to a `.rs` file if you wish to save the result:

```sh
cargo run -- /path/to/your/custom/script.zz > output.rs
```

## Custom Language Syntax

Briefly describe the syntax of your custom scripting language, including key constructs like variable declarations, conditionals, loops, and any unique features.

### Examples

Provide some basic examples of the custom language syntax and their Rust translations to help users understand how the translation process works.

## Contributing

Contributions are welcome! Please feel free to submit pull requests, report bugs, and suggest features.

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgments

- List any dependencies, tutorials, or other resources that you found helpful.

---
