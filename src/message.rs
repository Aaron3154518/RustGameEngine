use std::collections::HashMap;

use crate::enums::{Enum, Eq, Stringify};
use crate::{enum_type, enum_union};

// Enums
enum_type!(A, Y, Z);
enum_type!(B, S, T);
enum_union!(AB, A, B);

// Messages
struct Message<T: Eq> {
    code: T,
}

impl<T: Enum, U> std::cmp::PartialEq<U> for Message<T> {
    default fn eq(&self, other: &U) -> bool {
        self.code.equals(other)
    }
}

// Message extensions
trait Constructor<T> {
    fn new(t: T) -> Self;
}

trait MessageTrait<T> {
    const NAME: &'static str = "";

    fn get_code(&self) -> T;
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

        impl MessageTrait<$n1> for $n {
            const NAME: &'static str = stringify!($n);

            fn get_code(&self) -> $n1 {
                self.msg.code
            }
        }
    };
}

message!(MyMessage, MyMessageEnum, A, B);
message!(OtherMessage, OtherMessageEnum, A);

// Message bus
// trait SubTrait {
//     fn call<T>(&self, t: T);
// }

struct Subscription {
    id: u128,
    cb: Box<dyn Fn()>,
}

// impl<T> SubTrait for Subscription<T> {
//     fn call(&self, t: T) {
//         (self.cb)(t);
//     }
// }

struct SubscriptionHandle {
    id: u128,
    name: &'static str,
}

struct MessageBus {
    subs: HashMap<&'static str, Vec<Subscription>>,
}

impl MessageBus {
    fn subscribe<U, T: MessageTrait<U>>(&mut self, cb: &'static dyn Fn()) -> SubscriptionHandle {
        let id: u128 = self.subs.len() as u128;
        let sub = Subscription {
            id: id,
            cb: Box::new(cb),
        };
        match self.subs.get_mut(T::NAME) {
            Some(v) => v.push(sub),
            None => {
                self.subs.insert(T::NAME, Vec::new());
                match self.subs.get_mut(T::NAME) {
                    Some(v) => v.push(sub),
                    None => {
                        println!(
                            "MessageBus::subscribe() - Failed to create new subscription vector for {}", T::NAME
                        )
                    }
                }
            }
        }
        SubscriptionHandle {
            id: id,
            name: T::NAME,
        }
    }

    fn unsubscribe(&mut self, handle: SubscriptionHandle) {
        match self.subs.get_mut(handle.name) {
            Some(v) => match v.iter().position(|sub| sub.id == handle.id) {
                Some(i) => {
                    v.remove(i);
                }
                None => println!(
                    "MessageBus::unsubscribe() - No subscription with id {}",
                    handle.id
                ),
            },
            None => println!(
                "MessageBus::unsubscribe() - No subscription of type {}",
                handle.name
            ),
        }
    }

    fn send_message<U, T: MessageTrait<U>>(&self, _msg: T) {
        match self.subs.get(T::NAME) {
            Some(v) => {
                for sub in v {
                    (sub.cb)();
                }
            }
            None => println!(
                "MessageBus::send_message() - No subscriptions for {}",
                T::NAME
            ),
        }
    }
}

// Test
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

    let mut mb = MessageBus {
        subs: HashMap::new(),
    };
    let sub = mb.subscribe::<MyMessageEnum, MyMessage>(&|| println!("Hello From Callback"));
    println!("Sub Id: {}", sub.id);
    mb.send_message(msg);
    mb.send_message(OtherMessage::new(A::Z));
    mb.unsubscribe(SubscriptionHandle {
        id: 2,
        name: "MyMessage",
    });
    mb.unsubscribe(sub);
    mb.send_message(msg2);
    println!("Shouldn't print");
}
