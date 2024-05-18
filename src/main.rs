use jdeps::search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    search::run()
}
