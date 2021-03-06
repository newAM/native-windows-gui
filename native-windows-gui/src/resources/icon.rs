use winapi::um::winnt::HANDLE;
use winapi::um::winuser::IMAGE_ICON;
use crate::win32::resources_helper as rh;
use crate::{OemImage, OemIcon, NwgError};
use std::ptr;

#[cfg(feature = "embed-resource")]
use super::EmbedResource;


/**
A wrapper over a icon file (*.ico)

Note that icon object are only used as display resources (ie: it's impossible to read pixels or resized it).
If those features are needed, see the `image-decoder` feature.

To display a icon in an application, see the `ImageFrame` control.

Example:

```rust
use native_windows_gui as nwg;

fn load_icon() -> nwg::Icon {
    let mut icon = nwg::Icon::default();

    nwg::Icon::builder()
        .source_file(Some("Hello.cur"))
        .strict(true)
        .build(&mut icon);

    icon
}

*/
#[allow(unused)]
pub struct Icon {
    pub handle: HANDLE,
    pub(crate) owned: bool
}

impl Icon {

    pub fn builder<'a>() -> IconBuilder<'a> {
        IconBuilder {
            source_text: None,
            source_bin: None,
            source_system: None,

            #[cfg(feature = "embed-resource")]
            source_embed: None,

            #[cfg(feature = "embed-resource")]
            source_embed_id: 0,

            #[cfg(feature = "embed-resource")]
            source_embed_str: None,

            size: None,
            strict: false
        }
    }

}

pub struct IconBuilder<'a> {
    source_text: Option<&'a str>,
    source_bin: Option<&'a [u8]>,
    source_system: Option<OemIcon>,

    #[cfg(feature = "embed-resource")]
    source_embed: Option<&'a EmbedResource>,

    #[cfg(feature = "embed-resource")]
    source_embed_id: usize,

    #[cfg(feature = "embed-resource")]
    source_embed_str: Option<&'a str>,

    size: Option<(u32, u32)>,
    strict: bool,
}

impl<'a> IconBuilder<'a> {

    pub fn source_file(mut self, t: Option<&'a str>) -> IconBuilder<'a> {
        self.source_text = t;
        self
    }

    pub fn source_bin(mut self, t: Option<&'a [u8]>) -> IconBuilder<'a> {
        self.source_bin = t;
        self
    }

    pub fn source_system(mut self, t: Option<OemIcon>) -> IconBuilder<'a> {
        self.source_system = t;
        self
    }

    #[cfg(feature = "embed-resource")]
    pub fn source_embed(mut self, em: Option<&'a EmbedResource>) -> IconBuilder<'a> {
        self.source_embed = em;
        self
    }

    #[cfg(feature = "embed-resource")]
    pub fn source_embed_id(mut self, id: usize) -> IconBuilder<'a> {
        self.source_embed_id = id;
        self
    }

    #[cfg(feature = "embed-resource")]
    pub fn source_embed_str(mut self, id: Option<&'a str>) -> IconBuilder<'a> {
        self.source_embed_str = id;
        self
    }

    pub fn size(mut self, s: Option<(u32, u32)>) -> IconBuilder<'a> {
        self.size = s;
        self
    }

    pub fn strict(mut self, s: bool) -> IconBuilder<'a> {
        self.strict = s;
        self
    }

    pub fn build(self, b: &mut Icon) -> Result<(), NwgError> {
        if let Some(src) = self.source_text {
            let handle = unsafe { rh::build_image(src, self.size, self.strict, IMAGE_ICON)? };
            *b = Icon { handle, owned: true };
        } else if let Some(src) = self.source_system {
            let handle = unsafe { rh::build_oem_image(OemImage::Icon(src), self.size)? };
            *b = Icon { handle, owned: true };
        } else {
            #[cfg(feature = "embed-resource")]
            fn build_embed(builder: IconBuilder) -> Result<Icon, NwgError> {
                match builder.source_embed {
                    Some(embed) => {
                        match builder.source_embed_str {
                            Some(src) => embed.icon_str(src)
                                .ok_or_else(|| NwgError::resource_create(format!("No icon in embed resource identified by {}", src))),
                            None => embed.icon(builder.source_embed_id)
                                .ok_or_else(|| NwgError::resource_create(format!("No icon in embed resource identified by {}", builder.source_embed_id)))
                        }
                    },
                    None => Err(NwgError::resource_create("No source provided for Icon"))
                }
            }

            #[cfg(not(feature = "embed-resource"))]
            fn build_embed(_builder: IconBuilder) -> Result<Icon, NwgError> {
                Err(NwgError::resource_create("No source provided for Icon"))
            }

            *b = build_embed(self)?;
        }
    
        Ok(())
    }

}


impl Default for Icon {

    fn default() -> Icon {
        Icon {
            handle: ptr::null_mut(),
            owned: false
        }
    }

}

impl PartialEq for Icon {

    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }

}

