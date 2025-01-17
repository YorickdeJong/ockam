use clap::Args;

use ockam::Context;
use ockam_abac::{Action, Expr, Policy, Resource};
use ockam_api::nodes::BackgroundNodeClient;
use ockam_core::api::Request;

use crate::node::util::initialize_default_node;
use crate::policy::policy_path;
use crate::util::async_cmd;
use crate::CommandGlobalOpts;

#[derive(Clone, Debug, Args)]
pub struct CreateCommand {
    #[arg(long, display_order = 900, id = "NODE_NAME")]
    pub at: Option<String>,

    #[arg(short, long)]
    pub resource: Resource,

    #[arg(short, long, default_value = "handle_message")]
    pub action: Action,

    #[arg(short, long)]
    pub expression: Expr,
}

impl CreateCommand {
    pub fn run(self, opts: CommandGlobalOpts) -> miette::Result<()> {
        async_cmd(&self.name(), opts.clone(), |ctx| async move {
            self.async_run(&ctx, opts).await
        })
    }

    pub fn name(&self) -> String {
        "create policy".into()
    }

    pub async fn async_run(&self, ctx: &Context, opts: CommandGlobalOpts) -> miette::Result<()> {
        initialize_default_node(ctx, &opts).await?;
        let node = BackgroundNodeClient::create(ctx, &opts.state, &self.at).await?;
        let bdy = Policy::new(self.expression.clone());
        let req = Request::post(policy_path(&self.resource, &self.action)).body(bdy);
        node.tell(ctx, req).await?;
        Ok(())
    }
}
