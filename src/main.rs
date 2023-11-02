// SPDX-License-Identifier: MIT

use i_slint_backend_winit::WinitWindowAccessor;
use slint::{LogicalPosition, LogicalSize, WindowPosition, WindowSize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let main_window = MainWindow::new().unwrap();

    let weak_window = main_window.as_weak();
    main_window.on_popup_confirmed(move || {
        let window = weak_window.unwrap();
        window.hide().unwrap();
    });

    {
        let weak_window = main_window.as_weak();
        main_window.window().on_close_requested(move || {
            let window = weak_window.unwrap();

            window.invoke_show_confirm_popup();
            slint::CloseRequestResponse::KeepWindowShown
        });
    }
    {
        let weak_window = main_window.as_weak();
        main_window.on_start_pick_screen_color(move || {
            let window: MainWindow = weak_window.unwrap();

            window.set_cursor_position_changed(false);
            window.set_moving(false);

            window
                .window()
                .with_winit_window(|winit_win: &winit::window::Window| {
                    let monitor = winit_win.current_monitor().unwrap();
                    println!(
                        "screen width: {:?} height: {:?}",
                        monitor.size().width,
                        monitor.size().height
                    );
                    window
                        .window()
                        .set_size(WindowSize::Logical(LogicalSize::new(
                            ((monitor.size().width + 20) / 2) as f32,
                            ((monitor.size().height + 20) / 2) as f32,
                        )));
                    window
                        .window()
                        .set_position(WindowPosition::Logical(LogicalPosition::new(-10.0, -10.0)));
                });
        });
    }

    {
        let weak_window = main_window.as_weak();
        main_window.on_stop_pick_screen_color(move || {
            let window = weak_window.unwrap();
            window
                .window()
                .with_winit_window(|winit_win: &winit::window::Window| {
                    let monitor = winit_win.current_monitor().unwrap();
                    let width = monitor.size().width / 2;
                    let height = monitor.size().height / 2;
                    let window_width = 400 / 2;
                    let window_height = 600 / 2;
                    let center_width: i32 = (width - window_width) as i32;
                    let center_height: i32 = (height - window_height) as i32;
                    window
                        .window()
                        .set_size(WindowSize::Logical(LogicalSize::new(400.0, 600.0)));
                    window
                        .window()
                        .set_position(slint::PhysicalPosition::new(center_width, center_height));
                });
        });
    }

    {
        let weak_window = main_window.as_weak();
        main_window.on_mouse_move(move |circle_position| {
            println!("cursor position: {:?}", circle_position.as_str());
            if circle_position.as_str() != "0,0" {
                let window = weak_window.unwrap();
                let pos: Vec<f32> = circle_position
                    .as_str()
                    .split(",")
                    .map(|e| e.parse::<f32>().unwrap())
                    .collect();
                window.set_circle_position_x(pos[0] - window.get_picker_circle_radius() as f32);
                window.set_circle_position_y(pos[1] - window.get_picker_circle_radius() as f32);
                if window.get_cursor_position_changed() {
                    window.set_moving(true);
                } else {
                    window.set_cursor_position_changed(true);
                }
                println!("moving: {:?}", window.get_moving());
            }
            "".into()
        });
    }

    main_window
        .window()
        .with_winit_window(|winit_win: &winit::window::Window| {
            let monitor = winit_win.current_monitor().unwrap();
            let width = monitor.size().width / 2;
            let height = monitor.size().height / 2;
            let window_width = 400 / 2;
            let window_height = 600 / 2;
            let center_width: i32 = (width - window_width) as i32;
            let center_height: i32 = (height - window_height) as i32;
            main_window
                .window()
                .set_position(slint::PhysicalPosition::new(center_width, center_height));
        });
    main_window.run().unwrap();
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: i_slint_backend_android_activity::AndroidApp) {
    slint::platform::set_platform(Box::new(
        i_slint_backend_android_activity::AndroidPlatform::new_with_event_listener(app, |event| {
            eprintln!("Got event: {event:?}")
        }),
    ))
    .unwrap();
    main();
}
