#[macro_export]
macro_rules! enum_type {
    ($n:ident, $($e:tt), *) => {
        #[derive(Debug, PartialEq)]
        enum $n {
            $($e),*
        }

        impl<T> std::cmp::PartialEq<T> for $n {
            default fn eq(&self, other: &T) -> bool {
                false
            }
        }

        // impl<En: std::cmp::PartialEq, En2> std::cmp::PartialEq<En2> for Message<En> {
        //     default fn eq(&self, _other: &En2) -> bool {
        //         false
        //     }
        // }

        // impl<En2, En: std::cmp::PartialEq + std::cmp::PartialEq<En2>> std::cmp::PartialEq<En2>
        //     for Message<En>
        // {
        //     fn eq(&self, other: &En2) -> bool {
        //         self.code == *other
        //     }
        // }

    };
}

#[macro_export]
macro_rules! enum_union {
    ($n: ident, $($e:ident),*) => {
        #[derive(PartialEq)]
        enum $n {
            $($e($e)),*
        }

        impl std::fmt::Display for $n {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $($n::$e(v) => write!(f, "{:?}", v)),*
                }
            }
        }
    };
}
