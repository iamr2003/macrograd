//need a paren macro
macro_rules! grad {
    ($($ops:tt)*) => {
        //this implementation may actually be slow if work can be reused
        ($($ops)*,grad_implem!($($ops)*))
    };
}
//
// get functional for basic ML functions, ReLU, sigmoid, etc.

//will need a macro to parenthesize everything, basically make into an AST

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

    //power rule, analog for powf could exist
    ($var:ident.powi($pow:literal)) => {
        (f64::from($pow) * $var.powi($pow - 1))
    };

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
