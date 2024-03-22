#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

#[skyline::main(name = "no_hold_buffer")]
pub fn main() {
    // Custom buffer-state handling
    // Always uses the hitlag handling that cat4 uses
    let _ = skyline::patching::Patch::in_text(0x6bd448).nop();
    let _ = skyline::patching::Patch::in_text(0x6bd4a4).nop();
    // Stubs setting the buffer lifetime to 2 if held
    let _ = skyline::patching::Patch::in_text(0x6bd53c).nop();
    let _ = skyline::patching::Patch::in_text(0x6bd5b8).nop();
    // Stubs adding 1 to the buffer when released
    let _ = skyline::patching::Patch::in_text(0x6bd518).nop();
    let _ = skyline::patching::Patch::in_text(0x6bd5d4).nop();
}