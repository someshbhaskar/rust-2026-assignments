# Solution notes: Reverse the word order

## Approach

1. `sentence.split_whitespace()`: This creates an iterator that extracts words from the input string. It is highly efficient because it automatically ignores leading/trailing whitespace and treats consecutive spaces as a single boundary.

2. `.rev()`: Reverses the direction of the iterator so we process the last words first.

3. `.collect()`: This consumes the iterator. By explicitly typing the variable as let vec: Vec<&str>, we tell the Rust compiler to store these reversed word references into a temporary Vector.

4. `vec.join(" ")`: This takes our Vector of reversed words and allocates a single, new String. It automatically inserts exactly one space between each word, guaranteeing clean formatting.

5. return result;: Finally, we hand the newly constructed string back to the caller.

## Edge cases handled


## Anything special

_Tricks, alternatives you considered, performance notes, etc._
