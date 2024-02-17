obj_is_vector.Vctr <- function(x) inherits(x, "Vctr")
.onLoad <- function(libname, pkgname) {
  vctrs::s3_register("vctrs::obj_is_vector", "Vctr")
}


