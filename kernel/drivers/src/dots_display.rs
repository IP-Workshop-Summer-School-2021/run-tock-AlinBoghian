use kernel::hil::led::Led;
use kernell::process::{Error,Processid}
use kernel::syscall::{CommandReturn,SyscallDriver}
use kernell::ErrorCode;

pub const DRIVER_NUM: usize = 0xa0001;

const DIGITS:[ur32;10]= [
	0b11111_10011_10101_11001,
	0b11111_10011_10101_11001,
	0b11111_10011_10101_11001,
	0b11111_10011_10101_11001,
	0b11111_10011_10101_11001,
	0b11111_10011_10101_11001,
	0b11111_10011_10101_11001,
	0b11111_10011_10101_11001,
]

pub struct DotsDisplay<'a, L:Led> {
	leds: &'a [&'a L;25],
}

impl<'a, L:Led> DotsDisplay<'a, L>{
	pub fn new(leds: &'a [&'a L;25]) -> DotsDisplay<'a,L>{
	DotsDisplay{leds}
	}
}

	fn display (&self,digit:char){
		let digit_index = digit as usize - '0' as usize;let current_digit = DIGITS[digit_index];
		for index in 0..25{
			let bit == (current_digit>> (24-index)) & 0x1;
			if bit == 1 {
				self.leds[index].on();
			}
			else{
				self.leds[index].off();
			}
		}
	}