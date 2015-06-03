use iup_sys;
use std::ptr;
use std::ffi::CString;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AlarmButton {
	Button1,
	Button2,
	Button3,
}

/// Shows a modal dialog containing a message and up to three buttons. 
///
/// Returns the pressed button.
///
/// See the [IUP Alarm Documentation][1].
/// [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/dlg/iupalarm.html
pub fn alarm<S1, S2>(title: S1, message: S2, button1: String,
	     	 	     button2: Option<String>, button3: Option<String>) -> AlarmButton
										       	where S1: Into<String>, S2: Into<String> {

	let ctitle = CString::new(title.into()).unwrap();
	let cmessage = CString::new(message.into()).unwrap();
	let cbutton1 = CString::new(button1).unwrap();
	let cbutton2 = button2.map(|s| CString::new(s).unwrap());
	let cbutton3 = button3.map(|s| CString::new(s).unwrap());

	let pressed = unsafe {
		iup_sys::IupAlarm(ctitle.as_ptr(), cmessage.as_ptr(),
			              cbutton1.as_ptr(),
						  cbutton2.map_or(ptr::null(), |cs| cs.as_ptr()),
						  cbutton3.map_or(ptr::null(), |cs| cs.as_ptr()))
	};

	match pressed {
		1 => AlarmButton::Button1,
		2 => AlarmButton::Button2,
		3 => AlarmButton::Button3,
		_ => unreachable!(),
	}
}
