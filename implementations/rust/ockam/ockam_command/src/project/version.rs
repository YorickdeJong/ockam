use clap::Args;
use colorful::Colorful;
use miette::IntoDiagnostic;

use ockam::Context;
use ockam_api::nodes::InMemoryNode;

use crate::util::api::CloudOpts;
use crate::util::async_cmd;
use crate::{docs, fmt_ok, CommandGlobalOpts};

const LONG_ABOUT: &str = include_str!("./static/version/long_about.txt");
const AFTER_LONG_HELP: &str = include_str!("./static/version/after_long_help.txt");

/// Return the version of the Orchestrator Controller and the projects
#[derive(Clone, Debug, Args)]
#[command(
long_about = docs::about(LONG_ABOUT),
after_long_help = docs::about(AFTER_LONG_HELP)
)]
pub struct VersionCommand {
    #[command(flatten)]
    pub cloud_opts: CloudOpts,
}

impl VersionCommand {
    pub fn run(self, opts: CommandGlobalOpts) -> miette::Result<()> {
        async_cmd(&self.name(), opts.clone(), |ctx| async move {
            self.async_run(&ctx, opts).await
        })
    }

    pub fn name(&self) -> String {
        "get version".into()
    }

    async fn async_run(&self, ctx: &Context, opts: CommandGlobalOpts) -> miette::Result<()> {
        // Send request
        let node = InMemoryNode::start(ctx, &opts.state).await?;
        let controller = node.create_controller().await?;
        let project_version = controller.get_orchestrator_version_info(ctx).await?;

        let json = serde_json::to_string(&project_version).into_diagnostic()?;
        let project_version = project_version
            .project_version
            .unwrap_or("unknown".to_string());
        let plain = fmt_ok!("The version of the Projects is '{project_version}'");

        opts.terminal
            .stdout()
            .plain(plain)
            .machine(project_version)
            .json(json)
            .write_line()?;
        Ok(())
    }
}
