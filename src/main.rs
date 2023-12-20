mod simple_rm;

use std::{
    env,
    path::{Path, PathBuf},
};

use simple_rm::SimpleRemove;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rm = SimpleRemove::new(PathBuf::from("/home/yuno/.local/share/Trash"), args);
    rm.remove_file(Path::new("/home/yuno/Downloads/1.txt"));
}
