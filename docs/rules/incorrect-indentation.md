# incorrect-indentation (S105)
Fix is always available.

This rule is unstable and in [preview](../preview.md). The `--preview` flag is required for use.

## What it does
Checks that the correct indentation has been used

The complexity of handling semicolons requires that this
rule removes any semicolons used midway through a line

## Why is this bad?
Inconsistent indentation makes Fortran less readable and difficult to
understand the scoping of logic.

## Options
- [`check.indent-width`][check.indent-width]
- [`check.incorrect-indentation.num-indents-for-associate-contents`][check.incorrect-indentation.num-indents-for-associate-contents]
- [`check.incorrect-indentation.num-indents-for-block-contents`][check.incorrect-indentation.num-indents-for-block-contents]
- [`check.incorrect-indentation.num-indents-for-derived-type-contents`][check.incorrect-indentation.num-indents-for-derived-type-contents]
- [`check.incorrect-indentation.num-indents-for-do-contents`][check.incorrect-indentation.num-indents-for-do-contents]
- [`check.incorrect-indentation.num-indents-for-function-contents`][check.incorrect-indentation.num-indents-for-function-contents]
- [`check.incorrect-indentation.num-indents-for-if-contents`][check.incorrect-indentation.num-indents-for-if-contents]
- [`check.incorrect-indentation.num-indents-for-interface-contents`][check.incorrect-indentation.num-indents-for-interface-contents]
- [`check.incorrect-indentation.num-indents-for-module-contents`][check.incorrect-indentation.num-indents-for-module-contents]
- [`check.incorrect-indentation.num-indents-for-program-contents`][check.incorrect-indentation.num-indents-for-program-contents]
- [`check.incorrect-indentation.num-indents-for-select-contents`][check.incorrect-indentation.num-indents-for-select-contents]
- [`check.incorrect-indentation.num-indents-for-submodule-contents`][check.incorrect-indentation.num-indents-for-submodule-contents]
- [`check.incorrect-indentation.num-indents-for-line-continuation`][check.incorrect-indentation.num-indents-for-line-continuation]
- [`check.incorrect-indentation.num-indents-for-subroutine-contents`][check.incorrect-indentation.num-indents-for-subroutine-contents]


[check.indent-width]: ../settings.md#check_indent-width
[check.incorrect-indentation.num-indents-for-associate-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-associate-contents
[check.incorrect-indentation.num-indents-for-block-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-block-contents
[check.incorrect-indentation.num-indents-for-derived-type-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-derived-type-contents
[check.incorrect-indentation.num-indents-for-do-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-do-contents
[check.incorrect-indentation.num-indents-for-function-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-function-contents
[check.incorrect-indentation.num-indents-for-if-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-if-contents
[check.incorrect-indentation.num-indents-for-interface-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-interface-contents
[check.incorrect-indentation.num-indents-for-module-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-module-contents
[check.incorrect-indentation.num-indents-for-program-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-program-contents
[check.incorrect-indentation.num-indents-for-select-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-select-contents
[check.incorrect-indentation.num-indents-for-submodule-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-submodule-contents
[check.incorrect-indentation.num-indents-for-line-continuation]: ../settings.md#check_incorrect-indentation_num-indents-for-line-continuation
[check.incorrect-indentation.num-indents-for-subroutine-contents]: ../settings.md#check_incorrect-indentation_num-indents-for-subroutine-contents

