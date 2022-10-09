use crate::app::ui::RenderBox;
use crate::renderer::RenderParams;
use std::time::Instant;

mod ui;

pub struct MyApp {
    render_box: RenderBox,
    last_frame_time: Instant,
    params: RenderParams,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            render_box: RenderBox::new(),
            last_frame_time: Instant::now(),
            params: RenderParams::default(),
        }
    }
}
