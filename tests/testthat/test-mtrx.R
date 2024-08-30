test_that("model_matrix gives similar results to model.matrix", {
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
})
