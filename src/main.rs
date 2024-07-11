#![feature(async_closure, try_blocks, async_fn_traits)]

use portal_appearance::AppearanceConnection;

pub mod color_scheme;
pub mod daemon;
pub mod portal_appearance;

#[tokio::main]
async fn main() {
    let conn = AppearanceConnection::connect().await;
    let pref = conn.get_preference().await.unwrap();
    println!("{:?}", pref);
    conn.listen(async |pref| println!("{:?}", pref)).await;
}
