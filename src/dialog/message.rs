use iup_sys;
use std::ffi::CString;

use Element;

/// Shows a modal dialog containing a message. It simply creates and popup a `MessageDlg`.
pub fn message<S1, S2>(title: S1, message: S2)
                where S1: Into<String>, S2: Into<String> {
    let ctitle = CString::new(title.into()).unwrap();
    let cmessage = CString::new(message.into()).unwrap();
   	unsafe { iup_sys::IupMessage(ctitle.as_ptr(), cmessage.as_ptr()) };
}

/// An predefined dialog for displaying a message.
///
/// This dialog can be shown with the `popup` method only.
///
/// See the [IUP MessageDlg Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/dlg/iupmessagedlg.html
pub struct MessageDlg(*mut iup_sys::Ihandle);

impl MessageDlg {
    pub fn new() -> MessageDlg {
        unsafe { MessageDlg::from_raw(iup_sys::IupMessageDlg()) }
    }
}

impl_dialog!(MessageDlg, "messagedlg");
impl ::callback::HelpCb for MessageDlg {}
