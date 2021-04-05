A simple [Sudoku（数独）](https://en.wikipedia.org/wiki/Sudoku) solver in Rust. Use bit operations to gain better performance.

# The Puzzle
  ```
      A B C   D E F   G H I
    +-----------------------+
  1 | 8     |       |       |
  2 |     3 | 6     |       |
  3 |   7   |   9   | 2     |
    +-----------------------+
  4 |   5   |     7 |       |
  5 |       |   4 5 | 7     |
  6 |       | 1     |   3   |
    +-----------------------+
  7 |     1 |       |   6 8 |
  8 |     8 | 5     |   1   |
  9 |   9   |       | 4     |
    +-----------------------+
  ```
  - Cell: a puzzle board has 81 cells, e.g. cell 2D is '6' in above example.
  - Block: a 3x3 cells group, 9 blocks in total.
  - Peers: Cells in the same row, same column and same block.
  - Candidates: possible numbers in a cell, 1-9, if only one possible number(and no conflict with peers) then thise cell is solved.

# Data Model
  - The whole puzzle board is represented as below one:
  ```
  +---------------------------------------------------------------+
  |   8    1246  24569 | 2347  12357  1234  | 13569 4579  1345679 |
  | 12459   124    3   |   6   12578  1248  | 1589  45789  14579  |
  |  1456    7    456  |  348    9    1348  |   2    458   13456  |
  +---------------------------------------------------------------+
  | 123469   5   2469  | 2389   2368    7   | 1689  2489   12469  |
  | 12369  12368  269  | 2389    4      5   |   7    289   1269   |
  | 24679  2468  24679 |   1    268   2689  | 5689    3    24569  |
  +---------------------------------------------------------------+
  | 23457   234    1   | 23479  237   2349  |  359    6      8    |
  | 23467  2346    8   |   5    2367  23469 |  39     1    2379   |
  | 23567    9   2567  | 2378  123678 12368 |   4    257   2357   |
  +---------------------------------------------------------------+
  ```
  - Candidates: an i16, represents possible numbers (1-9) as a bitmap, e.g. `0` for no candidate is valid; Cell `1B("1246")` represented as `101011b`; The cell is solved while it has only 1 bit(one number) active.
  - Puzzle: a puzzle board has 81 cells, so it can be represented in 162 bytes.

# Algorithm

## Backtrack
  - The main algorithm is a backtrack procedure, try all the candidates in unsolved cells one by one, backtrack while conflicts (with peers), otherwise try next one.
  - Because the whole puzzle board is 162 bytes long, it's effortless to copy and rollback.
  - Optimization: **branch-cutting**, begin with the cell with least possible candidates.

## Ripple
  - While one candidates is solved to an unique number, then this number can't be used in its peers, so broadcast to it its peers, i.e. remove this number from candidates of all peers.
  - This procedure is recursive, and most puzzles can be solved by ripple only.
  - Simple but very powerfull branch-cutting, usually it reduces backtrack count dramatically.

## Triplex
  - It's another branch-cutting operation.
  - In a block, if there are three unsolved candidates union to 3 numbers, e.g. "AB", "BC" and "ABC", then it means "ABC" is the only numbers in these 3 cells, so "ABC" can't be candidates of their common peers, then we can eliminate "ABC" from its common peers. Useful while met complex puzzles.
