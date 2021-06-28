# IndxVec

![Crates.io](https://img.shields.io/crates/v/indxvec?logo=rust) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/liborty/indxvec/HEAD?logo=github)  

## Usage

Insert into your Cargo.toml file [dependencies] section:

```rust
indxvec = "^0.1" 
```

Import into your source file(s) macro `here`, struct `GS`, functions and trait as you need. There is just one trait `Indices` implemented on indices of type &[usize]. There is a bunch of functions in module `merge` which usually take generic vector(s) as arguments and may produce some indices.

```rust
use indxvec::{here,GS,merge::*,Indices};
```

See tests/tests.rs for examples of usage. To run the tests, use single thread. It may be slower but will produce the results in the right order:

```rust
cargo test --release -- --test-threads=1 --nocapture --color always
```

## Description

Indxvec is a spin-off from `rstats`. It is a self-contained unit, both in terms of the subject matter and also in not having any dependencies at all.

The tools included are: efficient ranking, sorting, merging, searching and indices manipulations. They are  applicable to generic vectors `Vec<T>` (or generic slices `&[T]`), thus they will work on primitive types but also on any arbitrarily complex end type T, as long as you implement their required traits, mostly just PartialOrd and/or Copy for T.

## Functions

are in the module `src/merge.rs`. They mostly take some generic data and produce the indices onto which the methods of the following trait can be conveniently chained. See the documentation.

## Trait Index

The methods of this trait are implemented for vectors of subscripts, i.e. `&[usize]`.

* `invindex` - method for inverting an index, e.g. given a sort index, returns ranks and vice versa.

* `unindex` - collects values from a vector in the order given by an index. This will, for example, sort a vector into sort order when supplied with a sort index. Can also be used for key sort, whereby a vector of keys is extracted from some data, then a sort index is obtained for the keys (with `sortidx`) and that is applied (with `unindex`) to the original data. Also, the original sort order can be easily and efficiently reversed by supplying a boolean `false` second argument to `unindex`. This without repeating the sort or reversing the potentially bulky sorted data.

* `ucorrelation` - Pearson's correlation coefficient of two indices, typically ranks. This is the same as Spearman's correlation of the original data.

## Release Notes (Latest First)

**Version 0.1.8** - added function `minmax` to module `merge`.

**Version 0.1.7** - added convenience conversion method `indx_to_f64`.

**Version 0.1.6** - improved comments. Used Vec::with_capacity for new vectors of known lengths. Maybe a bit faster but no change in functionality.

**Version 0.1.5** - fixed an inconsistency in `binsearch` result.

**Version 0.1.4** - swapped arguments of `unindex` for compatibility. Added more comments.

**Version 0.1.3** - added wrapper struct GS (generic slice), though it is not really needed. However, it does pretty-print generic vectors.
