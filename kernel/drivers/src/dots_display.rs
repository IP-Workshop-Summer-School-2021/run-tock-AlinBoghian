use kernel::hil::led;

pub const DRIVER_NUM:usize= 0xa0001;

pub struct DotsDisplay<'a>{
	leds: &a [&'a dyn Led;25]	
}
