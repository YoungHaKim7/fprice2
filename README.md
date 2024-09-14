# fprice
financial_price_indication


# 3자리 숫자마다 콤마(,) 찍어주는 api

- https://github.com/YoungHaKim7/fprice2

# Example
```rs
let int = 123456778;
let a = fprice::fmt_num_i64_str(int);
println!("{}", a);
```

# Result
```bash
 123,456,778
```


```rs
fn fmt_number(&self) -> String;
fn big_num_i64_str(&self) -> String;
fn fmt_num_f64_str(&self) -> String;
fn fmt_num_str(&self) -> String;
```
