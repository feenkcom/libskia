use shared_library_builder::{
    Library, LibraryCompilationContext, LibraryLocation, LibraryTarget, PathLocation, RustLibrary,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let target = LibraryTarget::for_current_platform();

    let mut library = RustLibrary::new(
        "Skia",
        LibraryLocation::Path(PathLocation::new(std::env::current_dir().unwrap())),
    )
    .package("libskia")
    .requires("python");

    if target.is_windows() {
        library = library.feature("skia_windows");
    }

    if target.is_mac() {
        library = library.feature("skia_mac");
    }

    if target.is_linux() {
        library = library.feature("skia_linux");
    }

    let context = LibraryCompilationContext::new("target", "target", target, false);
    let compiled_library = library.compile(&context)?;
    println!("Compiled {}", compiled_library.display());
    Ok(())
}
