You are given an array of characters in the form of a `&str`, e.g.
`"ab234dfe3drff"` with length `n`. Your task is to find the (first) longest
subarray of this string slice containing the same amount of digits 
(`0, 1, 2, ...`) and letters (`a, b, c, ...`). In case there is
no such array, i.e. the input contains solely digits or letters, you should
return a `None` value.

For example, in the above sample input, you should return `Some("b234dfe3")`
because, even though `Some("234dfe3d")` also contains the same amount of 
digits and letters, it comes after the first suitable subslice.

Check the time complexity of your solution and try to implement an
algorithm that's better than `O(n<sup>2</sup>)`.
