use std::io::stdin;

/// Convert Fahrenheit to Celsius
///
/// # Arguments
///
/// * `f`: Fahrenheit value
///
/// returns: f64 Celsius Value
///
/// # Examples
///
/// ```
/// f2c(32) = 0.0
/// ```
fn f2c(f: f64) -> f64 {
    (f - 32.) * 5. / 9.
}

/// Convert Celsius to Fahrenheit
///
/// # Arguments
///
/// * `c`: Celsius value
///
/// returns: f64
///
/// # Examples
///
/// ```
/// c2f(0) = 32.0
/// ```
fn c2f(c: f64) -> f64 {
    (c * 9. / 5.) + 32.
}

fn main() {
    println!("Convert Celsius to Fahrenheit");

    loop {
        let mut c_temp = String::new();

        stdin().read_line(&mut c_temp).expect("Failed to read line");

        let c_temp: f64 = match c_temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("Expected a number");
                continue;
            }
        };
        let f_temp = c2f(c_temp);
        println!("{c_temp} Celsius is {f_temp} Fahrenheit");
    }
}
