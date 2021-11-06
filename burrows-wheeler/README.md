# Burrows-Wheeler Transformation Huffman Compressor

Algorithms steps:

- Burrow-Wheeler transformation
- Move to front transformation
- Huffman compression

## Bench marks

| file name | compression time (in s) | decompression time (in s) | compression         |
| :-------- | :---------------------- | :------------------------ | :------------------ |
| bib       | 0.107                   | 0.082                     | 0.29832555882115025 |
| book1     | 0.834                   | 0.584                     | 0.34750400314267837 |
| book2     | 0.614                   | 0.452                     | 0.30609669054572602 |
| geo       | 0.301                   | 0.203                     | 0.67920898437499999 |
| news      | 0.396                   | 0.302                     | 0.35401966009827396 |
| obj1      | 0.049                   | 0.034                     | 0.54747953869047616 |
| obj2      | 0.347                   | 0.248                     | 0.35946097060944682 |
| paper1    | 0.053                   | 0.042                     | 0.34256315720170805 |
| paper2    | 0.080                   | 0.086                     | 0.34214528157276852 |
| pic       | 0.448                   | 0.328                     | 0.19776273537847611 |
| progc     | 0.042                   | 0.033                     | 0.34551008558228774 |
| progl     | 0.065                   | 0.051                     | 0.26145213968679343 |
| progp     | 0.046                   | 0.036                     | 0.25950302760282712 |
| trans     | 0.084                   | 0.065                     | 0.23894551470195849 |
