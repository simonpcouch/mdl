% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/mtrx_compatible.R
\name{mtrx_compatible}
\alias{mtrx_compatible}
\title{Check if \code{mtrx()} will give matching results to \code{model.matrix()}}
\usage{
mtrx_compatible(data, na_action = getOption("na.action"))
}
\arguments{
\item{data}{A data frame.}

\item{na_action}{What to do about missing values--see \code{\link[=na.pass]{na.pass()}}. Can be
either a function or character, just as the option can be.}
}
\value{
A logical value. Returns TRUE if \code{\link[=mtrx]{mtrx()}} will return a similar
matrix to \code{\link[=model.matrix]{model.matrix()}}, and \code{FALSE} otherwise.
}
\description{
When the following are true, \code{\link[=mtrx]{mtrx()}} returns a matrix that matches
that returned by \code{\link[=model.matrix]{model.matrix()}} :
\itemize{
\item There are no non-default factor contrasts.
\item Every column is one of a numeric, integer, character, or factor.
\item The \code{na.action} option is \code{"na.pass"}.
}

In that case, in code where \code{\link[=model.matrix]{model.matrix()}} results in slowdowns, one can
write:

\if{html}{\out{<div class="sourceCode">}}\preformatted{if (mtrx_compatible(data)) \{
  res <- mtrx(formula, data)
\} else \{
  res <- model.matrix(formula, data)
\}
}\if{html}{\out{</div>}}
}
\details{
In this case, "matching" output means that dummy variables will be encoded
in the same way, missing values will be handled in the same way (returned
as-is), and that \code{\link[=mtrx]{mtrx()}} won't error due to the presence of
unsupported column types.
}
\examples{
# Compatible data frame
df1 <- data.frame(a = 1:3, b = letters[1:3], c = factor(letters[1:3]))
mtrx_compatible(df1)

# Incompatible due to non-default contrasts
df2 <- df1
contrasts(df2$c) <- "contr.sum"
mtrx_compatible(df2)

# Incompatible due to unsupported column type
df3 <- data.frame(a = 1:3, b = as.Date(c("2023-01-01", "2023-01-02", "2023-01-03")))
mtrx_compatible(df3)

# Missing values are problematic only when na_action is not `na.pass`
df4 <- data.frame(a = c(1, NA, 3), b = letters[1:3])
mtrx_compatible(df4, na_action = "na.omit")
mtrx_compatible(df4, na_action = "na.pass")

}
