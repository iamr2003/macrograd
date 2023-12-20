macro_rules! grad {
    ($($ops:tt)*) => {
        //this implementation may actually be slow if work can be reused
        ($($ops)*,grad_implem!($($ops)*))
    };
}
//
// get functional for basic ML functions, ReLU, sigmoid, etc.

//will need a macro to parenthesize everything, basically make into an AST

// #[recursion_limit = "512"]
macro_rules! grad_implem {

    // constant and basic multiplicaton rules
    ($const:literal) => {
        0.0
    };
    ($var:ident) => {
        1.0
    };
    ($const:literal*$var:ident) => {
        $const
    };

    // need to figure out how to generalize everything's expressions to be recursive
    // ok parens work before I get tt muncher and accum working
    (($($lhs:tt)*) + ($($rhs:tt)*)) => {
        grad_implem!($($lhs)*) + grad_implem!($($rhs)*)
    };

    (($($lhs:tt)*) - ($($rhs:tt)*)) => {
        grad_implem!($($lhs)*) - grad_implem!($($rhs)*)
    };

    //power rule
    ($var:ident.powi($pow:literal)) => {
        (f64::from($pow) * $var.powi($pow - 1))
    };

    // analog for powf can exist

    // log rules
    ($var:ident.ln()) => {
        (1.0 / $var)
    }; // might have a div 0 issue



    //sugar
    ($lhs:ident*$rhs:ident) => {
        (grad_implem!($lhs) * $rhs + $lhs * grad_implem!($rhs))
    };

    //generalized mult rule
    (($($lhs:tt)*) * ($($rhs:tt)*)) => {
        (grad_implem!($($lhs)*) * $($rhs)* + $($lhs)* * grad_implem!($($rhs)*))
    };

    //generalized division rule
    (($($lhs:tt)*) * ($($rhs:tt)*)) => {
        (((grad_implem!($($lhs)*) * $($rhs)* - $($lhs)* * grad_implem!($($rhs)*))) / ( $($rhs)*.powi(2)))
    };

    //the special one, the chain rule
    //has weird syntax with a trailing function
    //this one could use the name of the variable
    (($($inside:tt)*).$func:ident($($args:tt)*)) => {
        ((|x:f64| grad_implem!(x.$func($($args)*)))($($inside)*) *grad_implem!($($inside)*))
    };

    // trig rules
    ($var:ident.sin()) => {
        ($var.cos())
    };
    ($var:ident.cos()) => {
        (-$var.tan())
    };
    ($var:ident.tan()) => {
        (1.0 / ($var.cos() * $var.cos()))
    };
}

// need to figure out the best way to wrap the function

//will need a grouping paren macro
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
