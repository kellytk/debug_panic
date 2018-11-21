# debug_panic
This allows a program to terminate immediately and provide feedback to the caller of the program.  Unlike panic!, debug_panic! statements are only enabled in non optimized builds by default.

## Example usage
```
// panic
debug_panic!();

// panic with a custom message
debug_panic!("panic");
```
