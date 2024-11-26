use crate::{viewer::ViewerContext, ViewerPanel};

#[derive(Default)]
pub(crate) struct TracingPanel {
    constant_redraw: bool,
}

impl ViewerPanel for TracingPanel {
    fn title(&self) -> String {
        "Load data".to_owned()
    }

    fn ui(&mut self, ui: &mut egui::Ui, _: &mut ViewerContext) {
        ui.checkbox(&mut self.constant_redraw, "Constant redraw");

        // Nb: this redraws the whole context so this will include the splat views.
        if self.constant_redraw {
            ui.ctx().request_repaint();
        }
    }
}
