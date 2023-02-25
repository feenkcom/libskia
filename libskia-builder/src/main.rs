use std::error::Error;

use libskia_builder::latest_libskia;
use shared_library_builder::build_standalone;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|target| Ok(Box::new(latest_libskia(target))))
}
