# Solution notes: Group anagrams

## Approach

1.  Generating the Signature: As we iterate through the input slice, we take each word and convert it to lowercase (`to_ascii_lowercase()`) to ensure case-insensitivity. We then convert this to a byte vector (`into_bytes()`), sort it, and convert it back to a `String`. This sorted string acts as our unique signature.
2.  Grouping: We use a `HashMap<String, Vec<String>>`. We use the `entry` API to look up the signature. If it doesn't exist, `or_default()` initializes an empty `Vec`. We then `.push()` a clone of the original, unmodified word into this vector. Because we push words as we encounter them, the input order within each group is naturally preserved.
3.  Returning Results: Finally, we call `into_values()` on the HashMap to consume it and yield an iterator over its values (the inner `Vec<String>`s), which we `collect()` into our final `Vec<Vec<String>>`.

## Edge cases handled


## Anything special

_Tricks, alternatives you considered, performance notes, etc._
