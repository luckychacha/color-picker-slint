// SPDX-License-Identifier: MIT


use i_slint_backend_winit::WinitWindowAccessor;
use image::imageops;
use screenshots::Screen;
use slint::{
    Image, LogicalSize, Rgba8Pixel, SharedPixelBuffer, WindowSize,
};

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
            window.set_has_image(false);

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
            window.set_cursor_position_changed(false);
            window.set_moving(false);
            window.set_has_image(false);
            if let Some((screen_width, screen_height)) = get_current_monitor_size(window.window()) {
                println!("windows has image: {:?}", window.get_has_image());
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
            let window = weak_window.unwrap();
            // window.set_circle_position_x(100.0);
            // window.set_circle_position_y(100.0);
            window.set_moving(false);
            window.set_has_image(false);
            if !window.get_has_image() && circle_position.as_str() != "0,0" {
                let pos: Vec<f32> = circle_position
                    .as_str()
                    .split(",")
                    .map(|e| e.parse::<f32>().unwrap())
                    .collect();
                // let circle_position_x = pos[0] - window.get_picker_circle_radius() as f32;
                // let circle_position_y = pos[1] - window.get_picker_circle_radius() as f32;

                let circle_position_x = pos[0] + (window.get_picker_circle_radius() as f32) / 3.0;
                let circle_position_y = pos[1] + (window.get_picker_circle_radius() as f32) / 3.0;

                if window.get_cursor_position_changed() {
                    let screens = Screen::all().unwrap();
                    if let Some(image_buffer) = screens
                        .get(0)
                        .and_then(|f| {
                            if !window.get_has_image() {
                                f.capture_area(pos[0] as i32 - 15, pos[1] as i32 - 20, 30, 30).ok()
                            } else {
                                None
                            }
                        })
                    {
                        let image_buffer = imageops::resize(&image_buffer, 90, 90, imageops::FilterType::Nearest);
                        let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                            image_buffer.as_raw(),
                            image_buffer.width(),
                            image_buffer.height(),
                        );
                        window.set_cursor_around_screenshot(Image::from_rgba8(buffer));

                        window.set_circle_position_x(circle_position_x);
                        window.set_circle_position_y(circle_position_y);
                        if !window.get_has_image() {
                            window.set_has_image(true);
                            window.set_moving(true);
                        }
                    }
                } else {
                    window.set_cursor_position_changed(true);
                    window.set_has_image(false);
                }
                println!("moving: {:?} has_image: {:?}", window.get_moving(), window.get_has_image());
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
            // 0,
            // 0,
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
