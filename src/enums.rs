trait TypeEq<T> {
    fn eq(s: &Self, t: &T) -> bool;
}

impl<S, T> TypeEq<T> for S {
    default fn eq(_s: &S, _t: &T) -> bool {
        false
    }
}

impl<S: std::cmp::PartialEq> TypeEq<S> for S {
    fn eq(s: &S, t: &S) -> bool {
        s == t
    }
}

pub trait Eq {
    fn equals<T>(&self, t: &T) -> bool
    where
        Self: Sized,
    {
        <Self as TypeEq<T>>::eq(self, t)
    }
}

#[macro_export]
macro_rules! enum_type {
    ($n:ident, $($e:tt), *) => {
        #[derive(Debug, PartialEq)]
        enum $n {
            $($e),*
        }

        impl Eq for $n {}
    };
}

#[macro_export]
macro_rules! enum_union {
    ($n: ident, $($e:ident),*) => {
        #[derive(PartialEq)]
        enum $n {
            $($e($e)),*
        }

        $(impl std::cmp::PartialEq<$e> for $n {
            fn eq(&self, other: &$e) -> bool {
                match self {
                    $n::$e(v) => v.equals(other),
                    _ => false
                }
            }
        })*

        impl Eq for $n {
            fn equals<T>(&self, t: &T) -> bool
            where
                Self: Sized,
            {
                match self {
                    $($n::$e(v) => v.equals(t)),*
                }
            }
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
