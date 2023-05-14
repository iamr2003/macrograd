// starting single variable
// eventually might like a way to include header involved, to deal with other vars
macro_rules! grad {
    ($($ops:tt)*) => {
        ($($ops)*,grad_implem!($($ops)*))
    };
}
//
// when I want to write custom syntax for relu or something, 

macro_rules! grad_implem {
    // constant and basic multiplicaton rules
    ($const:literal) => {0};
    ($var:ident)=>{1};
    ($const:literal*$var:ident) => {$const};
    
    // need to figure out how to generalize everything's expressions to be recursive
    // this might get hard


    // power rule
    ($var:ident.powi($pow:literal)) => {
        (f64::from($pow)*$var.powi($pow-1)) //we're using f32 in this proj
    };

    // analog for powf can exist

    // log rules
    ($var:ident.ln()) => {(1.0/$var)}; // might have a div 0 issue


    //divison rule

    // product rule
    // matching is going to get v fun lol
    // ($lhs:pat*$rhs:pat) => {
    //     (grad_implem!(lhs)*rhs + lhs*grad_implem!(rhs))
    // };

    // parens is the best we can do for now I think
    (($lhs:expr)*($rhs:expr)) => {
        ((grad_implem!($lhs))*($rhs) + ($lhs)*(grad_implem!($rhs)))
    };

    ($lhs:ident*$rhs:ident) => {
        (grad_implem!($lhs)*$rhs + $lhs*grad_implem!($rhs))
    };

    // --- important one, the chain rule
    //

    // trig rules
    //
    // match everything as just itself if not implemented
    // SHOULD GET REPLACED WITH MORE EXPLICIT FOR USEFUL PARSING
    // ($all:tt) =>{$all}
}

// need to figure out the best way to wrap the function

fn main() {
    println!("Hello, world!");
    // let square = |x|{
    //     (x*x,2*x)
    // };

    let square = |x| {grad!(x*x)};
    let three_mul = |x| {grad!(3*x)};
    let log = |x:f64| {grad!(x.ln())};
    let powi = |x:f64| {grad!(x.powi(3))};
    let prod = |x:f64| {grad!((x.ln())*(x))};
    // println!("Square: {:?}",square(10));
    assert!(square(5) == (25,10));
    assert!(three_mul(2) == (6,3));
    assert!(log(2.0) == (2.0_f64.ln(),0.5));
    assert!(powi(2.0) == (8.0,12.0));

    
    // pow syntax is wack
    // let power = |x| {grad!(f32::powi(x,4))};
    // assert!(power(2)==(16,32)); 
    // create proper ish test suite at some point
}
