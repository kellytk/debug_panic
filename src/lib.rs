#![no_std]

/// Entry point of thread panic.  For details on `panic`, see std::macros.
///
/// # Uses
///
/// Unlike [`panic!`], `debug_panic!` statements are only enabled in non
/// optimized builds by default. An optimized build will omit all
/// `debug_panic!` statements unless `-C debug-assertions` is passed to the
/// compiler.
///
/// # Examples
///
/// ```
/// // panic
/// debug_panic!();
/// 
/// // panic with a custom message
/// debug_panic!("panic");
/// ```
#[macro_export]
macro_rules! debug_panic {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { panic!($($arg)*); })
}
