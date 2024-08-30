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

test_that("mtrx() errors informatively with bad input", {
  expect_snapshot(error = TRUE, mtrx(1, 2))
  expect_snapshot(error = TRUE, mtrx(formula, 2))
  expect_snapshot(error = TRUE, mtrx(mpg ~ ., 2))
  expect_snapshot(error = TRUE, mtrx(mpg ~ ., data))
})
