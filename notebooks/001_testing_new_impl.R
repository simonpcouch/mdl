devtools::load_all()

d <- data.frame(x = factor(c("a", "b", "c")))

model.matrix(~x, d)
#>   (Intercept) xb xc
#> 1           1  0  0
#> 2           1  1  0
#> 3           1  0  1

