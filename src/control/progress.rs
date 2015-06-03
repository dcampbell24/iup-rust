use iup_sys;

use Element;

/// See the [IUP Progress Bar Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/elem/iupprogressbar.html
pub struct ProgressBar(*mut iup_sys::Ihandle);

impl ProgressBar {
    /// Creates a progress bar control.
    pub fn new() -> ProgressBar {
        unsafe { ProgressBar::from_raw(iup_sys::IupProgressBar()) } 
    }
}

impl_widget!(ProgressBar, "progressbar");
impl ::callback::MapCb for ProgressBar {}
impl ::callback::UnmapCb for ProgressBar {}
