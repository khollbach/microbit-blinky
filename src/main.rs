#![allow(unused_imports)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    display::{self, blocking},
    hal::{prelude::*, Timer},
    Board,
};
use panic_rtt_target as _;
use rtt_target::{rdbg, rprintln, rtt_init_print};
use void::ResultVoidExt;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();

    let mut row1 = board.display_pins.row1;
    let mut col1 = board.display_pins.col1;

    let mut timer = Timer::periodic(board.TIMER0);
    timer.start(1_000_000u32);

    col1.set_low().void_unwrap();

    loop {
        row1.set_high().void_unwrap();
        rprintln!("Light!");
        nb::block!(timer.wait()).void_unwrap();

        row1.set_low().void_unwrap();
        rprintln!("Dark!");
        nb::block!(timer.wait()).void_unwrap();
    }
}
