devtools::load_all()
nrow <- 1e6
nlevels <- 6

set.seed(1)

d <-
  data.frame(
    outcome = runif(nrow),
    pred_numeric = runif(nrow),
    pred_integer = sample(c(0L, 1L), nrow, replace = TRUE),
    pred_logical = sample(c(TRUE, FALSE), nrow, replace = TRUE),
    pred_factor_2 = factor(sample(letters[1:nlevels], nrow, replace = TRUE)),
    pred_factor_3 = factor(sample(letters[1:nlevels], nrow, replace = TRUE)),
    pred_character_2 = sample(letters[1:nlevels], nrow, replace = TRUE),
    pred_character_3 = sample(letters[1:nlevels], nrow, replace = TRUE)
  )

bench::mark(
  mdl::mtrx(outcome ~ ., d),
  model.matrix(outcome ~ ., d),
  min_iterations = 50,
  check = FALSE
)

# A[,1:9] # without rayon
# B[,1:9] # with rayon

# library(tidyverse)

# bind_rows(
#   bind_cols(rayon = "no", A[,1:9]), # without rayon
#   bind_cols(rayon = "yes", B[,1:9]) # with rayon
# )
# A tibble: 4 × 10
# rayon expression                        min   median `itr/sec` mem_alloc `gc/sec` n_itr  n_gc total_time
# <chr> <bch:expr>                   <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl> <int> <dbl>   <bch:tm>
# 1 no    mdl::mtrx(outcome ~ ., d)      58.2ms   67.8ms     12.6      277MB     7.32    50    29      3.96s
# 2 no    model.matrix(outcome ~ ., d)  144.9ms  158.6ms      5.37     520MB     9.03    50    84      9.31s
# 3 yes   mdl::mtrx(outcome ~ ., d)      59.4ms   62.8ms     14.6      276MB    14.6     50    50      3.43s
# 4 yes   model.matrix(outcome ~ ., d)  150.1ms  156.6ms      5.54     520MB     8.42    50    76      9.03s