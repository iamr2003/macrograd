// starting single variable
// eventually might like a way to include header involved, to deal with other vars
macro_rules! grad {
    ($($ops:tt)*) => {
        ($($ops)*,grad_implem!($($ops)*))
    };
}

macro_rules! grad_implem {
    // constant and basic multiplicaton rules
    ($const:literal) => {0};
    ($var:ident)=>{1};
    ($const:literal*$var:ident) => {$const};
    // product rule
    // matching is going to get v fun lol
    // ($lhs:pat*$rhs:pat) => {
    //     (grad_implem!(lhs)*rhs + lhs*grad_implem!(rhs))
    // };
    ($lhs:ident*$rhs:ident) => {
        (grad_implem!($lhs)*$rhs + $lhs*grad_implem!($rhs))
    };
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
    println!("Square: {:?}",square(10));
    // create proper ish test suite at some point
}
