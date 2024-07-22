use std::sync::Arc;

use libtest_mimic::{Arguments, Conclusion, Failed};
use tracing::info;

use golem_test_framework::config::{
    EnvBasedTestDependencies, EnvBasedTestDependenciesConfig, TestDependencies,
};

mod api_definition;
mod api_deployment;
pub mod cli;
mod component;
mod profile;
mod text;
mod worker;

fn run(deps: Arc<dyn TestDependencies + Send + Sync + 'static>) -> Conclusion {
    let args = Arguments::from_args();

    let mut tests = Vec::new();

    tests.append(&mut component::all(deps.clone()));
    tests.append(&mut worker::all(deps.clone()));
    tests.append(&mut text::all(deps.clone()));
    tests.append(&mut api_definition::all(deps.clone()));
    tests.append(&mut api_deployment::all(deps.clone()));
    tests.append(&mut profile::all(deps));

    libtest_mimic::run(&args, tests)
}

fn main() -> Result<(), Failed> {
    env_logger::init();

    let deps: Arc<dyn TestDependencies + Send + Sync + 'static> = Arc::new(
        EnvBasedTestDependencies::blocking_new_from_config(EnvBasedTestDependenciesConfig {
            worker_executor_cluster_size: 3,
            keep_docker_containers: true, // will be dropped by testcontainers (current version does not support double rm)
            ..EnvBasedTestDependenciesConfig::new()
        }),
    );
    let cluster = deps.worker_executor_cluster(); // forcing startup by getting it
    info!("Using cluster with {:?} worker executors", cluster.size());

    let res = run(deps.clone());

    drop(cluster);
    drop(deps);

    res.exit()
}
