# v1.2.4
## Miscellaneous
- Added missing docs for public items

# v1.2.3
- Accidentally skipped due to typo. Oops

# v1.2.2
## Bugfixes
- Fixed `DiceEvaluation::mean_z_score_normalized` always returning a positive value

# v1.2.1
## Features
- Added `statstics` module (gated behind `stats` feature flag) for getting statistical information dice roll results

# v1.1.1
## Tweaks
- `DiceEvaluation`s now have their `RollGroup`s sorted by the order in which they were initially evaluated

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