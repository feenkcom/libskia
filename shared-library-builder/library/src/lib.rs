use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libskia(version: impl Into<String>) -> RustLibrary {
    RustLibrary::new(
        "Skia",
        LibraryLocation::Git(GitLocation::github("feenkcom", "libskia").tag(version)),
    )
    .package("libskia")
    .requires("python")
}
