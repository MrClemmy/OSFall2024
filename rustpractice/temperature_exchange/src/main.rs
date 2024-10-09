

const FAHRENHEIT:f64 =32.0;

fn fahrenheit_to_celsius(f:f64) -> f64{
    let result = (f - FAHRENHEIT)*5.0/9.0;
    return result;
}

fn celsius_to_fahrenheit(g:f64) -> f64{
    let result = (9.0/5.0)*g + FAHRENHEIT;
    return result;
}

fn main() {
    let mut x = fahrenheit_to_celsius(104.0);
    println!("{} degrees celsius",x);
    for _i in 0..5{
        x = celsius_to_fahrenheit(x);
        x += 1.0;
        x = fahrenheit_to_celsius(x);
        println!("{} degrees celsius",x);
    }
}
