use crate::*;

mod control_test;
use control_test::*;

mod canvas_test;
use canvas_test::*;


#[derive(Default)]
pub struct TestControlPanel {
    window: Window,
    controls_test_button: Button,
    canvas_test_button: Button,

    controls_tests: ControlsTest,
    canvas_tests: CanvasTest,
}

mod test_control_panel_ui {
    use super::*;
    use crate::{NativeUi, SystemError};
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct TestControlPanelUi {
        inner: TestControlPanel
    }

    impl NativeUi<TestControlPanel, TestControlPanelUi> for TestControlPanel {
        fn build_ui(mut data: TestControlPanel) -> Result<Rc<TestControlPanelUi>, SystemError> {
            use crate::Event as E;

            // Controls
            Window::builder()
                .flags(WindowFlags::WINDOW | WindowFlags::VISIBLE)
                .size((200, 100))
                .position((1100, 300))
                .title("Tests Control Panel")
                .build(&mut data.window)?;

            Button::builder()
                .text("Control tests")
                .parent(&data.window)
                .build(&mut data.controls_test_button)?;

            Button::builder()
                .text("Canvas tests")
                .enabled(cfg!(feature = "canvas"))
                .parent(&data.window)
                .build(&mut data.canvas_test_button)?;

            // Partials
            ControlsTest::build_partial(&mut data.controls_tests, Some(&data.window))?;
            CanvasTest::build_partial(&mut data.canvas_tests, Some(&data.window))?;

            // Wrap-up
            let ui = Rc::new(TestControlPanelUi { inner: data });

            // Events
            let mut window_handles = vec![&ui.window.handle];
            window_handles.append(&mut ui.controls_tests.handles());
            window_handles.append(&mut ui.canvas_tests.handles());

            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |evt, _evt_data, handle| {

                    evt_ui.controls_tests.process_event(evt, &_evt_data, handle);
                    evt_ui.canvas_tests.process_event(evt, &_evt_data, handle);

                    match evt {
                        E::OnButtonClick =>
                            if handle == evt_ui.controls_test_button.handle {
                                show_control_test(&evt_ui.inner, evt);
                            } else if handle == evt_ui.canvas_test_button.handle {
                                show_canvas_test(&evt_ui.inner, evt);
                            },
                        E::OnWindowClose => 
                            if handle == evt_ui.window.handle {
                                close(&evt_ui.inner, evt);
                            },
                        _ => {}
                    }
                };

                bind_event_handler(handle, handle_events);
            }

            // Layouts
            VBoxLayout::builder()
                .parent(&ui.window)
                .child(0, &ui.controls_test_button)
                .child(1, &ui.canvas_test_button)
                .build();

            Ok(ui)
        }
    }

    impl Deref for TestControlPanelUi {
        type Target = TestControlPanel;

        fn deref(&self) -> &TestControlPanel {
            &self.inner
        }
    }

}

fn show_control_test(app: &TestControlPanel, _e: Event) {
    app.controls_tests.window.set_visible(true);
    app.controls_tests.panel.set_visible(true);
    app.controls_tests.window.set_focus();
}

fn show_canvas_test(app: &TestControlPanel, _e: Event) {
    app.canvas_tests.window.set_visible(true);
    app.canvas_tests.window.set_focus();
}

fn close(_app: &TestControlPanel, _e: Event) {
    stop_thread_dispatch();
}

#[test]
fn test_everything() {
    enable_visual_styles();
    init_common_controls().expect("Failed to init controls");
    
    let app = TestControlPanel::build_ui(Default::default()).expect("Failed to build UI");

    app.window.set_focus();

    dispatch_thread_events();
}
