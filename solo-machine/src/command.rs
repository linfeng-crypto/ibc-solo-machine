mod chain;
mod ibc;

use std::{
    convert::TryFrom,
    fmt::Display,
    io::{stdout, Write},
    net::SocketAddr,
    path::PathBuf,
};

use anyhow::{ensure, Context, Result};
use cli_table::{Cell, Row, RowStruct, Style};
use solo_machine_core::{connect_db, event::HandlerRegistrar as _, init_db, run_migrations};
use structopt::{clap::Shell, StructOpt};
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{
    event::{cli_event_handler::CliEventHandler, env_logger::EnvLogger, HandlerRegistrar},
    server::start_grpc,
    signer::SignerRegistrar,
};

use self::{chain::ChainCommand, ibc::IbcCommand};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "solo-machine-cli",
    about = "A command line interface for IBC solo machine"
)]
pub struct Command {
    /// Does not print styled/colored statements
    #[structopt(long)]
    no_style: bool,
    /// Database connection string
    #[cfg_attr(
        not(feature = "postgres"),
        doc = "[example: `sqlite://solo-machine.db`]"
    )]
    #[cfg_attr(
        feature = "postgres",
        doc = "[example: `postgresql://postgres:postgres@localhost:5432/solo_machine`]"
    )]
    #[structopt(long, env = "SOLO_DB_URI", hide_env_values = true)]
    db_uri: Option<String>,
    /// Register a signer (path to signer's `*.so` file)
    #[structopt(long, env = "SOLO_SIGNER", hide_env_values = true)]
    signer: Option<PathBuf>,
    /// Register an event handler. Multiple event handlers can be registered and they're executed in order they're
    /// provided in CLI. Also, if an event handler returns an error when handling a message, all the future event
    /// handlers will not get executed.
    #[structopt(long)]
    handler: Vec<PathBuf>,
    #[structopt(subcommand)]
    subcommand: SubCommand,
}

#[derive(Debug, StructOpt)]
#[allow(clippy::large_enum_variant)]
pub enum SubCommand {
    /// Chain operations (managing chain state and metadata)
    Chain(ChainSubCommand),
    /// Generate completion scripts for solo-machine-cli
    GenCompletion {
        #[structopt(long, default_value = "bash")]
        shell: Shell,
    },
    /// Used to connect, mint tokens and burn tokens on IBC enabled chain
    Ibc(IbcSubCommand),
    /// Initializes database for solo machine
    Init,
    /// Starts gRPC server for solo machine
    Start {
        /// gRPC server address
        #[structopt(short, long, env = "SOLO_GRPC_ADDR", default_value = "0.0.0.0:9000")]
        addr: SocketAddr,
    },
}

#[derive(Debug, StructOpt)]
pub struct ChainSubCommand {
    #[structopt(subcommand)]
    subcommand: ChainCommand,
}

#[derive(Debug, StructOpt)]
pub struct IbcSubCommand {
    #[structopt(subcommand)]
    subcommand: IbcCommand,
}

impl Command {
    pub async fn execute(self) -> Result<()> {
        let color_choice = if self.no_style {
            ColorChoice::Never
        } else {
            ColorChoice::Auto
        };

        match self.subcommand {
            SubCommand::Chain(chain) => {
                ensure!(
                    self.signer.is_some(),
                    "`signer` is required for chain commands"
                );
                ensure!(self.db_uri.is_some(), "`db-uri` is required");

                let db_pool = connect_db(&self.db_uri.unwrap()).await?;

                let mut handler_registrar = HandlerRegistrar::try_from(self.handler)?;
                handler_registrar.register(Box::new(CliEventHandler::new(color_choice)));
                let (sender, handle) = handler_registrar.spawn();

                let signer = SignerRegistrar::try_from(self.signer.unwrap())?.unwrap()?;

                chain
                    .subcommand
                    .execute(db_pool, signer, sender, color_choice)
                    .await?;

                handle
                    .await
                    .context("unable to join event hook registrar task")?
            }
            SubCommand::GenCompletion { shell } => {
                Self::clap().gen_completions_to("solo-machine", shell, &mut stdout());
                Ok(())
            }
            SubCommand::Ibc(ibc) => {
                ensure!(
                    self.signer.is_some(),
                    "`signer` is required for ibc commands"
                );
                ensure!(self.db_uri.is_some(), "`db-uri` is required");

                let db_pool = connect_db(&self.db_uri.unwrap()).await?;

                let mut handler_registrar = HandlerRegistrar::try_from(self.handler)?;
                handler_registrar.register(Box::new(CliEventHandler::new(color_choice)));
                let (sender, handle) = handler_registrar.spawn();

                let signer = SignerRegistrar::try_from(self.signer.unwrap())?.unwrap()?;

                ibc.subcommand
                    .execute(db_pool, signer, sender, color_choice)
                    .await?;

                handle
                    .await
                    .context("unable to join event hook registrar task")?
            }
            SubCommand::Init => {
                ensure!(self.db_uri.is_some(), "`db-uri` is required");

                let db_uri = self.db_uri.unwrap();
                init_db(&db_uri).await?;

                let db_pool = connect_db(&db_uri).await?;

                run_migrations(&db_pool).await?;

                let mut stdout = StandardStream::stdout(color_choice);
                print_stream(
                    &mut stdout,
                    ColorSpec::new().set_bold(true),
                    "Initialized solo machine!",
                )
            }
            SubCommand::Start { addr } => {
                ensure!(
                    self.signer.is_some(),
                    "`signer` is required for gRPC server"
                );
                ensure!(self.db_uri.is_some(), "`db-uri` is required");

                let db_pool = connect_db(&self.db_uri.unwrap()).await?;
                let mut handler_registrar = HandlerRegistrar::try_from(self.handler)?;
                handler_registrar.register(Box::new(EnvLogger::new()));
                let (sender, handle) = handler_registrar.spawn();

                let signer = SignerRegistrar::try_from(self.signer.unwrap())?.unwrap()?;

                start_grpc(db_pool, signer, sender, addr).await?;

                handle
                    .await
                    .context("unable to join event hook registrar task")?
            }
        }
    }
}

fn add_row(table: &mut Vec<RowStruct>, title: &str, value: impl Display) {
    table.push(vec![title.cell().bold(true), value.cell()].row());
}

fn print_stream(
    stdout: &mut StandardStream,
    color_spec: &ColorSpec,
    s: impl Display,
) -> Result<()> {
    stdout.set_color(color_spec)?;
    writeln!(stdout, "{}", s).context("unable to write to stdout")?;
    stdout.reset().context("unable to reset stdout")?;

    Ok(())
}
