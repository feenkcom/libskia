use shared_library_builder::{GitLocation, LibraryLocation, LibraryTarget, RustLibrary};

pub fn libskia(version: impl Into<String>) -> RustLibrary {
    let target = LibraryTarget::for_current_platform();

    let mut library = RustLibrary::new(
        "Skia",
        LibraryLocation::Git(GitLocation::github("feenkcom", "libskia").tag(version)),
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
    };

    library
}
