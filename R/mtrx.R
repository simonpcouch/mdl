#' Modern model matrices
#'
#' @description
#' `mdl::mtrx()` (read: "model matrix") implements an opinionated and performant
#' reimagining of model matrices. It takes in a formula and data frame,
#' like `model.frame()`, and outputs a numeric matrix of predictor values,
#' like `model.matrix()`.
#'
#' @param formula A formula. Cannot contain inlined functions like `-`or `*`.
#' @param data A data frame.
#'
#' @section Comparison to Base R:
#'
#' Compared to `model.frame(model.matrix())`, `mdl::mtrx()`:
#'
#' * Names its intercept `intercept` rather than `(Intercept)`.
#' * Does not accept formulae with inlined functions (like `-` or `*`).
#' * Names dummy variables created from characters and factors as `colname_level` rather than `colnamelevel`.
#' * Names dummy variables create from logicals as `colname` rather than `colnameTRUE`.
#' * Never drops rows (and thus doesn't accept an `na.action`).
#' * Assumes that factors levels are encoded as they're intended (i.e. `drop.unused.levels` and `xlev` are not accepted).
#'
#' @examples
#' mdl::mtrx(mpg ~ ., mtcars)
#'
#' @export
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
