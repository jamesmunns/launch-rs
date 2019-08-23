use crate::launchpad::mk2::LaunchpadMk2;

pub mod mk2;

use crate::{nearest_palette, RGBColor};

pub type Result<T> = std::result::Result<T, portmidi::Error>;

pub trait Launchpad {
	/// Light all LEDs to the same color.
	fn light_all(&self, color: u8) -> Result<()>;

	/// Light a single LED to a color.
	fn light_single(&self, raw_position: u8, raw_color: u8) -> Result<()>;

	/// Set a single LED to flash a color.
	fn flash_single(&self, raw_position: u8, raw_color: u8) -> Result<()>;

	/// Set a sinle LED to pulse a color.
	fn pulse_single(&self, raw_position: u8, raw_color: u8) -> Result<()>;

	/// Light a row of LEDs to the same color, including the side buttons.
	fn light_row(&self, y: u8, raw_color: u8) -> Result<()>;

	/// Light a column of LEDs to the same color, including the side buttons.
	fn light_column(&self, x: u8, raw_color: u8) -> Result<()>;

	/// Begin scrolling a message. The screen will be blanked, and the letters
	/// will be the same color. If the message is set to loop, it can be cancelled
	/// by sending an empty `scroll_text` command. String should only contain ASCII
	/// characters, or the byte value of 1-7 to set the speed (`\u{01}` to `\u{07}`)
	fn scroll_text(&self, do_loop: bool, text: &str, raw_color: u8) -> Result<()>;
}