mod mk2;
pub use mk2::*;

pub trait Launchpad<E> {
	/// Light all LEDs to the same color.
	fn light_all(&mut self, color: u8) -> Result<()>;

	/// Light a single LED to a color.
	fn light_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()>;

	/// Set a single LED to flash a color.
	fn flash_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()>;

	/// Set a sinle LED to pulse a color.
	fn pulse_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()>;

	/// Light a row of LEDs to the same color, including the side buttons.
	fn light_row(&mut self, y: u8, raw_color: u8) -> Result<()>;

	/// Light a column of LEDs to the same color, including the side buttons.
	fn light_column(&mut self, x: u8, raw_color: u8) -> Result<()>;

	fn light_single_rgb(&mut self, raw_position: u8, red: u8, green: u8, blue: u8) -> Result<()>;

	/// Begin scrolling a message. The screen will be blanked, and the letters
	/// will be the same color. If the message is set to loop, it can be cancelled
	/// by sending an empty `scroll_text` command. String should only contain ASCII
	/// characters, or the byte value of 1-7 to set the speed (`\u{01}` to `\u{07}`)
	fn scroll_text(&mut self, do_loop: bool, text: &str, raw_color: u8) -> Result<()>;

	/// Poll the device for MIDI events.
	fn poll(&mut self) -> Vec<E>;
}
