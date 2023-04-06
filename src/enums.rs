#[macro_export]
macro_rules! enum_type {
    ($n:ident, $($e:tt), *) => {
        #[derive(Debug)]
        enum $n {
            $($e),*
        }

        impl<T> std::cmp::PartialEq<T> for $n {
            fn eq(&self, other: &T) -> bool {
                false
            }
        }
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
