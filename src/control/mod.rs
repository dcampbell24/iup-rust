//! See [IUP Controls][1].
//! [1]: http://webserver2.tecgraf.puc-rio.br/iup/en/controls.html

pub mod label;
pub mod text;
pub mod button;
pub mod progress;
pub mod toggle;
pub mod frame;

pub use self::text::{Text, TextAction};
pub use self::label::Label;
pub use self::button::Button;
pub use self::progress::ProgressBar;
pub use self::toggle::{Toggle, ToggleAction};
pub use self::frame::Frame;
