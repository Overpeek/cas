# a Computer Algebra System project in rust

### cas-cli example usage
`cas-cli a/-v/-vv EXPR1 EXPR2 EXPR3 ...`
'a' accepts anything ex. -
```
$ cargo run --example=cas-cli -- - '(1-5)*2+3(2(2+2))/2*2+2' '1/2+2/4'
> 18
  1
  (x-68.8)
```
```
$ cargo run --example=cas-cli -- -vv '5*4+3'
> ...
  Evaluated: 23 ...
```

TODO:
- factorizer
- solver
- rationals (to fix floating point errors with for ex. '1/3+1/3+1/3-1' being equal to '-0.00000000000000011102230246251565')



https://en.wikipedia.org/wiki/Factorization_of_polynomials
https://en.wikipedia.org/wiki/Shunting-yard_algorithm
https://en.wikipedia.org/wiki/Quadratic_formula
https://en.wikipedia.org/wiki/Cubic_equation