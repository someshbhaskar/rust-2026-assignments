# Solution notes: Run-length encode

## Approach

1. Initialization: I create an empty result vector `res` and convert the input string into a character iterator `chars`.

2. First Character: I use a `match` statement on `chars.next()` to grab the very first character and store it in `curr_char`. 

3. Counting Loop: I initialize a `count` to 1 and loop through the remaining characters. 
   - If the current character `c` matches `curr_char`, I increment `count` by 1 (`count = count + 1`).
   - If `c` is different, it means the current run has ended. I push the `(curr_char, count)` tuple into `res`, update `curr_char` to the new character `c`, and reset `count` to 1.
   
4. Final Push: Because the loop finishes without pushing the very last run, I perform one final `res.push((curr_char, count))` after the loop ends before returning `res`.

## Edge cases handled


## Anything special

_Tricks, alternatives you considered, performance notes, etc._
