# mtrx() errors informatively with bad input

    Code
      mtrx(1, 2)
    Condition
      Error in `mtrx()`:
      ! `formula` must be a <formula>, not a number.

---

    Code
      mtrx(formula, 2)
    Condition
      Error in `mtrx()`:
      ! `formula` must be a <formula>, not a function.

---

    Code
      mtrx(mpg ~ ., 2)
    Condition
      Error in `mtrx()`:
      ! `data` must be a <data.frame>, not a number.

---

    Code
      mtrx(mpg ~ ., data)
    Condition
      Error in `mtrx()`:
      ! `data` must be a <data.frame>, not a function.

