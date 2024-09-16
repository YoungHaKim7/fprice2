// 문서용 설명 작성
//! # 3자리 숫자마다 콤마(,) 찍어주는 api
//!
//! # Example
//! ```rs
//! let mut price_comma = PriceComma::new(0, "".to_string()) ;
//! price_comma.push(11111178);
//! println!("{}", price_comma.fmt_number();
//! ```
//!
//! # Result
//! ```bash
//!  (fn new)i32 Formatted number: 11,111,178
//! ```

#[derive(Debug)]
struct PriceComma<T> {
    data: T,
    result: String,
}

trait Formatter {
    fn fmt_number(&self) -> String;
    fn big_num_i64_str(&self) -> String;
    fn fmt_num_f64_str(&self) -> String;
    fn fmt_num_str(&self) -> String;
}

impl<T> PriceComma<T> {
    fn new(data: T, result: String) -> Self {
        Self { data, result }
    }
    fn push(&mut self, data: T) {
        self.data = data;
    }
}

impl<T: Default> Default for PriceComma<T> {
    fn default() -> Self {
        Self::new(T::default(), String::new())
    }
}

impl<T: ToString> Formatter for PriceComma<T> {
    // i32 fn
    fn fmt_number(&self) -> String {
        let mut formatted_number = String::new();
        let mut count = 0;
        let number_convert = self.data.to_string();

        // Iterate through the characters of the number from right to left
        for c in number_convert.chars().rev() {
            if count == 3 {
                formatted_number.push(',');
                count = 0;
            }
            formatted_number.push(c);
            count += 1;
        }

        formatted_number.chars().rev().collect()
    }

    // big number
    fn big_num_i64_str(&self) -> String {
        let mut formatted_number = String::new();
        let mut count = 0;
        let number_convert = self.data.to_string();

        // Iterate through the characters of the number from right to left
        for c in number_convert.chars().rev() {
            if count == 3 {
                formatted_number.push(',');
                count = 0;
            }
            formatted_number.push(c);
            count += 1;
        }

        // Reverse the formatted number back to the original order
        formatted_number.chars().rev().collect()
    }

    // 123.45 소수점 콤마 표시fn
    fn fmt_num_f64_str(&self) -> String {
        let mut formatted_number = String::new();
        let mut count = 0;
        let number_convert = self.data.to_string();
        let mut number_point_raw = number_convert.split('.');

        let integer_part = number_point_raw.next().unwrap();
        let decimal_part = number_point_raw.next().unwrap_or("00");

        // Iterate through the characters of the integer part from right to left
        for c in integer_part.chars().rev() {
            if count == 3 {
                formatted_number.push(',');
                count = 0;
            }
            formatted_number.push(c);
            count += 1;
        }

        // Reverse the formatted number back to the original order
        formatted_number = formatted_number.chars().rev().collect();

        // Combine the integer and decimal parts
        formatted_number.push('.');
        formatted_number.push_str(decimal_part);

        formatted_number
    }

    // "123" String 숫자 콤마 표시fn
    fn fmt_num_str(&self) -> String {
        let number = self.data.to_string();
        let mut formatted_number = String::new();
        let mut count = 0;

        // Iterate through the characters of the number from right to left
        for c in number.chars().rev() {
            if count == 3 {
                formatted_number.push(',');
                count = 0;
            }
            formatted_number.push(c);
            count += 1;
        }

        // Reverse the formatted number back to the original order
        formatted_number.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_new_fn_test() {
        let mut price_comma = PriceComma::new(0, "".to_string());
        price_comma.push(11111178);

        let formatted_number = price_comma.fmt_number();
        println!("(fn new)i32 Formatted number: {}", formatted_number);
    }

    #[test]
    fn i32num_test() {
        let price_comma = PriceComma {
            data: 123456789,
            result: String::new(),
        };

        let formatted_number = price_comma.fmt_number();
        println!("i32 Formatted number: {}", formatted_number);
        assert_eq!("123,456,789", formatted_number);
    }

    #[test]
    fn i64num_big_num_test() {
        let price_comma = PriceComma {
            data: 1234567891233551324234234_i128,
            result: String::new(),
        };

        let formatted_number = price_comma.big_num_i64_str();
        println!("i128 (Big Number)Formatted number: {}", formatted_number);
        assert_eq!("1,234,567,891,233,551,324,234,234", formatted_number);
    }

    #[test]
    fn minus_i64num_big_num_test() {
        let price_comma = PriceComma {
            data: -1234567891233551324234234_i128,
            result: String::new(),
        };

        let formatted_number = price_comma.big_num_i64_str();
        println!(
            "-i128 (Big Number, Minus)Formatted number: {}",
            formatted_number
        );
        assert_eq!("-1,234,567,891,233,551,324,234,234", formatted_number);
    }

    #[test]
    fn f64num_test() {
        let price_comma = PriceComma {
            data: 123456.789,
            result: String::new(),
        };

        let formatted_number = price_comma.fmt_num_f64_str();
        println!("f64 Number Formatted: {}", formatted_number);
        assert_eq!("123,456.789", formatted_number);
    }

    #[test]
    fn string_num_test() {
        let price_comma = PriceComma {
            data: "12345667".to_string(),
            result: String::new(),
        };

        let formatted_number = price_comma.fmt_num_str();
        println!("String Number Formatted: {}", formatted_number);
        assert_eq!("12,345,667", formatted_number);
    }
}
