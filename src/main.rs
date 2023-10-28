// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

use slint::platform::WindowAdapter;
use slint::{FilterModel, Model, SortModel, Window};
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

// struct MyWindowAdapter {
//     window: Window,
// }

// impl WindowAdapter for MyWindowAdapter {
//     fn window(&self) -> &Window {
//         &self.window
//     }

//     fn size(&self) -> slint::PhysicalSize {
//         todo!()
//     }

//     fn renderer(&self) -> &dyn slint::platform::Renderer {
//         todo!()
//     }
// }

// fn create_window_adapter() -> Rc<dyn WindowAdapter> {
//     Rc::<MyWindowAdapter>::new_cyclic(|weak| MyWindowAdapter {
//         window: Window::new(weak.clone()),
//     })
// }

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let todo_model = Rc::new(slint::VecModel::<TodoItem>::from(vec![
        TodoItem {
            checked: true,
            title: "Implement the .slint file".into(),
        },
        TodoItem {
            checked: true,
            title: "Do the Rust part".into(),
        },
        TodoItem {
            checked: false,
            title: "Make the C++ code".into(),
        },
        TodoItem {
            checked: false,
            title: "Write some JavaScript code".into(),
        },
        TodoItem {
            checked: false,
            title: "Test the application".into(),
        },
        TodoItem {
            checked: false,
            title: "Ship to customer".into(),
        },
        TodoItem {
            checked: false,
            title: "???".into(),
        },
        TodoItem {
            checked: false,
            title: "Profit".into(),
        },
    ]));

    let main_window = MainWindow::new().unwrap();
    let tmp = slint::LogicalSize::new(1800.0, 1900.0);
    // main_window
    //     .window()
    //     .set_position(slint::WindowPosition::Physical(slint::PhysicalPosition { x: 0, y: 0 }));
    // println!(
    //     "{:?} {:?}",
    //     main_window.window().size(),
    //     main_window.window().is_visible()
    // );
    let confirm_dialog = ConfirmDialog::new().unwrap();
    main_window.on_todo_added({
        let todo_model = todo_model.clone();
        move |text| {
            todo_model.push(TodoItem {
                checked: false,
                title: text,
            })
        }
    });
    main_window.on_remove_done({
        let todo_model = todo_model.clone();
        move || {
            let mut offset = 0;
            for i in 0..todo_model.row_count() {
                if todo_model.row_data(i - offset).unwrap().checked {
                    todo_model.remove(i - offset);
                    offset += 1;
                }
            }
        }
    });

    slint::slint! {
        // import { StandardButton } from "std-widgets.slint";

        export component ConfirmDialog inherits Window {
            no-frame: true;
            confirm_popup_text := Text {
                text: "Some items are not done, are you sure you wish to quit?";
                wrap: word-wrap;
            }

            // StandardButton { kind: yes; }
            // StandardButton { kind: no; }
        }
    }

    // let _ = confirm_dialog.show();
    // confirm_dialog
    //     .window()
    //     .set_size(slint::WindowSize::Logical(slint::LogicalSize::new(
    //         1000.0, 1000.0,
    //     )));
    // confirm_dialog
    //     .window()
    //     .set_position(slint::WindowPosition::Physical(slint::PhysicalPosition {
    //         x: 0,
    //         y: 0,
    //     }));
    // confirm_dialog.hide().unwrap();
    // let weak_window = main_window.as_weak();
    // let weak_confirm_dialog = confirm_dialog.as_weak();
    // confirm_dialog.on_yes_clicked(move || {
    //     weak_window.unwrap().hide();
    //     weak_confirm_dialog.unwrap().hide();
    // });
    // let weak_confirm_dialog = confirm_dialog.as_weak();
    // confirm_dialog.on_no_clicked(move || {
    //     weak_confirm_dialog.unwrap().hide();
    // });

    main_window.window().on_close_requested({
        let todo_model = todo_model.clone();
        move || {
            if todo_model.iter().any(|t| !t.checked) {
                // let _ = confirm_dialog.show();
                // confirm_dialog.window().set_size(slint::WindowSize::Logical(
                //     slint::LogicalSize::new(500.0, 500.0),
                // ));
                // confirm_dialog
                //     .window()
                //     .set_position(slint::WindowPosition::Physical(slint::PhysicalPosition {
                //         x: 200,
                //         y: 0,
                //     }));

                slint::CloseRequestResponse::KeepWindowShown
            } else {
                slint::CloseRequestResponse::HideWindow
            }
        }
    });

    main_window.on_apply_sorting_and_filtering({
        let weak_window = main_window.as_weak();
        let todo_model = todo_model.clone();

        move || {
            let window = weak_window.unwrap();
            window.set_todo_model(todo_model.clone().into());

            if window.get_hide_done_items() {
                window.set_todo_model(
                    Rc::new(FilterModel::new(window.get_todo_model(), |e| !e.checked)).into(),
                );
            }

            if window.get_is_sort_by_name() {
                window.set_todo_model(
                    Rc::new(SortModel::new(window.get_todo_model(), |lhs, rhs| {
                        lhs.title.to_lowercase().cmp(&rhs.title.to_lowercase())
                    }))
                    .into(),
                );
            }

            // confirm_dialog.hide().unwrap();
            println!("todo_model: {}", todo_model.row_count());
            let _ = confirm_dialog.show();
            confirm_dialog
                .window()
                .set_size(slint::WindowSize::Logical(slint::LogicalSize::new(
                    500.0, 500.0,
                )));
            confirm_dialog
                .window()
                .set_position(slint::WindowPosition::Physical(slint::PhysicalPosition {
                    x: 200 + todo_model.row_count() as i32 * 10,
                    y: 0,
                }));
        }
    });

    main_window.set_show_header(true);
    main_window.set_todo_model(todo_model.into());

    main_window.run().unwrap();
}
