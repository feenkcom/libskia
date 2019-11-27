use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::gpu::{Context, SurfaceOrigin, BackendRenderTarget};
use skia_safe::{Budgeted, ImageInfo, Surface, ColorType};

#[no_mangle]
pub fn skia_surface_from_render_target(
    _backend_render_target_ptr: *mut ValueBox<BackendRenderTarget>,
    _context_ptr: *mut ValueBox<Context>,
    _color_type: ColorType,
) -> *mut ValueBox<Surface> {
    _backend_render_target_ptr.with(|backend_render_target| {
        _context_ptr.with_not_null_return(std::ptr::null_mut(), |context| {
            let surface_option = Surface::from_backend_render_target(
            context,
            backend_render_target,
            SurfaceOrigin::BottomLeft,
            _color_type,
            None,
            None
        );
        match surface_option {
            None => {
                if cfg!(debug_assertions) {
                        eprintln!("[skia_surface_from_render_target] Unable to create Skia Surface width: {:?} height: {:?} color type: {:?}", backend_render_target.width(), backend_render_target.height(), _color_type);
                }
                std::ptr::null_mut() },
            Some(surface) => {
                ValueBox::new(surface).into_raw()
            },
        }
        })
    })
}

#[no_mangle]
pub fn skia_surface_new_render_target(
    _image_info_ptr: *mut ValueBox<ImageInfo>,
    _context_ptr: *mut ValueBox<Context>,
) -> *mut ValueBox<Surface> {
    _image_info_ptr.with(|image_info| {
        _context_ptr.with_not_null_return(std::ptr::null_mut(), |context| {
            let surface_option = Surface::new_render_target(
            context,
            Budgeted::NO,
            &image_info,
            8,
            SurfaceOrigin::BottomLeft,
            None,
            true,
        );
        match surface_option {
            None => {
                if cfg!(debug_assertions) {
                        eprintln!("[skia_surface_new_render_target] Unable to create Skia Surface width: {:?} height: {:?} color type: {:?}", image_info.width(), image_info.height(), image_info.color_type());
                }
                std::ptr::null_mut() },
            Some(surface) => {
                ValueBox::new(surface).into_raw()
            },
        }
        })

    })
}