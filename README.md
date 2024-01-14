# 🎲 Saikoro
**Saikoro** is a library for evaluating dice rolls with a syntax similar to the [Dice Notation](https://en.wikipedia.org/wiki/Dice_notation)
used by many tabletop RPGs with a few (maybe) convenient extras thrown in. Expressions are treated as mathemical expressions
where the `D` or `d` operator is used for rolling dice, and is a high-priority operator. It also comes with a very (painfully) simple 
command-line executable implementation.

# Code Example
A very basic usage of Saikoro's library would look something like
```rust
fn main() {
    if let Ok(roll) = saikoro::evaluate("8d6") {
        println!("Fireball deals {} fire damage", roll.value),
    }
}
```

# Currently Implemented Features
- Dice operator for rolling (`D`/`d`)
- Common mathematical operators
  - Addition & Subtraction (`+` & `-`)
  - Multiplication & Division (`*` & `/`)
  - Modulo (`%`)
  - Exponentiation (`^`)
- Filtering comparison operators to conditionally remove dice rolls (`==`, `!=`, `<`, `>`, `<=`, `>=`)
- Individual roll operations returned and able to be individually used, not just final totals
  - Rolls 'removed' by the aforementioned filtering operators do actually get removed from the struct returned, just flagged as removed
  and not counted toward the total. As such, you can still see what was rolled even if it was 'removed'

# Syntax Documentation
## Binary Operator Priority
Elements higher in the list are evaluated before those lower in the list
 1. Dice `D`/`d`
 3. Exponentiation `^`
 4. Multiplicative `*` `/` `%`
 5. Additive `+` `-`

Unary `+` and `-` are implemented, as well as a unary `D`/`d` operator 

Unary `D`/`d` works the same as a binary `D`/`d` with `1` on the left-hand side (i.e. `d20` is equivalent to `1d20`)

## Comparison Operators
Comparison operators will remove elements in the left-hand roll item where the value does not meet the filter critera implied by the operator.  
For example: `5d8 > 5` will cause the `5d8` result to only count rolls with a value greater than 5
(eg. if `5d8` would produce `{1, 3, 4, 5, 8}`), the final total will be `13`, as the `1` `3` and `4` will be filtered out)

`D`/`d` + comparison operator is treated as a ternary operator with the absolute lowest priority (i.e. the comparison operator will
consider as its right-hand side, the entirety of the rest of the expression unless parentheses are used)

For example:
```rust
// this will produce an error variant at parse-time, as the left-hand side is a constant 6
let result = saikoro::evaluate("6 > 3");
assert!(result.is_err());

// this will also produce an error variant at parse-time, as after the addition, the `2d6` is a value, not a roll expression
let maybe_unintuitive = saikoro::evaluate("(2d6 + 5) > 9");
assert!(maybe_unintuitive.is_err());
```

# Planned Features
- `-H` and `-L` syntax for removing the highest/lowest value of a particular roll
- A means of applying operators to dice rolls element-wise instead of on simple total (syntax undecided)
- Common mathematical functions (eg. `abs`, `sin`, `max`, `log`)
- Support for custom function definitions for parser to use
- Macro for buidling fixed expressions at compile-time
