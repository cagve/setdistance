# Counter examples
## AX7

```
let val1 = vec!["1000000111111111111".to_string(), "1000100111111111111".to_string(), "1000010111111111111".to_string(),"1000001111111111111".to_string()];
let val2 = vec!["0000000000001111111".to_string(), "1000100111111111111".to_string(), "1000010111111111111".to_string(),"1000001111111111111".to_string()];
let val3 = vec!["1100000000000000000".to_string(), "1110000000000000000".to_string(),"1111000000000000000".to_string()];
let val4 = vec!["0100000000000000000".to_string(), "0110000000000000000".to_string(),"0111000000000000000".to_string()];
```

# REP DIS TRI
```
Some([["110", "101", "111"], ["100", "010", "101", "000", "001"], ["011", "000", "111", "010", "001", "101"]])
let val1 = vec!["110".to_string(), "101".to_string(), "111".to_string()];
let val2 = vec! ["100".to_string(), "010".to_string(), "101".to_string(), "000".to_string(), "001".to_string()];
let val3 = vec! ["011".to_string(), "000".to_string(), "111".to_string(), "010".to_string(), "001".to_string(), "101".to_string()];
```

# idisrec tri
 ```

 counterexample: Some([["01"], ["11", "00"], ["01", "11", "10", "00"]])
 ```
 
# Counterexample 19-06-2023
## Axiom 9-9'

```
let val1 = vec!["000110".to_string(), "000101".to_string(), "000111".to_string()];
let valz = vec!["111101".to_string(), "111000".to_string(), "111001".to_string(), "111010".to_string(), "111100".to_string()];
let valZ = vec!["000110".to_string(), "000101".to_string(), "000111".to_string(), "111000".to_string(), "111001".to_string(), "111010".to_string(), "111100".to_string()]; 
```

