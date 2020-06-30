

use {
    tide::Redirect,
};

#[async_std::main]
async fn main() -> tide::Result<()> {

    let mut app = tide::with_state(());

    app.at("/").get(|_| async { Ok("Hello, world!") });

    app.listen("localhost:8001").await?;

    Ok(())
}
