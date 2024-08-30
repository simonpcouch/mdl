mtrx <- function(formula, data) {
  if (!is_formula(formula, scoped = TRUE)) {
    cli_abort(
      "{.arg formula} must be a {.cls formula}, not {.obj_type_friendly {formula}}."
    )
  }

  # TODO: check for inlined functions in formula, as in recipes

  if (!inherits(data, "data.frame")) {
    cli_abort(
      "{.arg data} must be a {.cls data.frame}, not {.obj_type_friendly {data}}."
    )
  }

  model_matrix(data[predictors(formula, data)])
}

# adapted from recipes:::get_rhs_vars()
predictors <- function(formula, data) {
  outcomes_names <- all.names(
    rlang::f_lhs(formula),
    functions = FALSE,
    unique = TRUE
  )

  predictors_names <- all.names(
    rlang::f_rhs(formula),
    functions = FALSE,
    unique = TRUE
  )

  if (any(predictors_names == ".")) {
    predictors_names <- predictors_names[predictors_names != "."]
    predictors_names <- c(predictors_names, colnames(data))
    predictors_names <- unique(predictors_names)
    predictors_names <- predictors_names[!predictors_names %in% outcomes_names]
  }

  predictors_names
}
