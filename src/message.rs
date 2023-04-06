use std::collections::HashMap;
use std::marker::PhantomData;

use crate::enum_union;
use crate::enums;
use crate::enums::New;
use crate::test::{MyMessageEnum, OtherMessageEnum};

// Message extensions
pub trait Constructor<T> {
    fn new(t: T) -> Self;
}

pub trait MessageTrait<T> {
    const NAME: &'static str = "";

    fn get_code(&self) -> T;
}

// Subscriptions
pub trait SubTrait {
    fn get_id(&self) -> u128;
    fn call<'a>(&'a self, v: &'a Master) -> Option<Box<dyn Fn() + '_>>;
}

pub struct Subscription<T> {
    id: u128,
    pub(crate) cb: Box<dyn Fn(&T)>,
}

impl<T> SubTrait for Subscription<T> {
    fn get_id(&self) -> u128 {
        self.id
    }

    default fn call<'a>(&'a self, _v: &'a Master) -> Option<Box<dyn Fn() + '_>> {
        None
    }
}

// Messages
#[macro_export]
macro_rules! message {
    ($n: ident, $n1: ident, $($e: ident),+) => {
        enum_union!($n1, $($e),*);

        pub struct $n {
            code: $n1,
        }

        $(impl message::Constructor<$e> for $n {
            fn new(e: $e) -> $n {
                $n { code: $n1::$e(e) }
            }
        })*

        impl message::MessageTrait<$n1> for $n {
            const NAME: &'static str = stringify!($n);

            fn get_code(&self) -> $n1 {
                self.code
            }
        }

        impl message::SubTrait for message::Subscription<$n1> {
            fn call<'a>(&'a self, v: &'a message::Master) -> Option<Box<dyn Fn() + '_>> {
                match v {
                    message::Master::$n1(x) => Some(Box::new(move || (self.cb)(x))),
                    _ => None,
                }
            }
        }
    };
}

enum_union!(Master, MyMessageEnum, OtherMessageEnum);

// Subscription traits
pub trait SubscriptionHandleTrait {
    type U;
    type T: MessageTrait<Self::U>;

    fn get_id(&self) -> u128;
    fn get_name(&self) -> &'static str {
        Self::T::NAME
    }
}

pub struct SubscriptionHandle<U, T: MessageTrait<U>> {
    u: PhantomData<U>,
    t: PhantomData<T>,

    id: u128,
}

impl<U, T: MessageTrait<U>> SubscriptionHandle<U, T> {
    pub fn new(id: u128) -> SubscriptionHandle<U, T> {
        SubscriptionHandle {
            u: PhantomData,
            t: PhantomData,
            id: id,
        }
    }
}

impl<U, T: MessageTrait<U>> SubscriptionHandleTrait for SubscriptionHandle<U, T> {
    type U = U;
    type T = T;

    fn get_id(&self) -> u128 {
        self.id
    }
}

// MessageBus
pub struct MessageBus {
    subs: HashMap<&'static str, Vec<Box<dyn SubTrait>>>,
}

impl MessageBus {
    pub fn new() -> Self {
        MessageBus {
            subs: HashMap::new(),
        }
    }

    pub fn subscribe<U, T: MessageTrait<U>>(
        &mut self,
        cb: &'static dyn Fn(&U),
    ) -> SubscriptionHandle<U, T> {
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
        SubscriptionHandle::new(id)
    }

    pub fn unsubscribe(&mut self, handle: impl SubscriptionHandleTrait) {
        match self.subs.get_mut(handle.get_name()) {
            Some(v) => match v.iter().position(|sub| sub.get_id() == handle.get_id()) {
                Some(i) => {
                    v.remove(i);
                }
                None => println!(
                    "MessageBus::unsubscribe() - No subscription with id {}",
                    handle.get_id()
                ),
            },
            None => println!(
                "MessageBus::unsubscribe() - No subscription of type {}",
                handle.get_name()
            ),
        }
    }

    pub fn send_message<U, T: MessageTrait<U>>(&self, msg: T)
    where
        Master: enums::New<U>,
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
