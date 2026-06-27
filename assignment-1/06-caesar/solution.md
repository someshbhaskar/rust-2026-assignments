# Solution notes: Caesar cipher

## Approach

1.  Shift Normalization (The Euclidean Modulo): 
    In Rust (and many other C-like languages), the `%` operator is a *remainder* operator, not a true mathematical modulo. This means `-1 % 26` evaluates to `-1` instead of `25`. To smoothly handle negative shifts and shifts larger than the alphabet size, we normalize the shift using the formula `((shift % len) + len) % len`. This guarantees our `effective_shift` is always a positive integer strictly between `0` and `25`.
2.  Iterate and Transform: 
    We iterate over the string using `.chars().map(...)`. Inside the map:
    - We check if the character is lowercase or uppercase using `.is_ascii_lowercase()` and `.is_ascii_uppercase()`.
    - If it's a letter, we find its 0-indexed position (e.g., `'a'` -> `0`, `'c'` -> `2`) by subtracting the byte value of `'a'` (or `'A'`).
    - We add our `effective_shift` to this position, modulo `ALPHABET.len()`, to wrap around the end of the alphabet.
    - We use this new index to fetch the target character directly from `ALPHABET.as_bytes()`. If the original character was uppercase, we convert the fetched character to uppercase.
    - Non-alphabetic characters fall through to the `else` block and are returned completely untouched.
3.  Collect: 
    Finally, we `.collect()` the resulting characters back into a dynamically allocated `String`.

## Edge cases handled


## Anything special

_Tricks, alternatives you considered, performance notes, etc._
