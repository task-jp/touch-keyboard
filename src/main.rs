#![no_std]
#![no_main]
slint::include_modules!();

#[mcu_board_support::entry]
fn main() -> ! {
    mcu_board_support::init();

    let app = App::new().unwrap();
    app.global::<Keyboard>().on_send(move |shortcut| {});
    app.run().unwrap();

    panic!("The demo should not quit");
}

// End of file
