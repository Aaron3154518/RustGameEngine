use std::collections::HashMap;

use crate::enums::{Enum, Eq, New, Stringify};
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

enum_union!(Master, MyMessageEnum, OtherMessageEnum);

// Message bus
trait SubTrait {
    fn get_id(&self) -> u128;
    fn call<'a>(&'a self, v: &'a Master) -> Option<Box<dyn Fn() + '_>>;
}

struct Subscription<T> {
    id: u128,
    cb: Box<dyn Fn(&T)>,
}

impl<T> SubTrait for Subscription<T> {
    fn get_id(&self) -> u128 {
        self.id
    }

    default fn call<'a>(&'a self, _v: &'a Master) -> Option<Box<dyn Fn() + '_>> {
        None
    }
}

impl SubTrait for Subscription<MyMessageEnum> {
    fn call<'a>(&'a self, v: &'a Master) -> Option<Box<dyn Fn() + '_>> {
        match v {
            Master::MyMessageEnum(x) => Some(Box::new(move || (self.cb)(x))),
            _ => None,
        }
    }
}

impl SubTrait for Subscription<OtherMessageEnum> {
    fn call<'a>(&'a self, v: &'a Master) -> Option<Box<dyn Fn() + '_>> {
        match v {
            Master::OtherMessageEnum(x) => Some(Box::new(move || (self.cb)(x))),
            _ => None,
        }
    }
}

struct SubscriptionHandle {
    id: u128,
    name: &'static str,
}

struct MessageBus {
    subs: HashMap<&'static str, Vec<Box<dyn SubTrait>>>,
}

impl MessageBus {
    fn subscribe<U, T: MessageTrait<U>>(&mut self, cb: &'static dyn Fn(&U)) -> SubscriptionHandle {
        let id: u128 = self.subs.len() as u128;
        let sub = Subscription {
            id: id,
            cb: Box::new(cb),
        };
        match self.subs.get_mut(T::NAME) {
            Some(v) => v.push(Box::new(sub)),
            None => {
                self.subs.insert(T::NAME, Vec::new());
                match self.subs.get_mut(T::NAME) {
                    Some(v) => v.push(Box::new(sub)),
                    None => {
                        println!(
                            "MessageBus::subscribe() - Failed to create new subscription vector for {}", T::NAME
                        )
                    }
                }
            }
        }
        SubscriptionHandle { id, name: T::NAME }
    }

    fn unsubscribe(&mut self, handle: SubscriptionHandle) {
        match self.subs.get_mut(handle.name) {
            Some(v) => match v.iter().position(|sub| sub.get_id() == handle.id) {
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

    fn send_message<U, T: MessageTrait<U>>(&self, msg: T)
    where
        Master: New<U>,
    {
        match self.subs.get(T::NAME) {
            Some(v) => {
                for sub in v {
                    match sub.call(&Master::new(msg.get_code())) {
                        Some(f) => (f)(),
                        None => println!(
                            "MessageBus::send_message() - Could not call callback got {}",
                            T::NAME,
                        ),
                    };
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
    let sub = mb.subscribe::<MyMessageEnum, MyMessage>(&|v: &MyMessageEnum| {
        println!("Hello From Callback {}", v)
    });
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
