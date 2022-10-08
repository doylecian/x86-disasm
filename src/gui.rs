use eframe::egui;

// #[derive(Default)]
pub struct DisassemblyApp {
    hex_view_rows: u32,
    hex_view_columns: u32
}

impl Default for DisassemblyApp {
    fn default() -> Self {
        Self { hex_view_rows: 10, hex_view_columns: 10 }
    }
}

impl DisassemblyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for DisassemblyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("HexView").show(ui, |ui| {
                for _ in 0..self.hex_view_rows {
                    for _ in 0..self.hex_view_columns {
                        let button = egui::Button::new(
                            egui::RichText::new("FF"),
                        )
                        .frame(false);
            
                        let tooltip_ui = |ui: &mut egui::Ui| {
                            ui.label("Click to copy");
                        };
            
                        ui.add(button).on_hover_ui(tooltip_ui);
                    }
                    ui.end_row();
                }
            });
        });
    }
}