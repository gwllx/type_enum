# typestate-enum

A Rust macro to help build simple Typestate APIs.

## Example

The following example defines a trait ``State`` and 3 zero-sized types which
implement it: ``Ready``, ``Working``, and ``Complete``. The types can then be
used to build simple Typestate APIs.

```rust
use typestate_enum::typestate_enum;
use std::marker::PhantomData;

typestate_enum! {
    pub State {
        Ready,
        Working,
        Complete
    }
}

struct Action<S: State>(PhantomData<S>);

impl<S: State> Action<S> {
    fn new() -> Self {
        Action::<S>(PhantomData)
    }
}

impl Action<Ready> {
    fn start_work(self) -> Action<Working> {
        Action::new()
    }
}

impl Action<Working> {
    fn complete_work(self) -> Action<Complete> {
        Action::new()
    }
}
```
