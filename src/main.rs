#![feature(specialization)]
#![feature(trait_alias)]

mod enums;
mod message;
mod test;

use enums::*;
use message::*;
use test::*;

fn do_test() {
    println!("Test");

    println!(
        "{} {} {}",
        A::Y.equals(&A::Y),
        A::Y.equals(&A::Z),
        A::Y.equals(&B::T)
    );

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

    let msg = MyMessage::new(A::Y);
    let msg2 = MyMessage::new(B::S);
    println!("{} {}", msg.get_code(), msg2.get_code());

    let mut mb = message::MessageBus::new();
    let sub = mb.subscribe::<MyMessageEnum, MyMessage>(&|v: &MyMessageEnum| {
        println!("Hello From Callback {}", v)
    });
    println!("Sub Id: {}", sub.get_id());
    mb.send_message(msg);
    mb.send_message(OtherMessage::new(A::Z));
    mb.unsubscribe(message::SubscriptionHandle::<MyMessageEnum, MyMessage>::new(2));
    mb.unsubscribe(sub);
    mb.send_message(msg2);
    println!("Shouldn't print");
}

fn main() {
    do_test();
}
