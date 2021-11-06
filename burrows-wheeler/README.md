# Burrows-Wheeler Transformation Huffman Compressor

Algorithms steps:

- Burrow-Wheeler transformation
- Move to front transformation
- Huffman compression

## Bench marks

| file name | compression time (in s) | decompression time (in s) | compression         |
| :-------- | :---------------------- | :------------------------ | :------------------ |
| bib       | 0.105                   | 0.086                     | 0.38453725923728888 |
| book1     | 0.828                   | 0.609                     | 0.36147435322092014 |
| book2     | 0.615                   | 0.474                     | 0.32418769726416702 |
| geo       | 0.301                   | 0.207                     | 0.86225585937500004 |
| news      | 0.394                   | 0.315                     | 0.38210968181613275 |
| obj1      | 0.049                   | 0.035                     | 1.4770740327380953  |
| obj2      | 0.347                   | 0.260                     | 0.44400236615427002 |
| paper1    | 0.053                   | 0.044                     | 0.52149131882394983 |
| paper2    | 0.080                   | 0.066                     | 0.47146558960571294 |
| pic       | 0.433                   | 0.350                     | 0.23576622708567152 |
| progc     | 0.042                   | 0.034                     | 0.5858726111433693  |
| progl     | 0.066                   | 0.053                     | 0.40018982218128019 |
| progp     | 0.045                   | 0.038                     | 0.45132546224103365 |
| trans     | 0.085                   | 0.069                     | 0.34743582901969156 |
