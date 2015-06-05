//! Timer to periodically execute an action.
use iup_sys;

use Element;
use Guard;

/// A timer which periodically invokes a callback when the time is up.
///
/// # Ownership
///
/// The timer must be manually destroyed, thus for the user safety it returns a guarded object
/// on the `new` constructor.
///
/// Please refer to the crate level documentation of IUP-Rust (the main doc page) for details on
/// ownership of elements.
pub struct Timer(*mut iup_sys::Ihandle);

impl Timer {
    /// Constructs a timer.
    pub fn new() -> Guard<Timer> {
        Guard::new(
            Timer::from_raw(unsafe { iup_sys::IupTimer() })
        )
    }

    /// Gets the set time interval in milliseconds or `None` if not set.
    pub fn time(&self) -> Option<u32> {
        self.attrib_parse("TIME")
    }

    /// Sets the time interval in milliseconds.
    ///
    /// In Windows the minimum value is 10ms.
    pub fn set_time(&mut self, time: u32) -> Self {
        self.set_attrib("TIME", time.to_string())
    }

    /// Starts the timer.
    ///
    /// Does nothing if the TIME attribute is not set i.e. `set_time`.
    ///
    /// If you have multiple threads start the timer in the main thread.
    pub fn run(&mut self) -> Self {
        self.set_attrib("RUN", "YES")
    }

    /// Stops the timer.
    pub fn stop(&mut self) -> Self {
        self.set_attrib("RUN", "NO")
    }

    /// Returns the current timer state.
    pub fn is_running(&self) -> bool {
        self.attrib_bool("RUN").unwrap()
    }
}

impl_element!(Timer, "timer");

/// Called every time the defined time interval is reached.
///
/// To stop the callback from being called simply stop de timer with RUN=NO or `Timer::stop`.
///
/// `CallbackReturn::Close` will be processed.
impl ::callback::ActionCb for Timer {}
