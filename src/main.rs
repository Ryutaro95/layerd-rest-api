mod presentation;
mod infrastructure;

use presentation as server;

fn main() -> std::io::Result<()> {
    server::run()
}
