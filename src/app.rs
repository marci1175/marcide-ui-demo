use std::{env, path::PathBuf, fs::DirEntry};

use egui::{Rect, Color32, Layout, Image, Vec2, vec2, Frame};
use image::EncodableLayout;
use egui_dock::{DockArea, NodeIndex, Style, Tree, DockState};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    files: Vec<DirEntry>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            files: Vec::new(),
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

struct TabViewer<'a> {
    //tree: &'a mut Tree<usize>,
    added_nodes: &'a mut Vec<NodeIndex>,
    ctx: &'a egui::Context,
    frame: &'a mut eframe::Frame,
    data: &'a mut TemplateApp,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppData {
    app_data: TemplateApp,
    #[serde(skip)]
    tree: Tree<usize>,
    #[serde(skip)]
    f11_is_held: bool,
}
impl Default for AppData {
    fn default() -> Self {
        Self {
            app_data: TemplateApp::default(),
            tree: Tree::new(vec![1, 2, 3]),
            f11_is_held: false,
        }
    }
}
impl AppData {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}
impl eframe::App for AppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        let mut added_nodes = Vec::new();
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
                    
                    let file_list = ui.menu_image_button(egui::include_image!("C:\\Users\\marci\\Desktop\\Marcide icons\\files.png"), |ui|{
                        DockArea::new(&mut self.tree)
                        .show_close_buttons(true)
                        //.show_add_buttons(true)
                        .style({
                            let mut style = Style::from_egui(ctx.style().as_ref());
                            style.tab_bar.fill_tab_bar = true;
                            style
                        })
                        .show(
                            ctx,
                            &mut TabViewer {
                                //tree: &mut self.tree,
                                added_nodes: &mut added_nodes,
                                ctx,
                                frame: _frame,
                                data: &mut self.app_data,
                            },
                        );
                    });

                    if file_list.response.clicked() {

                        self.app_data.files.clear();
                        let current_dir = match env::current_dir(){
                            Ok(ok) => {ok},
                            Err(_) => {PathBuf::new()},
                        };
                        for entry in current_dir.read_dir().expect("read_dir call failed") {
                            if let Ok(entry) = entry {
                                self.app_data.files.push(entry);
                            }
                        }
                        for i in &self.app_data.files {
                            self.tree.push_to_first_leaf(1);
                        }

                    }
                
                });
                
                
            });
        });
    }
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}


impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = usize;
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        for i in 0..*tab - 1 {
            ui.label(self.data.files[i].file_name().to_str().unwrap());
        }
    }
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        format!("file{}",tab).to_string().into()
    }
}


