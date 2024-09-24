test_that("mtrx() gives similar results to model.matrix()", {
  set.seed(1)
  d <-
    data.frame(
      outcome = runif(30),
      pred_numeric = runif(30),
      pred_integer = sample(c(0L, 1L), 30, replace = TRUE),
      pred_logical = sample(c(TRUE, FALSE), 30, replace = TRUE),
      pred_factor_2 = factor(sample(letters[1:2], 30, replace = TRUE)),
      pred_factor_3 = factor(sample(letters[1:3], 30, replace = TRUE)),
      pred_character_2 = sample(letters[1:2], 30, replace = TRUE),
      pred_character_3 = sample(letters[1:3], 30, replace = TRUE)
    )

  res_mtrx <- mtrx(outcome ~ ., d)
  res_base <- model.matrix(outcome ~ ., d)
  dimnames(res_base) <- dimnames(res_mtrx)

  expect_equal(res_mtrx, res_base, ignore_attr = TRUE)
  expect_equal(
    colnames(res_mtrx),
    c("intercept", "pred_numeric", "pred_integer", "pred_logical",
      "pred_factor_2_b", "pred_factor_3_b", "pred_factor_3_c", "pred_character_2_b",
      "pred_character_3_b", "pred_character_3_c")
  )
  expect_equal(rownames(res_mtrx), as.character(seq(1, nrow(res_mtrx))))
  expect_equal(nrow(d), nrow(res_mtrx))
})

test_that("mtrx() works with missing values", {
  withr::local_options(na.action = "na.pass")

  set.seed(1)
  d <- data.frame(outcome = runif(30))

  # in numerics:
  d_numeric <- cbind(d, pred_numeric = c(NA, runif(29)))
  expect_no_condition(res_mtrx <- mtrx(outcome ~ ., d_numeric))
  res_base <- model.matrix(outcome ~ ., d_numeric)
  dimnames(res_base) <- dimnames(res_mtrx)
  expect_equal(res_mtrx, res_base, ignore_attr = TRUE)

  # in integer:
  d_integer <- cbind(d, pred_integer = sample(c(0L, 1L), 30, replace = TRUE))
  d_integer$pred_integer[1] <- NA
  expect_no_condition(res_mtrx <- mtrx(outcome ~ ., d_integer))
  res_base <- model.matrix(outcome ~ ., d_integer)
  dimnames(res_base) <- dimnames(res_mtrx)
  expect_equal(res_mtrx, res_base, ignore_attr = TRUE)

  # in factor:
  d_factor <- cbind(d, pred_factor = factor(sample(letters[1:3], 30, replace = TRUE)))
  d_factor$pred_factor[1] <- NA
  expect_no_condition(res_mtrx <- mtrx(outcome ~ ., d_factor))
  res_base <- model.matrix(outcome ~ ., d_factor)
  dimnames(res_base) <- dimnames(res_mtrx)
  expect_equal(res_mtrx, res_base, ignore_attr = TRUE)

  skip("TODO: mtrx() panics with NAs in character columns")
  # in character:
  d_character <- cbind(d, pred_character = sample(letters[1:3], 30, replace = TRUE))
  d_character$pred_character[1] <- NA
  expect_no_condition(res_mtrx <- mtrx(outcome ~ ., d_character))
  res_base <- model.matrix(outcome ~ ., d_character)
  dimnames(res_base) <- dimnames(res_mtrx)
  expect_equal(res_mtrx, res_base, ignore_attr = TRUE)

})

test_that("mtrx() errors informatively with bad input", {
  expect_snapshot(error = TRUE, mtrx(1, 2))
  expect_snapshot(error = TRUE, mtrx(formula, 2))
  expect_snapshot(error = TRUE, mtrx(mpg ~ ., 2))
  expect_snapshot(error = TRUE, mtrx(mpg ~ ., data))
})
