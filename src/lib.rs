#![doc = include_str!("../README.md")]
/// Provides zero-sized types which implement a common trait.
#[macro_export]
macro_rules! type_enum {
    (@elem
        $vis:vis $name:ident {
            $(#[$meta:meta])*
            $elem:ident,
            $(
                $(#[$tail_meta:meta])*
                $elems:ident
            ),+
        }
    ) => {
        type_enum!(@elem
            $vis $name {
                $(#[$meta])*
                $elem
            }
        );
        type_enum!(@elem
            $vis $name {
                $(
                    $(#[$tail_meta])*
                    $elems
                ),+
            }
        );
    };
    (@elem
        $vis:vis $name:ident {
            $(#[$meta:meta])*
            $elem:ident
        }
    ) => {
        $(#[$meta])*
        $vis struct $elem;
        impl $name for $elem {}
    };
    (
        $(#[$outer_meta:meta])*
        $vis:vis $name:ident {
            $(
                $(#[$inner_meta:meta])*
                $elems:ident
            ),+
            $(,)?
        }
    ) => {
        $(#[$outer_meta])*
        $vis trait $name {}

        type_enum!(@elem
            $vis $name {
                $(
                    $(#[$inner_meta])*
                    $elems
                ),+
            }
        );
    };
}

#[cfg(test)]
mod test {
    use super::type_enum;
    use std::marker::PhantomData;

    type_enum! {
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

    impl Action<Complete> {}

    #[test]
    fn test_type_enum() {
        let ready_action = Action::<Ready>::new();
        let working_action = ready_action.start_work();
        working_action.complete_work();
    }
}
