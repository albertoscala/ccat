# `ccat` — A Minimal CSV Viewer for the Terminal

`ccat` is a command-line utility for reading and displaying `.csv` files in a neatly formatted, Unicode-aligned table directly in your terminal.

## Features

- Reads and displays `.csv` files with clean column alignment
- Handles variable-length rows and missing data gracefully
- Supports UTF-8 and Unicode characters (e.g. accented names)
- Outputs using box-drawing characters for readable formatting

## Usage

```bash
ccat file.csv [file2.csv ...]
```

- Accepts one or more .csv files as arguments
- Prints each file as a well-structured table

## Example Output

```bash
┌─────────┬─────┬───────┬─────────┐
│Name     │Age  │Score  │Country  │
├─────────┼─────┼───────┼─────────┤
│Alice    │30   │85     │USA      │
├─────────┼─────┼───────┼─────────┤
│Bob      │25   │90     │Canada   │
├─────────┼─────┼───────┼─────────┤
│Charlie  │28   │78     │UK       │
├─────────┼─────┼───────┼─────────┤
│Diana    │22   │92     │Germany  │
├─────────┼─────┼───────┼─────────┤
│Evan     │35   │88     │France   │
└─────────┴─────┴───────┴─────────┘
```

## Build

To build the project:

```bash
cargo build --release
```

## License

[MIT](https://choosealicense.com/licenses/mit/)