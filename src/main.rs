fn main() {
    env_logger::init();

    let new_chain = vaporeon::chain::Chain::new();
    new_chain.start();
}
