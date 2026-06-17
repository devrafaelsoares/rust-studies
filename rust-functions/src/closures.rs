/// Demonstrates closures in Rust — anonymous functions that can capture
/// variables from their enclosing scope.
///
/// Closures in Rust can capture values by:
/// - Borrowing immutably (`&T`)
/// - Borrowing mutably (`&mut T`)
/// - Taking ownership (`T`)

/// Returns a closure that adds a given base value to its argument.
pub fn make_adder(base: i32) -> impl Fn(i32) -> i32 {
    move |x| base + x
}

/// Returns a closure that multiplies its argument by a factor.
pub fn make_multiplier(factor: f64) -> impl Fn(f64) -> f64 {
    move |x| x * factor
}

/// Applies a transformation function to each element of a slice,
/// returning a new Vec with the results.
pub fn transform<T, U, F>(items: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    items.iter().map(|item| f(item)).collect()
}

/// Composes two functions: applies `f` first, then `g` to the result.
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_adder() {
        let add_five = make_adder(5);
        assert_eq!(add_five(3), 8);
        assert_eq!(add_five(0), 5);
        assert_eq!(add_five(-5), 0);
    }

    #[test]
    fn test_make_multiplier() {
        let double = make_multiplier(2.0);
        assert!((double(3.0) - 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_transform() {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled = transform(&numbers, |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_compose() {
        let add_one = |x: i32| x + 1;
        let double = |x: i32| x * 2;
        let add_one_then_double = compose(add_one, double);
        assert_eq!(add_one_then_double(3), 8);
    }
}
