use std::any::Any;

fn main() {
    let (tx, rx) = tokio::sync::broadcast::channel::<u8>(10);

    println!("tx: {:?}, rx: {:?}", tx.type_id(), rx.type_id());
    let boxed_tx = Box::new(tx.clone());
    println!("boxed_tx: {:?}", boxed_tx.type_id());
    let dyn_boxed_tx: Box<dyn Any> = Box::new(tx.clone());
    println!("dyn_boxed_tx: {:?}", dyn_boxed_tx.type_id());

    if let Ok(downcast_tx) = dyn_boxed_tx.downcast::<tokio::sync::broadcast::Sender<u8>>() {
        println!("downcast_tx: {:?}", downcast_tx.type_id());
    } else {
        println!("Nope");
    }
}
