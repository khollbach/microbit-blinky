#![allow(unused_imports)]
#![no_std]
#![no_main]

use core::arch::asm;

use cortex_m_rt::entry;
use microbit::{
    display::{self, blocking},
    hal::{prelude::*, Timer},
    Board, pac::P0,
};
use panic_rtt_target as _;
use rtt_target::{rdbg, rprintln, rtt_init_print};
use void::ResultVoidExt;

#[entry]
fn main() -> ! {
    // asm!(" .... ");

    rtt_init_print!();

    let board = Board::take().unwrap();

    let mut row1 = board.display_pins.row1;
    let mut col1 = board.display_pins.col1;

    let mut timer = Timer::periodic(board.TIMER0);
    timer.start(1_000_000u32);

    // unsafe { &(*P0::PTR).outclr }.write(|w| w.pin28().set_bit());
    // col1.set_low().void_unwrap();

    unsafe {
        let base = 0x5000_0000;
        let offset = 0x050C;
        let mut outclr: *mut u32 = (base + offset) as *mut u32;
        *outclr = 1 << 28;
    }

    // 1. put (1<<28) in a register
    // 2. store that reg's value to mem-addr

    // 1.
    // 1.a. put 1 in a register (r0)
    // 1.b. shift by 28

    // 2.
    // 2.a. put ADDR in a register (r1)

    // // ON
    // col1.set_low().void_unwrap();
    // row1.set_high().void_unwrap();

    // // OFF
    // col1.set_high().void_unwrap();

    rprintln!("before");

    // let r0: u32 = 0;
    // let r1: u32 = 0;
    // let r2: u32 = 0;

    // " col1.set_low "
    unsafe {
        asm!(
            //???


            // 1.
            "mov {mask}, #1", // a.
            "lsl {mask}, #28", // b.  TODO : hex vs dec

            // 2.

            // 2. a.
            "mov {addr}, #0x5000",  // todo: single instr mov+shift ?
            "lsl {addr}, #16",

            "mov {offset}, #0x050C",
            "orr {addr}, {addr}, {offset}",  // todo: can we really not or with a literal ?

            "str {mask}, [{addr}]",
            mask = out(reg) _,
            addr = out(reg) _,
            offset = out(reg) _,
        );
    }

    rprintln!("after");

    row1.set_high().void_unwrap();

    rprintln!("done");

    // ON
    // col1.set_low().void_unwrap();

    // let mut col4 = board.display_pins.col4;
    // col4.set_low().void_unwrap();
    // row1.set_high().void_unwrap();

    // loop {
    //     row1.set_high().void_unwrap();
    //     rprintln!("Light!");
    //     nb::block!(timer.wait()).void_unwrap();

    //     row1.set_low().void_unwrap();
    //     rprintln!("Dark!");
    //     nb::block!(timer.wait()).void_unwrap();
    // }
    loop{}
}
