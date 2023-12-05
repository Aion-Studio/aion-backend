macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

pub fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

#[macro_export]
macro_rules! run_parallel {
    // Entry point for the macro
    ($args:tt; $($func:path),+ $(,)?) => {
        $(
            run_parallel!(@call $args; $func);
        )+
    };

    // Nested macro for making the function call
    (@call ($($arg:expr),*); $func:path) => {
        $func($($arg),*);
    };
}
