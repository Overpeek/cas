# a Computer Algebra System project in rust

### cas-cli example usage
`cas-cli [-v,-vv] EXPR1 EXPR2 EXPR3 ...`
```
$ cargo run --example=cas-cli "(1-5)*2+3(2(2+2))/2*2+2" "1/2+2/4"
> 18
  1
  (x-68.8)
```

factorizer and solver are WIP



https://en.wikipedia.org/wiki/Factorization_of_polynomials
https://en.wikipedia.org/wiki/Shunting-yard_algorithm
https://en.wikipedia.org/wiki/Quadratic_formula
https://en.wikipedia.org/wiki/Cubic_equation