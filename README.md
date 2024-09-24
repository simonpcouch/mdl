
<!-- README.md is generated from README.Rmd. Please edit that file -->

# mdl

<!-- badges: start -->

[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://lifecycle.r-lib.org/articles/stages.html#experimental)
[![CRAN
status](https://www.r-pkg.org/badges/version/mdl)](https://CRAN.R-project.org/package=mdl)
<!-- badges: end -->

mdl implements an opinionated and performant reimagining of model
matrices. The package supplies one function, `mdl::mtrx()` (read: “model
matrix”), that takes in a formula and data frame and outputs a numeric
matrix. Compared to its base R friend `model.matrix()`, it’s *really*
fast.

**This package is highly experimental. Interpret results with caution!**

## Installation

You can install the development version of mdl like so:

``` r
# install.packages("mdl")
pak::pak("simonpcouch/mdl")
```

## Example

The output of `mdl::mtrx()` looks a lot like that from `model.matrix()`:

``` r
# convert to factor to demonstrate dummy variable creations
mtcars$cyl <- as.factor(mtcars$cyl)

head(
  mdl::mtrx(mpg ~ ., mtcars)
)
#>   intercept cyl_6 cyl_8 disp  hp drat    wt  qsec vs am gear carb
#> 1         1     1     0  160 110 3.90 2.620 16.46  0  1    4    4
#> 2         1     1     0  160 110 3.90 2.875 17.02  0  1    4    4
#> 3         1     0     0  108  93 3.85 2.320 18.61  1  1    4    1
#> 4         1     1     0  258 110 3.08 3.215 19.44  1  0    3    1
#> 5         1     0     1  360 175 3.15 3.440 17.02  0  0    3    2
#> 6         1     1     0  225 105 2.76 3.460 20.22  1  0    3    1
```

Compared to `model.matrix()`, `mdl::mtrx()` is sort of a glorified
`as.matrix()` data frame method. More specifically:

- Names its intercept `intercept` rather than `(Intercept)`.
- Does not accept formulae with inlined functions (like `-` or `*`).
- Names dummy variables created from characters and factors as
  `colname_level` rather than `colnamelevel`.
- Names dummy variables create from logicals as `colname` rather than
  `colnameTRUE`.
- Never drops rows (and thus doesn’t accept an `na.action`).
- Assumes that factors levels are encoded as they’re intended
  (i.e. `drop.unused.levels` and `xlev` are not accepted).

It’s quite a bit faster for smaller data sets:

``` r
bench::mark(
  mdl::mtrx(mpg ~ ., mtcars),
  model.matrix(mpg ~ ., mtcars),
  check = FALSE
)
#> # A tibble: 2 × 6
#>   expression                         min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>                    <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 mdl::mtrx(mpg ~ ., mtcars)      22.6µs   23.9µs    40039.    3.32KB     20.0
#> 2 model.matrix(mpg ~ ., mtcars)  267.8µs  274.6µs     3581.  494.24KB     34.0
```

The factor of speedup isn’t so drastic for larger datasets and datasets
with more factors, but it is still quite substantial:

``` r
for (p in c("vs", "am", "gear", "carb")) {
  mtcars[[p]] <- as.factor(mtcars[[p]])
}

bench::mark(
  mdl::mtrx(mpg ~ ., mtcars[rep(1:32, 1e5), ]),
  model.matrix(mpg ~ ., mtcars[rep(1:32, 1e5), ]),
  check = FALSE
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
#> # A tibble: 2 × 6
#>   expression                             min median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>                           <bch> <bch:>     <dbl> <bch:byt>    <dbl>
#> 1 mdl::mtrx(mpg ~ ., mtcars[rep(1:32,… 1.36s  1.36s     0.736  803.01MB    0.736
#> 2 model.matrix(mpg ~ ., mtcars[rep(1:… 2.02s  2.02s     0.494    1.86GB    1.98
```

Check out [this
article](https://github.com/simonpcouch/mdl/blob/main/vignettes/articles/plain-r.Rmd)
for more detailed benchmarks.
