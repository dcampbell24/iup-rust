//! Image elements to be embedded in other controls.
use iup_sys;
use Element;

#[derive(Debug, Copy, Clone)]
pub struct InPixels<'a, T: 'static>(pub &'a [&'a [T]]);

/// This macro is meant to represent expressively pixels of a image.
///
/// It's output should go in the `ImageElement::with` constructor.
/// The output can be also stored in a variable and passed to several image elements.
///
/// # Example
/// ```ignore
/// ImageRgb::with(pixels![
///     [(255, 0, 0), (255, 0, 0), (255, 0, 0)],
///     [(0, 255, 0), (0, 255, 0), (0, 255, 0)],
///     [(0, 0, 255), (0, 0, 255), (0, 0, 255)],
///     [(0, 0, 255), (0, 0, 255), (0, 0, 255)],
///     [(128, 128, 128), (0, 0, 0), (0, 88, 99)],
/// ])
/// ```
#[macro_export]
macro_rules! pixels {
    ( $($row:expr),+, ) => { pixels![ $($row),+ ] };
    ( $($row:expr),* ) => { $crate::image::InPixels(&[ $(&$row),* ]) };
}

pub trait ImageElement : Element {
    /// The type of a single pixel of this image.
    type Pixel : 'static + Copy;

    /// Creates a new image element with the specified dimensions from the specified pixel array.
    ///
    /// # Panics
    /// Should panic if `width`x`height` does not match the length of `pixels`.
    fn new<U>(width: u32, height: u32, pixels: U) -> Self where U: AsRef<[Self::Pixel]>;

    /// Creates a new image element from the output of the `pixels!` macro.
    ///
    /// # Panics
    /// Panics if not all the rows of `pixels` have the same length.
    fn with<'a>(pixels: InPixels<'a, Self::Pixel>) -> Self {
        let pixels = pixels.0;
        let height = pixels.len();
        let width  = pixels.first().map(|row| row.len()).unwrap_or(0);

        let mut v = Vec::with_capacity(height * width);

        for row in pixels.iter() {
            assert_eq!(width, row.len());
            for &pixel in row.iter() {
                v.push(pixel);
            }
        }

        Self::new(width as u32, height as u32, v)
    }
}


/// A one-channel image which each pixel is a byte index to a pallet of RGB colors.
/// 
/// The image is meant to be shown on a label, button, toggle, or as a cursor.
///
/// See the [IUP Image Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupimage.html
pub struct Image(*mut iup_sys::Ihandle);

impl Image {
    /// Sets the pallet of colors to be used by the pixels.
    ///
    /// This is a shortcut to the `0`, `1`, `2`, ..., `i` attributes.
    ///
    /// # Panics
    /// Panics if the length of the colors pallet is greater than 256.
    pub fn set_colors<U>(&mut self, colors: U) -> Image where U: AsRef<[(u8, u8, u8)]> {
        assert!(colors.as_ref().len() < 256);
    	for (i, &color) in colors.as_ref().iter().enumerate() {
    		self.set_attrib_rgb(i.to_string(), color);
    	}
    	self.clone()
    }
}

impl ImageElement for Image {
    type Pixel = u8;
    fn new<U>(width: u32, height: u32, pixels: U) -> Image where U: AsRef<[u8]> {
        unsafe {
            let pixels = pixels.as_ref();
            assert_eq!((width * height) as usize, pixels.len());
            Image::from_raw(iup_sys::IupImage(width as i32, height as i32, pixels.as_ptr()))
        }
    }
}

impl_element!(Image, "image");


/// A three-channel image to be shown on a label, button, toggle, or as a cursor.
///
/// See the [IUP Image Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupimage.html
pub struct ImageRgb(*mut iup_sys::Ihandle);

impl ImageElement for ImageRgb {
    type Pixel = (u8, u8, u8);
    fn new<U>(width: u32, height: u32, pixels: U) -> ImageRgb where U: AsRef<[(u8, u8, u8)]> {
        unsafe {
            let pixels = pixels.as_ref();
            assert_eq!((width * height) as usize, pixels.len());
            ImageRgb::from_raw(iup_sys::IupImageRGB(width as i32, height as i32,
                                                    pixels.as_ptr() as *const _))
        }
    }
}

impl_element!(ImageRgb, "imagergb");


/// A four-channel image to be shown on a label, button, toggle, or as a cursor.
///
/// See the [IUP Image Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupimage.html
pub struct ImageRgba(*mut iup_sys::Ihandle);

impl ImageElement for ImageRgba {
    type Pixel = (u8, u8, u8, u8);
    fn new<U>(width: u32, height: u32, pixels: U) -> ImageRgba where U: AsRef<[(u8, u8, u8, u8)]> {
        unsafe {
            let pixels = pixels.as_ref();
            assert_eq!((width * height) as usize, pixels.len());
            ImageRgba::from_raw(iup_sys::IupImageRGBA(width as i32, height as i32,
                                                     pixels.as_ptr() as *const _))
        }
    }
}

impl_element!(ImageRgba, "imagergba");
