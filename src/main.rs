mod enums;
mod message;

// enum_type!(Hi, A, B, C);
// enum_type!(A, Y, Z);
// enum_type!(B, T, U);

// enum_union!(AB, A, B, Hi);

fn main() {
    // let a: AB = AB::A(A::Z);
    // let b: AB = AB::Hi(Hi::A);
    // println!("{} {} {}", a, (if a == b { "==" } else { "!=" }), b);
    message::test();
}
