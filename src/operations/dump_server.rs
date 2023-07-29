use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::fs;
use std::hash::{Hash, Hasher};

use dotenv_parser::parse_dotenv;
use rand::{Rng, SeedableRng};

use crate::{direnv, topic, verbose, Command, Context, OrFail};

pub fn main(port: Option<u16>) -> anyhow::Result<String> {
    let context = Context::get()?;
    context.platform.move_cwd();

    let mut envs: BTreeMap<String, String> = read_env_local()?;

    topic!("Generating dumper url...");
    let port = port.unwrap_or_else(|| 9912_u16 + seed_rand(&context.platform.path_hash));
    envs.insert(
        "VAR_DUMPER_FORMAT".into(),
        format!("tcp://127.0.0.1:{port}"),
    );
    println!("Dumper url: 'tcp://127.0.0.1:{port}'");

    topic!("Writing VAR_DUMPER_FORMAT to .env.local...");
    write_env_local(&envs)?;

    topic!("Starting VarDumper server...");
    // TODO - Ctr+C doesn't work, main app is terminated
    direnv![
        "./vendor/bin/var-dump-server",
        &format!("--host=127.0.0.1:{port}")
    ]
    .await_success()?;

    topic!("Removing VAR_DUMPER_FORMAT from .env.local...");
    write_env_local(&read_env_local().unwrap_or_default())?;

    Ok("VarDumper server stopped and envs cleaned up".into())
}

fn read_env_local() -> anyhow::Result<BTreeMap<String, String>> {
    let Ok(env_local) = fs::read_to_string(".env.local") else {
        verbose!("No .env.local was found");
        return Ok(BTreeMap::new());
    };

    verbose!("Found .env.local to read");
    let mut envs = parse_dotenv(&env_local).map_err(|e| anyhow::anyhow!(e))?;

    envs.remove("VAR_DUMPER_FORMAT");

    Ok(envs)
}

fn write_env_local(envs: &BTreeMap<String, String>) -> anyhow::Result<()> {
    let file = envs
        .iter()
        .fold(String::new(), |acc, (k, v)| format!("{acc}{k}={v}\n"));

    fs::write(".env.local", file).or_error("Failed to write .env.local")?;

    Ok(())
}

fn seed_rand(seed: &str) -> u16 {
    println!("Generating random number from path hash...");
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let seed_hash = hasher.finish();

    rand::rngs::StdRng::seed_from_u64(seed_hash).gen_range(0..=20)
}
