// SPDX-License-Identifier: MIT

use i_slint_backend_winit::WinitWindowAccessor;
use slint::{LogicalPosition, LogicalSize, WindowPosition, WindowSize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

fn get_current_monitor_size(window: &slint::Window) -> Option<(f32, f32)> {
    window.with_winit_window(|winit_win: &winit::window::Window| {
        let monitor = winit_win.current_monitor().unwrap();
        (monitor.size().width as f32, monitor.size().height as f32)
    })
}

fn set_window_position_and_size(window: &slint::Window, width: f32, height: f32, x: i32, y: i32) {
    window.set_size(WindowSize::Logical(LogicalSize::new(width, height)));
    window.set_position(slint::PhysicalPosition::new(x, y));
}

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
            if let Some((screen_width, screen_height)) = get_current_monitor_size(window.window()) {
                set_window_position_and_size(
                    window.window(),
                    (screen_width + 20.0) / 2.0,
                    (screen_height + 20.0) / 2.0,
                    -10,
                    -10,
                );
            }
        });
    }

    {
        let weak_window = main_window.as_weak();
        main_window.on_stop_pick_screen_color(move || {
            let window = weak_window.unwrap();
            if let Some((screen_width, screen_height)) = get_current_monitor_size(window.window()) {
            set_window_position_and_size(
                window.window(),
                400.0,
                600.0,
                (screen_width / 2.0 - 200.0) as i32,
                (screen_height / 2.0 - 300.0) as i32,
            );
                }
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

    if let Some((screen_width, screen_height)) = get_current_monitor_size(main_window.window()) {
        set_window_position_and_size(
            main_window.window(),
            400.0,
            600.0,
            (screen_width / 2.0 - 200.0) as i32,
            (screen_height / 2.0 - 300.0) as i32,
        );
    }
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
