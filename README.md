# Transmutation

Transmutation is a command-line tool written in Rust that allows you to use the power of language models to generate Rust code, and then seamlessly route that code to a specific file or directory while automatically formatting it to the Rust standard. By leveraging state-of-the-art language models, Transmutation can help you rapidly prototype, create, and maintain your Rust projects.

## Use Case

Transmutation is designed for developers who want to streamline their Rust development process. By utilizing language models, Transmutation can generate code snippets, functions, or even entire modules based on your input, saving you time and effort. This tool also automatically formats the generated code according to Rust standards, ensuring that your codebase remains clean and consistent.

## Usage

To use Transmutation, you'll need to have Rust installed on your computer. Once you have Rust installed, you can build the tool by running the following command in your terminal:

Clone this repository:
git clone https://github.com/yourusername/transmutation.git
cd transmutation

You can build the tool by running the following command in your terminal:
cargo build --release

./target/release/transmutation --target /path/to/file/or/directory --code "your code here"

run the application with your desired parameters:
cargo run -- -t <target> -c <code> [--rustfmt-config <config_path>] [--rust-version <version>]

Replace <target> with the file or directory you want to upload the code to, <code> with the Rust code you want to format and upload, <config_path> with the optional path to a rustfmt configuration file, and <version> with the optional Rust version you want to use for formatting.
For example:

cargo run -- -t ./formatted_code.rs -c "fn main() { println!(\"Hello, world!\"); }"

