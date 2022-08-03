# editdistancek

Calculate [edit distance](https://en.wikipedia.org/wiki/Levenshtein_distance) (or Levenshtein edit distance) between two
strings.

The library is inspired by algorithms proposed by [Ukkonen](https://www.cs.helsinki.fi/u/ukkonen/InfCont85.PDF)
and [Landau, Myers, and Schmidt](https://epubs.siam.org/doi/pdf/10.1137/S0097539794264810) implemented in Rust.
The running time of algorithm is $O(d*min(n,m))$ (where $d$ is the edit distance between strings and $n$ and $m$ are
lengths of strings).

## Motivation

The initial goal of this lib is to create a fast implementation for the bounded version of the edit distance problem (
given a threshold, $k$ return the edit distance $d$ if it is at most $k$ or return that it is more than $k$).
Typically, the classic edit distance algorithm is slow in such cases.

## Installation

Add to Cargo.toml

```toml
[dependecies]
editdistancek = "0.1.0"
```

## Usage

```rust
edit_distance("kitten".as_bytes(), "sitting".as_bytes()); // => 3
edit_distance_k("kitten".as_bytes(), "sitting".as_bytes(), 3); // => Some(3)
edit_distance_k("kitten".as_bytes(), "sitting".as_bytes(), 2); // => None
```

## License

MIT License