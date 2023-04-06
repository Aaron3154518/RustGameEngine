use crate::enums;
use crate::{enum_type, enum_union, message};

enum_type!(A, Y, Z);
enum_type!(B, S, T);
enum_union!(AB, A, B);

message!(MyMessage, MyMessageEnum, A, B);
message!(OtherMessage, OtherMessageEnum, A);
