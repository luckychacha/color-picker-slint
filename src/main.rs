// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

use slint::{Color, FilterModel, Model, SortModel};
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
    // let tmp = slint::LogicalSize::new(1800.0, 1900.0);
    // main_window
    //     .window()
    //     .set_position(slint::WindowPosition::Physical(slint::PhysicalPosition { x: 0, y: 0 }));
    // println!(
    //     "{:?} {:?}",
    //     main_window.window().size(),
    //     main_window.window().is_visible()
    // );

    slint::slint! {
        // import { StandardButton } from "std-widgets.slint";

        import { LineEdit, CheckBox } from "std-widgets.slint";
        export global Logic  {
            pure callback mouse-move(string) -> string;

        }

        struct CirclePosition {
            x: float,
            y: float,
        }

        component Circle inherits Rectangle {
            width: 50px;
            height: 50px;
            border-width: 4px;
            border-radius: root.width / 10;
            // animate x { duration: 250ms; easing: ease-in; }
            // animate y { duration: 250ms; easing: ease-in-out; }
            // animate background { duration: 250ms; }
        }

        export component ConfirmDialog inherits Window {
            no-frame: true;
            in-out property <bool> show-circle: false;
            in-out property <CirclePosition> circle_position: { x: 5.0, y: 5.0 };
            in-out property <color> circle_border_color: transparent;
            in-out property <float> circle_position_x: 1.0;
            in-out property <float> circle_position_y: 1.0;

            callback move-action(string);
            callback try_to_show_circle();

            background: transparent;

            i-touch-area := TouchArea {
                mouse-cursor: none;
            }

            Circle {

                background: transparent;
                border-color: root.circle_border_color;
                x: circle_position_x * 1px;
                y: circle_position_y * 1px;
                Image {
                    source: @image-url("ui/licon.png");
                    visible: root.show-circle;
                    // image-fit default is `contain` when in layout, preserving aspect ratio
                }
            }

            Text {
                visible: false;
                text: {
                    Logic.mouse-move(i-touch-area.mouse-x / 1px + "," + i-touch-area.mouse-y / 1px);
                }
            }

            // CheckBox {
            //     text: "show_circle";

            //     toggled => {
            //         root.try_to_show_circle();
            //     }
            // }

        }
    }
    let confirm_dialog = ConfirmDialog::new().unwrap();
    confirm_dialog.set_show_circle(false);

    confirm_dialog.global::<Logic>().on_mouse_move({
        let confirm_dialog_weak = confirm_dialog.as_weak();

        move |circle_position| {
            // println!("before: {:?}", circle_visible);
            println!("aaaa: {:?}", circle_position.as_str());
            if let Some(confirm_dialog) = confirm_dialog_weak.upgrade() {
                if circle_position.as_str() == "0,0" {
                } else {
                    confirm_dialog.set_show_circle(true);
                    confirm_dialog.set_circle_border_color(Color::from_rgb_u8(0, 0, 0));
                }
                // confirm_dialog.set_show_circle(true);

                // confirm_dialog.set_circle_position(CirclePosition {
                //     x: circle_position.as_str().split(",").collect::<Vec<&str>>()[0]
                //         .parse::<f32>()
                //         .unwrap(),
                //     y: circle_position.as_str().split(",").collect::<Vec<&str>>()[1]
                //         .parse::<f32>()
                //         .unwrap(),
                // });
                let pos: Vec<f32> = circle_position
                    .as_str()
                    .split(",")
                    .map(|e| e.parse::<f32>().unwrap())
                    .collect();
                confirm_dialog.set_circle_position_x(pos[0]);
                confirm_dialog.set_circle_position_y(pos[1]);

                // confirm_dialog.set_show_circle(true);
                // println!("get_show_circle: {:?}", confirm_dialog.get_show_circle());
                // println!("aaaaa");
                // println!(
                //     "show-circle: {:?} changed: {:?}",
                //     confirm_dialog.get_show_circle(),
                //     confirm_dialog.get_changed()
                // );
                // confirm_dialog.set_show_circle(true);
                // confirm_dialog.set_changed(true);
            }
            "".into()
        }
    });

    // confirm_dialog.on_try_to_show_circle({
    //     let confirm_dialog_weak = confirm_dialog.as_weak();
    //     move || {
    //         if let Some(confirm_dialog) = confirm_dialog_weak.upgrade() {
    //             confirm_dialog.set_show_circle(true);
    //             confirm_dialog.set_circle_border_color(Color::from_rgb_u8(0, 0, 0));
    //         }
    //     }
    // });

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

            println!("todo_model: {}", todo_model.row_count());
            let _ = confirm_dialog.show();

            confirm_dialog
                .window()
                .set_position(slint::WindowPosition::Physical(slint::PhysicalPosition {
                    x: 0,
                    y: 0,
                }));

            confirm_dialog
                .window()
                .set_size(slint::WindowSize::Logical(slint::LogicalSize::new(
                    500.0, 500.0,
                )));
        }
    });

    main_window.set_show_header(true);
    main_window.set_todo_model(todo_model.into());

    main_window.run().unwrap();
}
