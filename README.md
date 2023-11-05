# type_enum

A Rust macro to generate zero-sized types with a common trait for Typestate
APIs.

## Example

The following example defines a trait ``State`` and 3 zero-sized types which
implement it: ``Ready``, ``Working``, and ``Complete``.

```rust
use type_enum::type_enum;

type_enum! {
    pub State {
        Ready,
        Working,
        Complete
    }
}
```

The types can then be used to build simple Typestate APIs:

```rust
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
