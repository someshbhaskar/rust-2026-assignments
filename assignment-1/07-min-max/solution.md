# Solution notes: `min_max`

## Approach

1. Initialization: By returning early on an empty slice, it is now mathematically safe to initialize our tracking variables, `min` and `max`, using the first element of the slice (`xs[0]`).
2. The Manual Loop: We iterate over the slice using a standard `for` loop. By using pattern matching (`for &x in xs`), we destructure the `&i32` references yielded by the slice iterator directly into owned `i32` values. 
3. Updating Locals: Inside the loop, two simple `if` statements compare the current element against the running `min` and `max`. If an element is strictly smaller or strictly larger, the respective local variable is overwritten.
4. Return: Finally, the updated local variables are wrapped in a tuple and returned inside `Some()`.

## Edge cases handled

Edge Case Handling: The very first step is to check if the slice is empty (`xs.is_empty()`). If it is, we immediately return `None`. This prevents a panic in the next step and satisfies the empty input requirement.

## Anything special

_Tricks, alternatives you considered, performance notes, etc._
