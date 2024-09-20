#' Check if `mtrx()` will give analogous results to `model.matrix()`
#'
#' @description
#' When the following are true, [mtrx()] returns a matrix that is similar to
#' that returned by [model.matrix()] :
#'
#' * There are no non-default factor contrasts.
#' * Every column is one of a numeric, integer, character, or factor.
#' * There are no missing values.
#'
#' In that case, in code where [model.matrix()] results in slowdowns, one can
#' write:
#'
#' ```
#' if (mtrx_compatible(data)) {
#'   res <- mtrx(formula, data)
#' } else {
#'   res <- model.matrix(formula, data)
#' }
#' ```
#'
#' @param data A data frame.
#'
#' @return A logical value. Returns TRUE if [mtrx()] will return a similar
#'   matrix to [model.matrix()], and `FALSE` otherwise.
#'
#' @details
#' In this case, "similar" output means that dummy variables will be encoded
#' in the same way and that [mtrx()] won't error due to the presence of
#' missing values or unsupported column types.
#'
#' @examples
#' # Compatible data frame
#' df1 <- data.frame(a = 1:3, b = letters[1:3], c = factor(letters[1:3]))
#' mtrx_compatible(df1)
#'
#' # Incompatible due to non-default contrasts
#' df2 <- df1
#' contrasts(df2$c) <- "contr.sum"
#' mtrx_compatible(df2)
#'
#' # Incompatible due to unsupported column type
#' df3 <- data.frame(a = 1:3, b = as.Date(c("2023-01-01", "2023-01-02", "2023-01-03")))
#' mtrx_compatible(df3)
#'
#' # Incompatible due to missing values
#' df4 <- data.frame(a = c(1, NA, 3), b = letters[1:3])
#' mtrx_compatible(df4)
#'
#' @export
mtrx_compatible <- function(data) {
  if (has_unsupported_column_types(data)) {
    return(FALSE)
  }

  if (has_missing_values(data)) {
    return(FALSE)
  }

  if (has_non_default_contrasts(data)) {
    return(FALSE)
  }

  TRUE
}

has_unsupported_column_types <- function(data) {
  res <- FALSE

  for (col in data) {
    if (!inherits(col, c("numeric", "integer", "character", "factor"))) {
      res <- TRUE
      break
    }
  }

  res
}

has_missing_values <- function(data) {
  !all(vctrs::vec_detect_complete(data))
}

default_contrasts <-
  list(contrasts = c(unordered = "contr.treatment", ordered = "contr.poly"))

has_non_default_contrasts <- function(data) {
  if (!identical(options("contrasts"), default_contrasts)) {
    return(TRUE)
  }

  data_contrasts <- attr(data, "contrasts", exact = TRUE)
  if (!is.null(data_contrasts) && !identical(data_contrasts, default_contrasts)) {
    return(TRUE)
  }

  res <- FALSE
  for (col in data) {
    if (inherits(col, "factor")) {
      col_contrasts <- attr(col, "contrasts", exact = TRUE)
      if (!is_null(col_contrasts) && !identical(col_contrasts, default_contrasts)) {
        res <- TRUE
      }
      break
    }
  }

  res
}
