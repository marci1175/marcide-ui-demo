use egui::{Rect, Color32, Layout, Image, Vec2, vec2};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {

        }
    }
}

impl TemplateApp {

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui_extras::install_image_loaders(ctx);
        
        egui::CentralPanel::default().show(ctx, |ui|{

            
            
            ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui|{
                ui.allocate_space(vec2(95. , ui.available_height() as f32));
                ui.label("text");

            });
            
            
            
        });

        egui::SidePanel::new(egui::panel::Side::Left, "Test").resizable(false).show_separator_line(false).show(ctx, |ui|{

            //menu paint strip

            ui.painter()
            .rect_filled(
                Rect::everything_left_of(95.),
                0.0,
                Color32::BLACK
            );
            
            //menu strip

            ui.with_layout(Layout::top_down(egui::Align::Min), |ui: &mut egui::Ui|{
                ui.allocate_ui(vec2(80.,80.), |ui|{
                    ui.menu_image_button(egui::include_image!("C:\\Users\\marci\\Desktop\\Marcide icons\\files.png"), |ui|{

                });
                
                });
                
                
            });
        });
            
        
    }
}
