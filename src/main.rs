mod presentation;
mod infrastructure;

fn main() -> std::io::Result<()> {
    // presentation::run()
    presentation::server::run()
}
