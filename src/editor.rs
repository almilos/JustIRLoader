use std::sync::Arc;

use nih_plug::editor::Editor;
use nih_plug_egui::{create_egui_editor, egui::Vec2, resizable_window::ResizableWindow};

use crate::ir_loader::IRLoaderParams;
use std::path::PathBuf;

pub(crate) fn create(params: Arc<IRLoaderParams>) -> Option<Box<dyn Editor>> {
    let egui_state = params.editor_state.clone();
    create_egui_editor(
        egui_state.clone(),
        (),
        |_, _| {},
        move |ctx, _setter, _state| {
            ResizableWindow::new("res-wind")
                .min_size(Vec2::new(128.0, 128.0))
                .show(ctx, egui_state.as_ref(), |ui| {
                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        ui.add_space(20.0);
                        ui.vertical(|ui| {
                            ui.heading("Just IR Loader");

                            ui.add_space(10.0);

                            if ui.button("Pick file").clicked() {
                                let file = tinyfiledialogs::open_file_dialog(
                                    "Select an impulse response",
                                    "/",
                                    Some((&["*.wav"], "/")),
                                );

                                if file.is_some() {
                                    params.set_ir_file(PathBuf::from(file.unwrap()));
                                    params.set_need_reload(true);
                                }
                            }
                            ui.add_space(10.0);

                            ui.label(format!("{:?}", params.get_ir_file()));

                            ui.add_space(20.0);
                        });
                        ui.add_space(20.0);
                    });
                });
        },
    )
}
