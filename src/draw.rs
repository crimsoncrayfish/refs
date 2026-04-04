//pub fn draw_world() {}
//pub fn draw_debug_menu() {}
//pub fn draw_overlay() {}
//pub fn draw_menu() {}
//pub fn draw_debug_menu() {}
//pub fn draw_help_menu() {}

use crate::{app::AppState, camera::Camera, world::world::World};
use eframe::egui::{self, Color32, Stroke, pos2};

pub fn draw_world(painter: &egui::Painter, world: &World, camera: &Camera) {
    for element in world.entities() {
        if !camera.coordinates.contains(element.coord) {
            continue;
        }
        painter.circle_filled(
            camera.world_pos2_to_pos2(element.coord),
            10.0,
            Color32::from_rgb(100, 100, 100),
        );
    }
}

pub fn draw_debug_window(
    ctx: &egui::Context,
    app_state: &AppState,
    camera: &Camera,
    world: &World,
) {
    egui::Area::new(egui::Id::new("debug_info"))
        .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(10.0, -10.0))
        .show(ctx, |ui| {
            egui::Frame::NONE
                .fill(egui::Color32::from_black_alpha(150))
                .corner_radius(4.0)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.visuals_mut().override_text_color = Some(egui::Color32::from_gray(180));
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                        ui.monospace(format!("Elapsed {}", app_state.elapsed().as_secs()));

                        ui.monospace(format!(
                            "World Coordinates Min = {}: Max = {}",
                            camera.coordinates.min(),
                            camera.coordinates.max()
                        ));
                        ui.monospace(format!("Entity Count {}", world.entities().iter().count()));

                        if let Some(pos) = app_state.mouse_pos() {
                            let world_pos = camera.pos2_to_world_pos2(pos);
                            ui.monospace(format!(
                                "Mouse World pos {:.1}:{:.1}",
                                world_pos.x, world_pos.y
                            ));
                        } else {
                            ui.monospace("Mouse not in window");
                        }

                        if let Some(pos) = app_state.mouse_pos() {
                            ui.monospace(format!("Mouse {:.1}:{:.1}", pos.x, pos.y));
                        } else {
                            ui.monospace("Mouse not in window");
                        }

                        ui.monospace(format!("Offset {}", camera.offset));
                        ui.monospace(format!("Zoom factor {}", camera.zoom()));
                        ui.monospace(format!("Show grid {}", app_state.show_grid));

                        ui.monospace(format!(
                            "Pan: ({:.0}, {:.0}) | Middle-drag to pan | FPS {:.0}",
                            camera.offset.x,
                            camera.offset.y,
                            app_state.last_fps()
                        ));
                    });
                });
        });
}

pub fn draw_grid(painter: &egui::Painter, rect: egui::Rect, camera: &Camera) {
    let ss = 25.0 * camera.zoom(); //screen spacing - at 1 zoom have a spacing of 25 px
    let color = Color32::from_rgba_unmultiplied(80, 80, 90, 60);

    // Vertical lines
    let screen_offset = camera.screen_offset();
    let mut x = (screen_offset.x % ss + ss) % ss;
    while x < rect.width() {
        painter.line_segment(
            [
                pos2(rect.left() + x, rect.top()),
                pos2(rect.left() + x, rect.bottom()),
            ],
            Stroke::new(1.0, color),
        );
        x += ss;
    }

    // Horizontal lines
    let mut y = (screen_offset.y % ss + ss) % ss;
    while y < rect.height() {
        painter.line_segment(
            [
                pos2(rect.left(), rect.top() + y),
                pos2(rect.right(), rect.top() + y),
            ],
            Stroke::new(1.0, color),
        );
        y += ss;
    }
}
pub fn draw_coords(painter: &egui::Painter, rect: egui::Rect, camera: &Camera) {
    let ss = 100.0 * camera.zoom(); //At zoom = 1.0 100px spacing

    let screen_offset = camera.screen_offset();
    let mut x = (screen_offset.x % ss + ss) % ss - ss;
    while x < rect.width() {
        let mut y = (screen_offset.y % ss + ss) % ss - ss;
        while y < rect.height() {
            let world_coord = camera.pos2_to_world_pos2(pos2(x, y));
            painter.text(
                pos2(x, y),
                egui::Align2::LEFT_TOP,
                format!("({:.1}:{:.1})", world_coord.x, world_coord.y),
                egui::FontId::monospace(8.0),
                Color32::from_rgba_unmultiplied(180, 180, 180, 160),
            );

            y += ss;
        }
        x += ss;
    }
}
