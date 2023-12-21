#![no_std]
#![no_main]
slint::include_modules!();
use slint::Model;
mod pico_st7789;

#[pico_st7789::entry]
fn main() -> ! {
    pico_st7789::init();

    let app = App::new().unwrap();

    app.global::<Keyboard>().on_send(move |modifier, keycodes| {
        let len = keycodes.row_count();
        if len == 0 {
            return;
        }
        let m: u8 = modifier as u8;
        let mut k: [u8; 6] = [0; 6];
        for i in 0..len {
            k[i] = keycodes.row_data(i).unwrap() as u8;
        }

        unsafe {
            let _ = pico_st7789::KEY_INFO.insert(pico_st7789::KeyInfo {
                modifier: m,
                keycodes: k,
            });
        };
    });

    app.run().unwrap();

    panic!("The demo should not quit");
}

// End of file
