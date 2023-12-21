#![no_std]
#![no_main]
slint::include_modules!();

mod pico_st7789;
pub use pico_st7789::*;

#[pico_st7789::entry]
fn main() -> ! {
    pico_st7789::init();

    let app = App::new().unwrap();
    app.global::<Keyboard>().on_send(move |shortcut| {});
    app.run().unwrap();

    panic!("The demo should not quit");
}

// End of file
