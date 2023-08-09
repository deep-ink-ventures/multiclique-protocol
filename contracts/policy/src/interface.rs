use soroban_sdk::{Env, auth::Context};

pub trait MultiCliquePolicy {
    fn get_threshold(env: Env, ctx: Context) -> u32;

    fn run_policy(env: Env, ctx: Context);
}