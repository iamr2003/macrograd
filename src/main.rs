#[macro_use]
mod grad;

fn main() {
    // let square = |x|{
    //     (x*x,2*x)
    // };

    //should switch to an actual testing framework
    let square = |x: f64| grad!(x * x);
    let three_mul = |x: f64| grad!(3.0 * x);
    let log = |x: f64| grad!(x.ln());
    let powi = |x: f64| grad!(x.powi(3));
    let powi_parts = |x: f64| grad!((x) * ((x) * (x)));
    let nested_powi = |x: f64| grad!(((x) - (1.0)).powi(3));

    let add = |x: f64| grad!((4.0 * x) - ((x) + (3.0)));

    // let sin = |x:f64| {grad!(x.sin())};
    // let cos = |x:f64| {grad!(x.cos())};
    // let tan = |x:f64| {grad!(x.tan())};

    // let prod = |x:f64| {grad!((x.ln())*(x))};
    // println!("Square: {:?}",square(10));
    assert!(square(5.0) == (25.0, 10.0));
    assert!(three_mul(2.0) == (6.0, 3.0));
    assert!(log(2.0) == (2.0_f64.ln(), 0.5));
    assert!(powi(2.0) == (8.0, 12.0));
    assert!(powi_parts(2.0) == (8.0, 12.0));
    assert!(nested_powi(3.0) == (8.0, 12.0));
    assert!(add(5.0) == (12.0, 3.0));

    println!("Passed all tests");

    // pow syntax is wack
    // let power = |x| {grad!(f32::powi(x,4))};
    // assert!(power(2)==(16,32));
    // create proper ish test suite at some point
}
