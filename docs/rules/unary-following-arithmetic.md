# unary-following-arithmetic (PORT051)
Fix is sometimes available.

This rule is unstable and in [preview](../preview.md). The `--preview` flag is required for use.

## What it does
Checks for use of a unary expression following an arithmetic operator.

## Why is this bad?
The use of a unary operator (`+`, `-` but *not* user-defined) following an arithmetic operator (
`+`, `-`, `*`, `/`, `**` but *not* user-defined) is not part of the Fortran standard. In some
cases, this may be ambiguous as the order of operations does not necessarily follow typical
mathematical order. The computed output may therefore vary between compilers.

Many compilers, Intel Fortran for example, will let you do this as an extension; sometimes with
a warning on compilation. Use parentheses to remove ambiguity of user expected output; the fix
may change prior behaviour on some compilers.

## Example
```f90
x = 10 ** -2 * 2
! Would expected x = 0.02 but some compilers may give x = 0.0001.
```

Use instead:
```f90
x = 10 ** (-2) * 2
! Result is unambiguously x = 0.02.
```

## References
* [Doctor Fortran in "Order! Order!"](https://stevelionel.com/drfortran/2021/04/03/doctor-fortran-in-order-order/)
