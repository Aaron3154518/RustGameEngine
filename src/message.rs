use crate::enum_type;

/// Are `T` and `U` are the same type?
pub fn eq<T, U>(t: &T, u: &U) -> bool {
    // Helper trait. `VALUE` is false, except for the specialization of the
    // case where `T == U`.
    trait TypeEq<U> {
        fn eq(t: &Self, u: &U) -> bool;
    }

    // Default implementation.
    impl<T, U> TypeEq<U> for T {
        default fn eq(_t: &T, _u: &U) -> bool {
            false
        }
    }

    // Specialization for `T == U`.
    impl<T: std::cmp::PartialEq> TypeEq<T> for T {
        fn eq(t: &T, u: &T) -> bool {
            t == u
        }
    }

    <T as TypeEq<U>>::eq(t, u)
}

struct Message<En: std::cmp::PartialEq> {
    code: En,
}

impl<En: std::cmp::PartialEq, En2> std::cmp::PartialEq<En2> for Message<En> {
    default fn eq(&self, _other: &En2) -> bool {
        false
    }
}

impl<En2, En: std::cmp::PartialEq + std::cmp::PartialEq<En2>> std::cmp::PartialEq<En2>
    for Message<En>
{
    fn eq(&self, other: &En2) -> bool {
        self.code == *other
    }
}

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

trait Eq {
    fn equ<T>(&self, t: T) -> bool
    where
        Self: Sized,
    {
        eq::<Self, T>(self, &t)
    }
}

#[derive(Debug, PartialEq)]
enum A {
    Y,
    Z,
}
impl Eq for A {}
// impl<T: std::cmp::PartialEq<A> + std::fmt::Debug> std::cmp::PartialEq<T> for A {
//     default fn eq(&self, other: &T) -> bool {
//         println!("{:?} {:?}", *self, *other);
//         match *self {
//             A::Y => *other == A::Y,
//             A::Z => *other == A::Z,
//         }
//     }
// }
#[derive(Debug)]
enum B {
    T,
    U,
}
// impl<T> std::cmp::PartialEq<T> for B {
//     default fn eq(&self, _other: &T) -> bool {
//         false
//     }
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
    println!("{} {} {}", A::Y.equ(A::Y), A::Y.equ(A::Z), A::Y.equ(B::T));
    let mut container: Container<u8, TestSignal> = Container { slots: vec![] };
    let sig: TestSignal = TestSignal {};
    let slot: TestSlot = TestSlot {};
    container.add_slot(slot);
    container.send_signal(&sig);
}
