# v1.1.0
## Changes
- Reworked how comparison operators work. Comparisons are now properly parsed as ternary operators, and now no operators can produce
an error at evaluation-time. Due to received feedback, the comparison operators are also the lowest priority operators. Their priority
is still subject to change.

# v1.0.0
## Features
Absolute necessities for the library now exist including:
- Added dice operator (`D`/`d`)
- Added basic math operators (`+`, `-`, `*`, `/`, `%`, `^`)
- Added comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)