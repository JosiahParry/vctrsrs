
# vctrsrs

The goal of `vctrsrs` is to allow `extendr` developers to return a
single struct to R that can be interacted with like a normal vector.

This is based on
[extendr_usize_vector](https://github.com/sorhawell/extendr_usize_vector)
sample repo.

## Example

Here is an example of a `Vec<Option<usize>>`

``` r
library(vctrsrs)

x <- new_usize(sample(1e5, 10))

length(x)
```

    ## [1] 10

``` r
x[1:10]
```

    ## <Vctr[10]> 
    ##  [1] "88786" "81486" "11988" "12025" "5214"  "36683" "39256" "30246" "32422"
    ## [10] "66222"

``` r
x[c(NA, -1, 5)]
```

    ## <Vctr[3]> 
    ## [1] "NA"   "NA"   "5214"

``` r
data.frame(id = 1:10, x = x)
```

    ##    id     x
    ## 1   1 88786
    ## 2   2 81486
    ## 3   3 11988
    ## 4   4 12025
    ## 5   5  5214
    ## 6   6 36683
    ## 7   7 39256
    ## 8   8 30246
    ## 9   9 32422
    ## 10 10 66222

## how it works

In the Rust source a trait and a struct are implemented. The trait
`Rvctr` looks like so:

``` rust
pub trait Rvctr {
    fn show(&self) -> Strings;
    fn length(&self) -> Rint ;
    fn subset(self, idx: Integers) -> Self;
    fn extend(self, y: Self) -> Self;
}
```

These methods are all that are needed to make a single pointed feel like
a vector by providing users the ability to use `length()`, `c()`, `[`,
and `[[`.

The struct `Vctr` is intended to be a general purpose wrapper struct for
any custom struct. It is a tuple struct that accepts and object that has
implemented the `Rvctr` trait. It looks like so:

``` rust
pub struct Vctr<T: Rvctr>(pub T);
```

You then generate an extendr R object by defining the `impl` with the
`#[extendr]` macro. In this impl you have to also specify the methods
for the trait which should be autopopulated with a custom macro like
`#[vctr]` which isn’t yet made.

``` rust
// this could be created by a macro...
#[extendr]
impl Vctr<VecUsize> {
    fn length(&self) -> Rint {
        self.0.length()
    }

    fn show(&self) -> Strings {
        self.0.show()
    }

    fn subset(&mut self, idx: Integers) -> Self {
        let x = self.0.clone();
        let res = x.subset(idx);
        Vctr(res)
    }
}
```

## Problem: pointers aren’t vectors

This implementation can be put into a data frame. But it can’t be put
into a tibble. That’s because tibble does all of its validation with
vctrs. And vctrs only works with atomic and list vectors. So even though
all of this witch craft has been implemented here. It cannot be
compatible with the tidyverse unless it can pass `vctrs::vec_is()`.

``` r
data.frame(id = 1:10, x = x)
```

    ##    id     x
    ## 1   1 88786
    ## 2   2 81486
    ## 3   3 11988
    ## 4   4 12025
    ## 5   5  5214
    ## 6   6 36683
    ## 7   7 39256
    ## 8   8 30246
    ## 9   9 32422
    ## 10 10 66222

``` r
tibble::tibble(id = 1:10, x = x)
```

    ## Error:
    ## ! All columns in a tibble must be vectors.
    ## ✖ Column `x` is a `Vctr` object.
