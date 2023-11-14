// dependencies from packages
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt; //compat::TokioAsyncWriteCompatExt;

// dependencies from other files
use axum_practice::functions::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config =
        Config::from_ado_string("server=localhost;Database=SampleDatabase;Trusted_Connection=True");

    let tcp = TcpStream::connect(config.as_ref().unwrap().get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config.unwrap(), tcp.compat_write()).await?;

    run().await;

    Ok(())
}
