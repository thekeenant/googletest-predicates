# googletest-predicates

Provides a function to convert [googletest matchers](https://docs.rs/googletest/latest/googletest/) into [predicates](https://docs.rs/predicates/latest/predicates). This enables you to write write your rich matcher logic using the `googletest` library and reuse it in other libraries which expect `predicates`.

For example, when using `mockall` for mocking types, you can use the `googletest` matcher, `eq` by wrapping it in the `matcher` function provided by this library:

```rust
use googletest::{expect_that, matchers::*};
use googletest_predicates::matcher;

let mut mock = MockMyTrait::new();
mock.expect_foo()
    .with(matcher(eq(1)))
    .times(1)
    .returning(|x| x + 1);
expect_that!(mock.foo(1), eq(2))
```
