mod simple_rm;

use std::env;

use simple_rm::SimpleRemove;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rm = SimpleRemove::new(args);
    rm.execute();
}
