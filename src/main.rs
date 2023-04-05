#[macro_export]
macro_rules! enum_type {
    ($n:ident, $($e:tt), *) => {
        #[derive(Debug, PartialEq)]
        enum $n {
            $($e),*
        }
    };
}

#[macro_export]
macro_rules! enum_union {
    ($n: ident, $($e:ident),*) => {
        #[derive(PartialEq)]
        enum $n {
            $($e { v : $e }),*
        }

        impl std::fmt::Display for $n {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $($n::$e { v } => write!(f, "{:?}", v)),*
                }
            }
        }
    };
}

enum_type!(Hi, A, B, C);
enum_type!(A, Y, Z);
enum_type!(B, T, U);

enum_union!(AB, A, B, Hi);

fn main() {
    let a: AB = AB::A { v: A::Z };
    let b: AB = AB::Hi { v: Hi::A };
    println!("{} {} {}", a, (if a == b { "==" } else { "!=" }), b);
}
