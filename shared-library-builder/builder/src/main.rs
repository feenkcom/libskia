use std::error::Error;

use shared_library_builder::build_standalone;

use libskia_library::latest_libskia;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|target| Ok(Box::new(latest_libskia(target))))
}
