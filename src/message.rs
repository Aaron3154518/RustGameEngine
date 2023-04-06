use crate::enums::{Enum, Eq, Stringify};
use crate::{enum_type, enum_union};

// Enums
enum_type!(A, Y, Z);
enum_type!(B, S, T);
enum_union!(AB, A, B);

struct Message<T: Eq> {
    code: T,
}

impl<T: Enum, U> std::cmp::PartialEq<U> for Message<T> {
    default fn eq(&self, other: &U) -> bool {
        self.code.equals(other)
    }
}

trait Constructor<T> {
    fn new(t: T) -> Self;
}

trait MessageTrait {
    const NAME: &'static str = "";
}

#[macro_export]
macro_rules! message {
    ($n: ident, $n1: ident, $($e: ident),+) => {
        enum_union!($n1, $($e),*);

        struct $n {
            msg: Message<$n1>,
        }

        $(impl Constructor<$e> for $n {
            fn new(e: $e) -> $n {
                $n {
                    msg: Message {
                        code: $n1::$e(e)
                    }
                }
            }
        })*

        impl MessageTrait for $n {
            const NAME: &'static str = stringify!($n);
        }
    };
}

message!(MyMessage, MyMessageEnum, A, B);

// trait SubTrait {
//     fn get_name(&self) -> &'static str {
//         ""
//     }
// }

// struct Subscription<En> {
//     code: En,
//     name: &'static str,
// }

// impl<En: std::cmp::PartialEq<En2>, En2> std::cmp::PartialEq<En2> for Subscription<En> {
//     fn eq(&self, other: &En2) -> bool {
//         self.code == *other
//     }
// }

// impl<En> Subscription<En> {
//     fn handle_message<En2: std::cmp::PartialEq + std::cmp::PartialEq<En>>(
//         &self,
//         msg: &Message<En2>,
//     ) {
//         if (msg.code == self.code) {
//             println!("{}", self.name)
//         }
//     }
// }

// fn eq<T, U>(s: &Subscription<T>, m: &Message<T>) -> bool {
//     false
// }

// fn eq<T, T>(s: &Subscription<T>, m: &Message<T>) -> bool {
//     true
// }

// enum SubTypes {
//     SubA(Subscription<A>),
//     SubB(Subscription<B>),
// }

// impl SubTrait for SubTypes {
//     fn get_name(&self) -> &'static str {
//         match self {
//             SubTypes::SubA(a) => a.name,
//             SubTypes::SubB(b) => b.name,
//         }
//     }
// }

// // impl<En> std::cmp::PartialEq<Message<En>> for SubTypes {
// //     fn eq(&self, other: &Message<En>) -> bool {
// //         match self {
// //             SubTypes::SubA(a) => matches!(&other.code, a),
// //             SubTypes::SubB(b) => matches!(&other.code, b),
// //         }
// //     }
// // }

// struct MessageBus {
//     subs: Vec<SubTypes>,
// }

// impl MessageBus {
//     // fn send_msg<En>(&self, msg: Message<En>) {
//     //     for sub in &self.subs {
//     //         if *sub == msg {
//     //             println!("{}", sub.get_name())
//     //         }
//     //     }
//     // }

//     // fn subscribe<En>(&self, name: &'static str, code: En) {
//     //     self.subs.push(SubTypes::SubA(Subscription {
//     //         code: code,
//     //         name: name,
//     //     }));
//     // }
// }

// pub fn test() {
//     println!("Test");
//     let msgA: Message<A> = Message { code: A::Y };
//     let subA: Subscription<A> = Subscription {
//         code: A::Y,
//         name: "A::Y",
//     };
//     let subB: Subscription<B> = Subscription {
//         code: B::T,
//         name: "B::T",
//     };
//     let subA2: Subscription<A> = Subscription {
//         code: A::Z,
//         name: "A::Z",
//     };
//     print!("{}", A::Z == B::T);
//     subA.handle_message(&msgA);
//     subA2.handle_message(&msgA);
//     subB.handle_message(&msgA);

//     // let msg_bus: MessageBus = MessageBus { subs: vec![] };
//     // msg_bus.subscribe("A::Y", A::Y);
// }

// Signal/Slot
trait Signal<T> {
    fn val(&self) -> T;
}

trait Slot<T, U: Signal<T>> {
    fn receive_val(&self, val: T);
}

struct Container<T, U> {
    slots: Vec<Box<dyn Slot<T, U>>>,
}

impl<T, U: Signal<T>> Container<T, U> {
    fn add_slot<V: Slot<T, U> + 'static>(&mut self, slot: V) {
        self.slots.push(Box::new(slot));
    }

    fn send_signal<V: Signal<T>>(&self, sig: &V) {
        for slot in &self.slots {
            (*slot).receive_val(sig.val());
        }
    }
}

struct TestSignal {}
impl Signal<u8> for TestSignal {
    fn val(&self) -> u8 {
        10
    }
}

struct TestSlot {}
impl Slot<u8, TestSignal> for TestSlot {
    fn receive_val(&self, val: u8) {
        println!("TestSlot: {}", val)
    }
}

pub fn test() {
    println!("Test");

    println!(
        "{} {} {}",
        A::Y.equals(&A::Y),
        A::Y.equals(&A::Z),
        A::Y.equals(&B::T)
    );

    let msg = Message { code: AB::A(A::Y) };
    println!("{} {} {}", msg == A::Y, msg == A::Z, msg == B::T);

    let ab = AB::A(A::Y);
    println!("{} {} {}", ab == A::Y, ab == A::Z, ab == B::T);
    println!(
        "{} {} {}",
        ab.equals(&A::Y),
        ab.equals(&A::Z),
        ab.equals(&B::T)
    );

    println!(
        "{} {} {} {}",
        A::Y.to_str(),
        A::Z.to_str(),
        B::T.to_str(),
        ab.to_str()
    );

    let msg = MyMessage {
        msg: Message {
            code: MyMessageEnum::A(A::Y),
        },
    };
    let msg2 = MyMessage::new(B::S);
    println!("{} {}", msg.msg.code, msg2.msg.code);

    let mut container: Container<u8, TestSignal> = Container { slots: vec![] };
    let sig: TestSignal = TestSignal {};
    let slot: TestSlot = TestSlot {};
    container.add_slot(slot);
    container.send_signal(&sig);
}
