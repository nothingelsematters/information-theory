# Huffman compression algorithm

## Usage

Build: `cargo build --release`

Three binaries:

- `huf <input file path> <output file path>` for compressing
- `unhuf <input file path> <output file path>` for decompressing
- `test <input file path>` benchmarking

## Benchmarks

| file                                                            | source file size | compressed file size | compressed size | compressing time | decompressing time |
| :-------------------------------------------------------------- | :--------------- | :------------------- | :-------------- | :--------------- | :----------------- |
| war and peace                                                   | 3.12 Mb          | 1.81 Mb              | 57.99%          | 2s 318ms         | 1s 351ms           |
| [enwik8](http://www.mattmahoney.net/dc/textdata.html) test data | 95.37Mb          | 60.92Mb              | 63.88%          | 71s 370ms        | 35s 172ms          |
