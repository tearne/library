use battleships::keyboard::grab_all_keyboards;

#[tokio::main]
async fn main() {
    let mut rx = grab_all_keyboards();

    while let Some(t) = rx.recv().await {
        println!("--> {:?}", t);
    }
}
