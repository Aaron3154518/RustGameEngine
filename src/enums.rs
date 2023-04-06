trait TypeEq<T> {
    fn eq(s: &Self, t: &T) -> bool;
    const TYPE_EQ: bool;
}

impl<S, T> TypeEq<T> for S {
    default fn eq(_s: &S, _t: &T) -> bool {
        false
    }
    default const TYPE_EQ: bool = false;
}

impl<S: std::cmp::PartialEq> TypeEq<S> for S {
    fn eq(s: &S, t: &S) -> bool {
        s == t
    }
    const TYPE_EQ: bool = true;
}

pub trait Eq {
    fn equals<T>(&self, t: &T) -> bool
    where
        Self: Sized,
    {
        <Self as TypeEq<T>>::eq(self, t)
    }
}

pub trait Stringify {
    fn to_str(&self) -> &str;
}

pub trait New<T> {
    fn new(t: T) -> Self;
}

pub trait Enum = Eq + Stringify;

#[macro_export]
macro_rules! enum_type {
    ($n:ident, $($e:tt),+) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum $n {
            $($e),*
        }

        impl enums::Eq for $n {}

        impl enums::Stringify for $n {
            fn to_str(&self) -> &str {
                stringify!($n)
            }
        }
    };
}

#[macro_export]
macro_rules! enum_union {
    ($n: ident, $($e: ident),+) => {
        #[derive(PartialEq, Clone, Copy, Debug)]
        pub enum $n {
            $($e($e)),*
        }

        $(impl enums::New<$e> for $n {
            fn new(t: $e) -> Self {
                $n::$e(t)
            }
        })*

        $(impl std::cmp::PartialEq<$e> for $n {
            fn eq(&self, other: &$e) -> bool {
                match self {
                    $n::$e(v) => enums::Eq::equals(v, other),
                    _ => false
                }
            }
        })*

        impl enums::Eq for $n {
            fn equals<T>(&self, t: &T) -> bool
            where
                Self: Sized,
            {
                match self {
                    $($n::$e(v) => v.equals(t)),*
                }
            }
        }

        impl enums::Stringify for $n {
            fn to_str(&self) -> &str {
                match self {
                    $($n::$e(v) => v.to_str()),*
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
