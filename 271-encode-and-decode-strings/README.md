# 271. Encode and Decode Strings

## Description

Design an algorithm to encode a list of strings to a string. The encoded string is then sent over the network and is decoded back to the original list of strings.

Machine 1 (sender) has the function:

```rust
fn encode(&self, strs: Vec<String>) -> String 
```

Machine 2 (receiver) has the function:

```rust 
fn decode(&self, s: String) -> Vec<String>
```

So Machine 1 does:

```rust
let encoded_string: String = Codec::new().encode(strs);
```

and sends the encoded message to Machine 2 which does:

```rust
let strs2: Vec<String> = Codec::new().decode(encoded_string);
```

`strs2` in Machine 2 should be the same as `strs` in Machine 1.

Implement the `encode` and `decode` methods.

You are not allowed to solve the problem using any serialize methods (such as eval).

### Example 1:

```
Input: dummy_input = ["Hello","World"]
Output: ["Hello","World"]
```

### Example 2:

```
Input: dummy_input = [""]
Output: [""]
```

### Constraints:

- `1 <= strs.length <= 200`
- `0 <= strs[i].length <= 200`
- `strs[i] contains any possible characters out of 256 valid ASCII characters.`
 

### Follow up:

Could you write a generalized algorithm to work on any possible set of characters?
