//! Access to the system clipboard.
//!
//! The clipboard is a software facility used for short-term data storage and/or data transfer
//! between documents or applications, via copy and paste operations.
//!
//! The clipboard is user-driven. A window should transfer data to or from the clipboard only in
//! response to a command from the user. A window must not use the clipboard to transfer data
//! without the user's knowledge. 
//!
//! # Clipboard Data Formats
//!
//! A window can place more than one object on the clipboard, each representing the same
//! information in a different clipboard format. Users need not be aware of the clipboard formats
//! used for an object on the clipboard.
//!
//! Many applications work with data that cannot be translated into a standard clipboard format
//! without loss of information. These applications can create their own clipboard formats.
//! A clipboard format that is defined by an application, is called a registered clipboard format.
//! For example, if a word-processing application copied formatted text to the clipboard using a
//! standard text format, the formatting information would be lost.
//! The solution would be to register a new clipboard format, such as Rich Text Format (RTF). 
//!
//! To register a new clipboard format, use the `Clipboard::add_format` function. This format can
//! then be used by `Clipboard::data` and `Clipboard::set_data` functions.
//!
//! ## Standard Formats
//!
//! IUP gives the user two platform-dependent formats, text and image. If any other is required
//! the user may need to use a custom data format.
//!
//!
//! ## Multiple Clipboard Formats
//!
//! A window can place more than one clipboard object on the clipboard, each representing the same
//! information in a different clipboard format. When placing information on the clipboard, the
//! window should provide data in as many formats as possible.
//!
//! It's a recommended that clipboard formats that contain the most information should be placed on
//! the clipboard first, followed by less descriptive formats. This is because OS-specific functi-
//! onalities usually iterate on the clipboard data in the same order it was placed and uses the
//! first it is actually able to handle.
//!
//! For example, suppose a user copies styled text from a word-processing document. The window
//! containing the document might first place data on the clipboard in a registered format, such
//! as RTF. Subsequently, the window would place data on the clipboard in a less descriptive
//! format, such as text.
//!
//! When the content of the clipboard is pasted into another window, the window retrieves data in
//! the most descriptive format it recognizes. If the window recognizes RTF, the corresponding data
//! is pasted into the document. Otherwise, the text data is pasted into the document and the
//! formatting information is lost.
//!
//! Explanation borrowed off [MSDN][MSDN].
//! [MSDN]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms649013%28v=vs.85%29.aspx
//!
//! ## Example
//!
//! ### Copying
//!
//! ```no_run
//! # use iup::clipboard::Clipboard;
//! // Sets the user clipboard to have a text in two possible formats, HTML (as a custom format
//! // using it's MIME type) and text. If the application that receives the paste supports 
//! // any of these it'll use it.
//! Clipboard::new()
//!               .clear()
//!               .add_format("text/html")
//!               .set_data("text/html", r"This is <b>my</b> <i>awesome</i> text")
//!               .set_text("This is my awesome text");
//! ```
//!
//! ### Pasting
//! 
//! ```no_run
//! # use iup::clipboard::Clipboard;
//! let mut clipboard = Clipboard::new();
//! if let Some(html) = clipboard.add_format("text/html").data("text/html") {
//!     // Use HTML pasted content.
//! } else if let Some(text) = clipboard.text() {
//!     // Use text pasted content.
//! }
//! ```
//!
use iup_sys;
use std::ptr;
use std::slice;

use Element;
use Guard;
use image::ImageElement;

/// An element that allows access to the clipboard.
///
/// You can use only one for the entire application because it does not store any data inside.
/// Or you can simply create and drop every time you need to copy or paste.
///
/// See the clipboard module documentation for more details on how the system clipboard works.
///
/// Other platform-dependent attributes can be found on the [IUP Clipboard Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupclipboard.html
///
/// # Ownership
///
/// The clipboard must be manually destroyed, thus for the user safety it returns a guarded object
/// on the `new` constructor.
///
/// Please refer to the crate level documentation of IUP-Rust (the main doc page) for details on
/// ownership of elements.
pub struct Clipboard(*mut iup_sys::Ihandle);

impl Clipboard {
    /// Creates a new clipboard operarator.
    pub fn new() -> Guard<Clipboard> {
        Guard::new(
            Clipboard::from_raw(unsafe { iup_sys::IupClipboard() })
        )
    }

    /// Clears any data on the clipboard.
    pub fn clear(&mut self) -> Self {
        self.set_attrib_data("FORMATDATA", ptr::null())
    }

    /// Register a custom format for clipboard data given its name.
    pub fn add_format<S: Into<String>>(&mut self, format: S) -> Self {
        self.set_attrib("ADDFORMAT", format)
    }

    /// Informs if there is a text available at the clipboard.
    pub fn has_text(&mut self) -> bool {
        self.attrib_bool("TEXTAVAILABLE").unwrap()
    }

    /// Copy text into the clipboard.
    pub fn set_text<S: Into<String>>(&mut self, text: S) -> Self {
        self.set_attrib("TEXT", text)
    }

    /// Paste text off the clipboard.
    pub fn text(&mut self) -> Option<String> {
        self.attrib("TEXT")
    }

    /// Informs if there is a image available at the clipboard.
    pub fn has_image(&mut self) -> bool {
        self.attrib_bool("IMAGEAVAILABLE").unwrap()
    }

    /// Copy text into the clipboard.
    pub fn set_image<I: ImageElement>(&mut self, image: &I) -> Self {
        self.set_attrib_handle("IMAGE", *image)
    }


    /// Informs if there is data of the specified format available at the clipboard.
    pub fn has_data<S: Into<String>>(&mut self, format: S) -> bool {
        self.set_attrib("FORMAT", format);
        self.attrib_bool("FORMATAVAILABLE").unwrap()
    }

    /// Copy data from the specified format into the clipboard.
    pub fn set_data<S, D>(&mut self, format: S, data: D) -> Self
                                where S: Into<String>, D: AsRef<[u8]> {
        let data = data.as_ref();
        self.set_attrib("FORMAT", format);
        self.set_attrib("FORMATDATASIZE", data.len().to_string());
        self.set_attrib_data("FORMATDATA", data.as_ptr() as *const _)
    }
 
    /// Paste data from the specified format off the clipboard.
    pub fn data<S: Into<String>>(&mut self, format: S) -> Option<Vec<u8>> {
        self.set_attrib("FORMAT", format);
        match self.attrib_data("FORMATDATA") as *const u8 {
            ptr if !ptr.is_null() => unsafe {
                let len: usize = self.attrib_parse("FORMATDATASIZE").unwrap();
                Some(slice::from_raw_parts(ptr, len).into_iter().cloned().collect())
            },
            _ => None,
        }
    }
}

impl_element!(Clipboard, "clipboard");


// TODO please someone test the HTML copying example of the module documentation! In my experience
// when I paste it in another application that supports HTML pasting it gives me some weirdo
// chinese characters, after some testing it seems to require a UTF-16 stream... A place that
// allows pasting HTML is this editor: http://versadus.com/flavius/editor/_samples/editor-html-online.html
