use iup_sys;
use libc::{c_char, c_int};
use std::ptr;

use Element;
use callback::IntoRust;

/// See the [IUP List Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iuplist.html
pub struct List(*mut iup_sys::Ihandle);

impl List {
    /// Creates an interface element that displays a list of items.
    pub fn new() -> List {
        unsafe { List::from_raw(iup_sys::IupList(ptr::null())) }
    }

    /// Creates an interface element that displays a list of items in a dropdown.
    pub fn new_dropdown() -> List {
        List::new().set_attrib_data("DROPDOWN", cstr!("YES") as *const _)
    }

    /// Creates an interface element that displays a list of items with a edit box for text input.
    pub fn new_editbox() -> List {
        List::new().set_attrib_data("EDITBOX", cstr!("YES") as *const _)
    }

    // TODO how to call the fusion of dropbox and editbox?

    /// Sets the list of items.
    pub fn set_items<A>(&mut self, items: A) -> Self where A: AsRef<[String]> {
        self.clear();
        for (i, value) in items.as_ref().iter().enumerate() {
            self.set_attrib((i+1).to_string(), value.clone());
        }
        *self
    }

    /// Gets the item at the specified id (starts from 1).
    ///
    /// # Panics
    /// Panics if id is less than 1.
    pub fn item<A>(&self, id: u32) -> Option<String> {
        assert!(id > 0);
        self.attrib(id.to_string())
    }

    /// Clears the list of items. Ignored if called before being mapped.
    pub fn clear(&mut self) -> Self {
        self.set_attrib("REMOVEITEM", "ALL")
    }
}

impl_widget!(List, "list");

/// Returns a list item position from it's xy coordinate.
impl ::element::ConvertXYToPos for List {}

impl ::callback::MapCb for List {}
impl ::callback::UnmapCb for List {}
impl ::callback::GetFocusCb for List {}
impl ::callback::KillFocusCb for List {}
impl ::callback::EnterWindowCb for List {}
impl ::callback::LeaveWindowCb for List {}
impl ::callback::HelpCb for List {}
// TODO impl K_ callbacks when it's implemented.

// TODO impl future DragSource and DragTarget traits.

/// Action generated when any mouse button is pressed or released inside the list.
///
/// Called only when DROPDOWN=NO. If the list has an editbox the message is called when cursor
/// is at the listbox only (ignored at the editbox).
///
/// Use `convert_xy_to_pos` to convert (x,y) coordinates in item position.
impl ::callback::button::ButtonCb for List {}

/// Action generated when the caret/cursor position is changed. Valid only when EDITBOX=YES.
///
/// For lists `lin` (2nd param) is always *1*, and pos (3rd param) is always *col-1*.
impl ::callback::CaretCb for List {}

/// Action generated when one or more files are dropped in the element.
impl ::callback::DropFilesCb for List {}

///  Action generated when the mouse is moved over the list. Called only when DROPDOWN=NO.
///
/// If the list has an editbox the message is called when cursor is at the listbox only (ignored
/// at the editbox).
///
/// Use `convert_xy_to_pos` to convert (x,y) coordinates in item position.
impl ::callback::button::MotionCb for List {}

/// Called after the value was interactively changed by the user. Called when the selection is
/// changed or when the text is edited.
impl ::callback::ValueChangedCb for List {}

// TODO:
// DBLCLICK_CB
// MULTISELECT_CB
// EDIT_CB
// DROPDOWN_CB
// DRAGDROP_CB

/// See the `ListAction` documentation.
impl self::ListAction for List {}
impl_callback! {
    #[doc="Action generated when the state of an item in the list is changed."]
    #[doc="Also provides information on the changed item."]
    #[doc=""]
    #[doc="The `String` parameter is the text of the changed item."]
    #[doc="The `u32` parameter is the number of the changed item starting at 1."]
    #[doc="The `ListItemState` parameter is whether the item was selected or deselected. "]
    #[doc=""]
    #[doc="The  `ListItemState::Deselected` is simulated internally by IUP in all systems."]
    #[doc="If you add or remove items to/from the list and you count on the `ListItemState::Deselected`"]
    #[doc="value, then after adding/removing items set the VALUE attribute to ensure proper"]
    #[doc="`ListItemState::Deslected` value."]
    pub trait ListAction where Self: Element {
        let name = "ACTION";
        extern fn listener(ih: *mut iup_sys::Ihandle, text: *const c_char, item: c_int, state: c_int) -> CallbackReturn;
        fn set_action<F: Callback(Self, String, u32, ListItemState)>(&mut self, cb: F) -> Self;
        fn remove_action(&mut self) -> Option<Box<_>>;
    }
}

pub enum ListItemState {
    Deselected,
    Selected,
}

impl IntoRust<ListItemState> for c_int {
    fn into_rust(self) -> ListItemState {
        if self != 0 { ListItemState::Selected } else { ListItemState::Deselected }
    }
}
