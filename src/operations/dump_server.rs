use std::collections::BTreeMap;
use std::fs;

use dotenv_parser::parse_dotenv;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::{Rng, SeedableRng};

use crate::{fail, direnv, Context, ExitCode, AppCommand, success};

pub fn main(port: Option<u16>) {
    Context::get().platform.move_to();

    let mut envs: BTreeMap<String, String> = read_env_local().unwrap_or_default();

    let port = port.unwrap_or_else(|| 9912_u16 + seed_rand(&Context::get().platform.path_hash));
    envs.insert("VAR_DUMPER_FORMAT".to_owned(), format!("tcp://127.0.0.1:{port}"));

    write_env_local(&envs);

    log::info!("VarDumper server starting on port {port}");
    // TODO - Ctr+C doesn't work, main app is terminated
    direnv!["./vendor/bin/var-dump-server", &format!("--host=127.0.0.1:{port}")].start_await_success();

    log::info!("Remove VAR_DUMPER_FORMAT from .env.local");
    write_env_local(&read_env_local().unwrap_or_default());

    success!("VarDumper server stopped and envs cleaned up");
}

fn read_env_local() -> Option<BTreeMap<String, String>> {
    log::debug!("Read .env.local");
    if let Ok(env_local) = fs::read_to_string(".env.local") {
        log::info!("Found .env.local to read");
        let mut envs = parse_dotenv(&env_local).unwrap_or_else(|_| {
            fail!(ExitCode::Runtime, "Failed to parse .env.local");
        });

        envs.remove("VAR_DUMPER_FORMAT");

        return Some(envs);
    }

    log::info!("There is no .env.local to read");
    None
}

fn write_env_local(envs: &BTreeMap<String, String>) {
    let file = envs.iter().fold(String::new(), |acc, (k, v)| {
        format!("{acc}{k}={v}\n")
    });

    log::info!("Writing .env.local");
    fs::write(".env.local", file).unwrap_or_else(|_| {
        fail!(ExitCode::Runtime, "Failed to write .env.local");
    });
}

fn seed_rand(seed: &str) -> u16 {
    log::info!("Generate random number from path hash");
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let seed_hash = hasher.finish();

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed_hash);
    rng.gen_range(0..=20)
}