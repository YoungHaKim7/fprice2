# fprice
financial_price_indication

# 3자리 숫자마다 콤마(,) 찍어주는 api

- https://github.com/YoungHaKim7/fprice2

# Example
```rs
let mut price_comma = PriceComma::new(0, "".to_string()) ;
price_comma.push(11111178);
println!("{}", price_comma.fmt_number();
```
# Result
```bash
 (fn new)i32 Formatted number: 11,111,178
```



```rs
fn fmt_number(&self) -> String;
fn big_num_i64_str(&self) -> String;
fn fmt_num_f64_str(&self) -> String;
fn fmt_num_str(&self) -> String;
```
