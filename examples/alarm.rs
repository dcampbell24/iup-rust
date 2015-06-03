//! Example based on http://sourceforge.net/p/iup/iup/HEAD/tree/trunk/iup/html/examples/Lua/alarm.lua

#[macro_use]
extern crate iup;
use iup::dialog::AlarmButton;

fn main () {
    iup::with_iup(|| {
        match iup::dialog::alarm("Alarm Example", "File not saved! Save it now?", "Yes".into(),
                                 Some("No".into()), Some("Cancel".into())) {
            AlarmButton::Button1 => iup::dialog::message("Save file", "File saved successfully - leaving program"),
            AlarmButton::Button2 => iup::dialog::message("Save file", "File not saved - leaving program anyway"),
            AlarmButton::Button3 => iup::dialog::message("Save file", "Operation canceled"),
        }
        Err("don't let the main loop run or we'll get frozen because we didn't get any dialog!".into())
    }).unwrap();
}
