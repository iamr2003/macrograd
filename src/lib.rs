//I'd like to name this test.rs, but it doesn't appear to work like that
#[macro_use]
mod grad;

#[cfg(test)]
mod grad_implem_tests {
    //procedurally generating some of these with something else would be helpful
    #[test]
    fn old_tests() {
        let square = |x: f64| grad!(x * x);
        let three_mul = |x: f64| grad!(3.0 * x);
        let log = |x: f64| grad!(x.ln());
        let powi = |x: f64| grad!(x.powi(3));
        let powi_parts = |x: f64| grad!((x) * ((x) * (x)));
        let nested_powi = |x: f64| grad!(((x) - (1.0)).powi(3));

        let add = |x: f64| grad!((4.0 * x) - ((x) + (3.0)));

        assert_eq!(square(5.0), (25.0, 10.0));
        assert_eq!(three_mul(2.0), (6.0, 3.0));
        assert_eq!(log(2.0), (2.0_f64.ln(), 0.5));
        assert_eq!(powi(2.0), (8.0, 12.0));
        assert_eq!(powi_parts(2.0), (8.0, 12.0));
        assert_eq!(nested_powi(3.0), (8.0, 12.0));
        assert_eq!(add(5.0), (12.0, 3.0));
    }

    #[test]
    fn relu() {
        let relu = |x: f64| grad!((x).max(0.0));
    }
}
