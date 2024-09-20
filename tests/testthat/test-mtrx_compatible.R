# unsupported column types -----------------------------------------------------
test_that("mtrx_compatible() correctly handles supported column types", {
  df <- data.frame(
    a = 1:3,
    b = c(1.1, 2.2, 3.3),
    c = letters[1:3],
    d = factor( letters[1:3])
  )
  expect_true(mtrx_compatible(df))
})

test_that("mtrx_compatible() correctly identifies unsupported column types", {
  df_date <- data.frame(
    a = 1:3,
    b = as.Date(c("2023-01-01", "2023-01-02", "2023-01-03"))
  )
  expect_false(mtrx_compatible(df_date))

  df_list <- data.frame(
    a = 1:3,
    b = I(list(1, 2, 3))
  )
  expect_false(mtrx_compatible(df_list))
})

test_that("mtrx_compatible() handles mixed supported and unsupported column types", {
  df_mixed <- data.frame(
    a = 1:3,
    b = c("A", "B", "C"),
    c = as.Date(c("2023-01-01", "2023-01-02", "2023-01-03")),
    d = factor(c("X", "Y", "Z"))
  )
  expect_false(mtrx_compatible(df_mixed))
})

# missing values ---------------------------------------------------------------

test_that("mtrx_compatible() correctly detects missing values", {
  df <- data.frame(
    a = 1:3,
    b = c(1.1, 2.2, 3.3),
    c = letters[1:3],
    d = factor(letters[1:3])
  )
  expect_true(mtrx_compatible(df))
})

test_that("mtrx_compatible() correctly identifies missing values in numeric columns", {
  df <- data.frame(
    a = c(1, NA, 3),
    b = c(1.1, 2.2, 3.3)
  )
  expect_false(mtrx_compatible(df))
})

test_that("mtrx_compatible() correctly identifies missing values in character columns", {
  df <- data.frame(
    a = 1:3,
    b = c("A", NA, "C")
  )
  expect_false(mtrx_compatible(df))
})

test_that("mtrx_compatible() correctly identifies missing values in factor columns", {
  df <- data.frame(
    a = 1:3,
    b = factor(c("X", NA, "Z"))
  )
  expect_false(mtrx_compatible(df))
})

test_that("mtrx_compatible() handles data frames with mixed column types and missing values", {
  df <- data.frame(
    a = c(1, 2, NA),
    b = c(1.1, NA, 3.3),
    c = c("A", "B", NA),
    d = factor(c("X", NA, "Z"))
  )
  expect_false(mtrx_compatible(df))
})

test_that("mtrx_compatible() correctly handles empty data frames", {
  df_empty <- data.frame()
  expect_true(mtrx_compatible(df_empty))
})

# non-default contrasts -------------------------------------------------------
test_that("mtrx_compatible() correctly handles default contrasts", {
  withr::local_options(
    contrasts = c(unordered = "contr.treatment", ordered = "contr.poly")
  )

  df_numeric <- data.frame(a = 1:3, b = 4:6)
  expect_true(mtrx_compatible(df_numeric))

  df_factor <- data.frame(a = factor(letters[1:3]), b = 1:3)
  expect_true(mtrx_compatible(df_factor))
})

test_that("mtrx_compatible() correctly identifies non-default global contrasts", {
  withr::local_options(
    contrasts = c(unordered = "contr.sum", ordered = "contr.poly")
  )
  df <- data.frame(a = factor(letters[1:3]), b = 1:3)
  expect_false(mtrx_compatible(df))
})

test_that("mtrx_compatible() correctly identifies non-default data frame contrasts", {
  df <- data.frame(a = factor(letters[1:3]), b = 1:3)
  attr(df, "contrasts") <- list(a = "contr.sum")

  expect_false(mtrx_compatible(df))
})

test_that("mtrx_compatible() correctly identifies non-default factor contrasts", {
  df <- data.frame(a = factor(letters[1:3]), b = 1:3)
  contrasts(df$a) <- "contr.sum"

  expect_false(mtrx_compatible(df))
})

test_that("mtrx_compatible() handles mixed factor and non-factor columns", {
  df <- data.frame(
    a = factor(letters[1:3]),
    b = 1:3,
    c = c( letters[1:3])
  )

  expect_true(mtrx_compatible(df))

  contrasts(df$a) <- "contr.sum"

  expect_false(mtrx_compatible(df))
})
