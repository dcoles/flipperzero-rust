//! Example Images application.
//! See <https://github.com/flipperdevices/flipperzero-firmware/blob/dev/applications/examples/example_images/example_images.c>

#![no_std]
#![no_main]

use core::time::Duration;
use flipperzero::{
    furi::message_queue::MessageQueue,
    gui::{
        canvas::CanvasView,
        icon::Icon,
        view_port::{ViewPort, ViewPortCallbacks},
        Gui, GuiLayer,
    },
    input::{InputEvent, InputKey, InputType},
};

extern crate flipperzero_alloc;

use flipperzero_rt as rt;
use flipperzero_sys as sys;

rt::manifest!(name = "Example: Images");
rt::entry!(main);

// NOTE: `*mut`s are required to enforce `unsafe` since there are raw pointers involved
static mut TARGET_FRAMES: [*const u8; 1] = [include_bytes!("icons/rustacean-48x32.icon").as_ptr()];
static mut SYS_ICON: sys::Icon = sys::Icon {
    width: 48,
    height: 32,
    frame_count: 1,
    frame_rate: 0,
    frames: unsafe { TARGET_FRAMES.as_ptr() },
};

#[repr(C)]
struct ImagePosition {
    pub x: u8,
    pub y: u8,
}

fn main(_args: *mut u8) -> i32 {
    // SAFETY: `Icon` is a read-only;
    // there will be a safe API for this in this future
    let icon = unsafe { Icon::from_raw(&SYS_ICON as *const _ as *mut _) };

    // Configure view port
    struct State<'a> {
        exit_queue: &'a MessageQueue<()>,
        image_position: ImagePosition,
        target_icon: &'a Icon,
        hidden: bool,
    }

    impl ViewPortCallbacks for State<'_> {
        fn on_draw(&mut self, mut canvas: CanvasView) {
            canvas.clear();
            if !self.hidden {
                // Screen is 128x64 px
                canvas.draw_icon(
                    self.image_position.x % 128,
                    self.image_position.y % 64,
                    self.target_icon,
                );
            }
        }

        fn on_input(&mut self, event: InputEvent) {
            if matches!(event.r#type, InputType::Press | InputType::Repeat) {
                match event.key {
                    InputKey::Left => {
                        self.image_position.x = self.image_position.x.saturating_sub(2)
                    }
                    InputKey::Right => {
                        self.image_position.x = self.image_position.x.saturating_add(2)
                    }
                    InputKey::Up => self.image_position.y = self.image_position.y.saturating_sub(2),
                    InputKey::Down => {
                        self.image_position.y = self.image_position.y.saturating_add(2)
                    }
                    // to be a bit more creative than the original example
                    // we make `Ok` button (un)hide the canvas
                    InputKey::Ok => self.hidden = !self.hidden,
                    _ => {
                        let _ = self.exit_queue.put_now(());
                    }
                }
            }
        }
    }

    // The original example has all `InputEvent`s transferred via `MessageQueue`
    // While this is possible, there is no need for this
    // since we do all the handling in `on_input_event`
    // thus we only have to send a single object indicating shutdown
    let exit_queue = MessageQueue::new(1);
    let view_port = ViewPort::new(State {
        exit_queue: &exit_queue,
        image_position: ImagePosition { x: 0, y: 0 },
        target_icon: &icon,
        hidden: false,
    });

    // Register view port in GUI
    let mut gui = Gui::new();
    let mut view_port = gui.add_view_port(view_port, GuiLayer::Fullscreen);

    let mut running = true;
    while running {
        if exit_queue.get(Duration::from_millis(100)).is_ok() {
            running = false
        }
        view_port.update();
    }

    0
}
