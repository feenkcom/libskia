use shared_library_builder::{GitLocation, LibraryLocation, LibraryTarget, RustLibrary};

pub fn libskia(target: LibraryTarget, version: Option<impl Into<String>>) -> RustLibrary {
    let mut location = GitLocation::github("feenkcom", "libskia");
    if let Some(version) = version {
        location = location.tag(version);
    }

    let mut library = RustLibrary::new("Skia", LibraryLocation::Git(location))
        .package("libskia")
        .env(
            "SKIA_GN_ARGS",
            "extra_cflags+=[\"-DSK_AVOID_SLOW_RASTER_PIPELINE_BLURS\"]",
        )
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

    if target.is_android() {
        library = library.feature("skia_android");
    };

    library
}

pub fn latest_libskia(target: LibraryTarget) -> RustLibrary {
    let version: Option<String> = None;
    libskia(target, version)
}
