// Adapted from from ruff
// Copyright 2022 Charles Marsh
// SPDX-License-Identifier: MIT

use std::num::NonZeroUsize;

use ruff_diagnostics::Applicability;
use ruff_text_size::{Ranged, TextLen, TextRange, TextSize};
use similar::{ChangeTag, TextDiff};

use crate::diagnostics::Fix;
use crate::diagnostics::stylesheet::{DiagnosticStylesheet, fmt_styled};
use crate::text_helpers::ShowNonprinting;
use ruff_source_file::{OneIndexed, SourceFile};

use crate::Diagnostic;

/// Renders a diff that shows the code fixes.
///
/// The implementation isn't fully fledged out and only used by tests. Before using in production, try
/// * Improve layout
/// * Replace tabs with spaces for a consistent experience across terminals
/// * Replace zero-width whitespaces
/// * Print a simpler diff if only a single line has changed
pub(super) struct Diff<'a> {
    fix: &'a Fix,
    diagnostic_source: &'a SourceFile,
    stylesheet: &'a DiagnosticStylesheet,
}

impl<'a> Diff<'a> {
    pub(super) fn from_diagnostic(
        diagnostic: &'a Diagnostic,
        stylesheet: &'a DiagnosticStylesheet,
    ) -> Option<Diff<'a>> {
        let file = &diagnostic.primary_span_ref()?.file;
        Some(Diff {
            fix: diagnostic.fix()?,
            diagnostic_source: file,
            stylesheet,
        })
    }
}

// Get slices slightly larger than the diff groups, helps to
// minimise changes to the existing diffs
const DIFF_CONTEXT_WINDOW: usize = 4;

impl std::fmt::Display for Diff<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source_code = self.diagnostic_source.to_source_code();
        let source_text = source_code.text();

        let range = TextRange::new(TextSize::ZERO, source_text.text_len());
        let input = source_code.slice(range);

        let mut output = String::with_capacity(input.len());
        let mut last_end = range.start();

        let mut first = None;
        for edit in self.fix.edits() {
            if range.contains_range(edit.range()) {
                first.get_or_insert(edit);
                output.push_str(source_code.slice(TextRange::new(last_end, edit.start())));
                output.push_str(edit.content().unwrap_or_default());
                last_end = edit.end();
            }
        }

        // Some edits were applied, so add diff.
        if let Some(first) = first {
            let both_start = first.start();
            let new_end = output.text_len();

            output.push_str(&source_text[usize::from(last_end)..usize::from(range.end())]);

            // Both slices start from the same point
            let both_line_start = source_code
                .line_index(both_start)
                .saturating_sub(DIFF_CONTEXT_WINDOW);
            let both_start = source_code.line_start(both_line_start);

            let old_end = line_end_after(source_text, last_end);
            let new_end = line_end_after(&output, new_end);

            // Slices just around the changes
            let old_slice = source_code.slice(TextRange::new(both_start, old_end));
            let new_slice = &output[TextRange::new(both_start, new_end)];

            let diff = TextDiff::from_lines(old_slice, new_slice);

            let grouped_ops = diff.grouped_ops(3);

            // Find the new line number with the largest number of digits to align all of the line
            // number separators.
            let line_offset = both_line_start.to_zero_indexed();
            let last_op = grouped_ops.last().and_then(|group| group.last());
            let largest_new = last_op
                .map(|op| op.new_range().end + line_offset)
                .unwrap_or_default();

            let digit_with = OneIndexed::new(largest_new).unwrap_or_default().digits();

            for (idx, group) in grouped_ops.iter().enumerate() {
                if idx > 0 {
                    writeln!(f, "{:-^1$}", "-", 80)?;
                }
                for op in group {
                    for change in diff.iter_inline_changes(op) {
                        let (sign, style, line_no_style, index) = match change.tag() {
                            ChangeTag::Delete => (
                                "-",
                                self.stylesheet.deletion,
                                self.stylesheet.deletion_line_no,
                                None,
                            ),
                            ChangeTag::Insert => (
                                "+",
                                self.stylesheet.insertion,
                                self.stylesheet.insertion_line_no,
                                change.new_index(),
                            ),
                            ChangeTag::Equal => (
                                "|",
                                self.stylesheet.none,
                                self.stylesheet.line_no,
                                change.new_index(),
                            ),
                        };

                        let line = Line {
                            index: index.map(|i| {
                                OneIndexed::from_zero_indexed(i).saturating_add(line_offset)
                            }),
                            width: digit_with,
                        };

                        write!(
                            f,
                            "{line} {sign}",
                            line = fmt_styled(line, self.stylesheet.line_no),
                            sign = fmt_styled(sign, line_no_style),
                        )?;

                        let mut needs_separator = true;
                        for (emphasized, value) in change.iter_strings_lossy() {
                            if needs_separator && !value.trim_end_matches(['\n', '\r']).is_empty() {
                                f.write_str(" ")?;
                                needs_separator = false;
                            }

                            let styled = fmt_styled(value.show_nonprinting(), style);
                            if emphasized {
                                write!(f, "{}", fmt_styled(styled, self.stylesheet.emphasis))?;
                            } else {
                                write!(f, "{styled}")?;
                            }
                        }
                        if change.missing_newline() {
                            writeln!(f)?;
                        }
                    }
                }
            }
        }

        match self.fix.applicability() {
            Applicability::Safe => {}
            Applicability::Unsafe => {
                writeln!(
                    f,
                    "{note}: {msg}",
                    note = fmt_styled("note", self.stylesheet.warning),
                    msg = fmt_styled(
                        "This is an unsafe fix and may change runtime behavior",
                        self.stylesheet.emphasis
                    )
                )?;
            }
            Applicability::DisplayOnly => {
                // Note that this is still only used in tests. There's no `--display-only-fixes`
                // analog to `--unsafe-fixes` for users to activate this or see the styling.
                writeln!(
                    f,
                    "{note}: {msg}",
                    note = fmt_styled("note", self.stylesheet.error),
                    msg = fmt_styled(
                        "This is a display-only fix and is likely to be incorrect",
                        self.stylesheet.emphasis
                    )
                )?;
            }
        }

        Ok(())
    }
}

/// Returns the offset of the line ending of some context window after `offset`
///
/// Same guarantees as [`context_after`]
fn line_end_after(source: &str, offset: TextSize) -> TextSize {
    let bytes = &source.as_bytes()[usize::from(offset)..];

    let line_end = memchr::memchr2_iter(b'\r', b'\n', bytes)
        // Skip `\r` in `\r\n` sequences (only count the `\n`).
        .filter(|&i| bytes[i] == b'\n' || !(bytes[i] == b'\r' && bytes.get(i + 1) == Some(&b'\n')))
        .nth(DIFF_CONTEXT_WINDOW)
        .map(|end| offset.to_u32().saturating_add(end.try_into().unwrap()) as usize)
        .unwrap_or(source.len());
    TextSize::try_from(line_end).expect("offset should be representable as u32")
}

struct Line {
    index: Option<OneIndexed>,
    width: NonZeroUsize,
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.index {
            None => {
                for _ in 0..self.width.get() {
                    f.write_str(" ")?;
                }
                Ok(())
            }
            Some(idx) => write!(f, "{:<width$}", idx, width = self.width.get()),
        }
    }
}
