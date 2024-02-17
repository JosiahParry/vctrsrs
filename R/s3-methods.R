#' @export
.DollarNames.Vctr = function(env, pattern = "") {
  ls(Vctr, pattern = pattern)
}

#' @export
format.Vctr <- function(x, ...) {
  x$show()
}

#' @export
print.Vctr <- function(x, ...) {
  cat(sprintf("<Vctr[%i]>", length(x)), "\n")
  print(format(x, ...))
}


#' @export
length.Vctr <- function(x, ...) {
  x$length()
}

#' @export
`[.Vctr` <- function(x, i, ...) {
  i <- as.integer(i)
  x$subset(i)
}

#' @export
`[[.Vctr` <- function(x, i, ...) {
  i <- as.integer(i)
  x$subset(i)
}


#' @export
as.data.frame.Vctr <- function(x, ...) {
  x_name <- deparse1(substitute(x))
  n_row <- length(x)
  structure(list(x = x), row.names = 1:n_row, class = "data.frame")
}


#'
#' #' @import vctrs
#' #' @export
#' vec_proxy.Vctr <- function(x, ...) format(x)
#'
#' #' @export
#' vec_restore.Vctr <- function(x, ...) new_usize(as.integer(x))
#'

# For compatibility with vctrs it seems like the only way to do that
# is going to be if the object is a true "vector" in R's eye.
# so thats going to be a list, or any other atomic. In this case
# a list is the only vector type that can hold a pointer so voila
# thats what we're stuck with

# Let call the class "RVctr"
# RVctr will be a list with a single element with is a pointer
# to a Rust Vctr<T: Rvctr>(pub T) which is a tuple struct with a
# single element that must implement the Rvctr trait


#
# x$length()
#
# x$subset(0L)$show()
# x$subset(NA_integer_)
#
# x$new(sample(0:1e7, 1000))
#
# Reduce(`+`, c(1, 2, 9, 10))
