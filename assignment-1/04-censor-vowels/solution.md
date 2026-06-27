# Solution notes: Censor vowels in place

## Approach

For this problem, the main constraint was mutating the string directly without allocating a new one. To do a true in-place edit, I realized I needed to drop down to the byte level. 

I used `unsafe { s.as_bytes_mut() }` to get a mutable slice of the string's underlying bytes. Then, I set up a `for` loop to iterate through each byte using `.iter_mut()`. Inside the loop, I used a `match` statement to check if the current byte matches the byte literal of any ASCII vowel (handling both uppercase and lowercase, like `b'A'` and `b'a'`). If it's a match, I dereference the pointer (`*byte`) and overwrite it with the byte value of an asterisk (`b'*'`).

## Edge cases handled


## Anything special

_Tricks, alternatives you considered, performance notes, etc._
