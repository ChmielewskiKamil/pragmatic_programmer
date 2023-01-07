# Turtle language parser

User can provide the following commands:
| Command | Meaning |
| --- | --- |
| P x | Select pen number "x" |
| D | Put the pen down |
| U | Pick the pen up |
| N x | Draw north "x" cm |
| S x | Draw south "x" cm |
| W x | Draw west "x" cm |
| E x | Draw east "x" cm |

Each command has to be separated by the newline character (`\n`).

Run the program with:
```
cargo run -- "P 2
D
N 3
S 2
W 1
U"
```

Would result in: 
```
Pen 2 selected
Pen down
Draw north 3 cm
Draw south 2 cm
Draw west 1 cm
Pen up
```
