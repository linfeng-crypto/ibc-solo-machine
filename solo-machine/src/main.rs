use anyhow::Result;
use solo_machine::command::Command;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    Command::from_args().execute().await
}
