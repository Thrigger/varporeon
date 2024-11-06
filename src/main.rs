fn main() {
    env_logger::init();

    let new_chain = vaporeon::chain::NodeRoot::new();
    new_chain.start();
}
