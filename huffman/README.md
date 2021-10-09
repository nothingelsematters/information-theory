# Huffman compression algorithm

## Usage

Build: `cargo build --release`

Three binaries:

- `huf <input file path> <output file path>` for compressing
- `unhuf <input file path> <output file path>` for decompressing
- `test <input file path>` benchmarking

## Benchmarks

| file                                                            | source file size | compressed file size | compressed (in times) | compressing time |
| :-------------------------------------------------------------- | :--------------- | :------------------- | :-------------------- | :--------------- |
| war and peace (text)                                            | 3.12 Mb          | 1.81 Mb              | 1.725                 | 2s 318ms         |
| [enwik8](http://www.mattmahoney.net/dc/textdata.html) test data | 95.37Mb          | 60.92Mb              | 1.565                 | 71s 370ms        |
