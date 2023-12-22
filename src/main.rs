#[macro_use]
mod grad;

mod nn;

fn main() {
    // let square = |x|{
    //     (x*x,2*x)
    // };

    //should switch to an actual testing framework

    // pow syntax is wack
    let power = |x: f64| grad!(x.powi(4));
    // assert!(power(2)==(16,32));
    // create proper ish test suite at some point
}
