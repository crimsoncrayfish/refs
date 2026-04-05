//pub fn draw_overlay() {}
//pub fn draw_menu() {}
//pub fn draw_debug_menu() {}
//pub fn draw_help_menu() {}

use crate::{app::AppState, camera::Camera, world::world::World};
use eframe::egui::{self, Color32, Response, Stroke, pos2};

pub fn draw_world(painter: &egui::Painter, world: &World, camera: &Camera, state: &mut AppState) {
    let mut drawn_entities = 0;
    for (id, element) in world.entities().iter() {
        if !camera.coordinates.contains(element.coord) {
            continue;
        }
        element.draw_at(
            painter,
            camera.world_pos2_to_pos2(element.coord),
            camera.zoom(),
            Some(*id) == world.selected_id(),
        );
        drawn_entities += 1;
    }
    state.set_drawn_entities(drawn_entities);
}

pub fn draw_debug_window(
    ctx: &egui::Context,
    app_state: &AppState,
    camera: &Camera,
    world: &World,
) -> Option<Response> {
    let stuff = egui::Window::new("🪲 Debug Inspector")
        .default_pos(egui::pos2(10.0, 10.0))
        .resizable(true)
        .show(ctx, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

            // --- SECTION 1: SYSTEM STATS ---
            egui::CollapsingHeader::new("📊 System Stats")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label(format!("FPS: {:.0}", app_state.last_fps()));
                    ui.label(format!("Elapsed: {}s", app_state.elapsed().as_secs()));
                    ui.label(format!("Entities: {}", world.entities().len()));
                    ui.label(format!("Drawn: {}", app_state.drawn_entities()));
                });

            // --- SECTION 2: CAMERA & MOUSE ---
            egui::CollapsingHeader::new("🎥 Camera & Mouse")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label(format!("Zoom: {:.2}x", camera.zoom()));
                    ui.label(format!(
                        "Offset: {:.0}, {:.0}",
                        camera.offset.x, camera.offset.y
                    ));

                    if let Some(pos) = app_state.mouse_pos() {
                        let world_pos = camera.pos2_to_world_pos2(pos);
                        ui.label(format!("Screen: {:.1}, {:.1}", pos.x, pos.y));
                        ui.label(format!("World:  {:.1}, {:.1}", world_pos.x, world_pos.y));
                    }
                });

            ui.separator();

            ui.heading("🔍 Entity List");
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (id, entity) in world.entities() {
                        let is_selected = world.selected_id() == Some(*id);

                        let header_text = if is_selected {
                            format!("▶ Entity {:?}", id)
                        } else {
                            format!("Entity {:?}", id)
                        };

                        egui::CollapsingHeader::new(header_text)
                            .id_salt(id)
                            .show(ui, |ui| {
                                ui.label(format!("{:#?}", entity));
                            });
                    }
                });
        });
    if let Some(inner) = stuff {
        Some(inner.response)
    } else {
        None
    }
}

pub fn draw_grid(painter: &egui::Painter, rect: egui::Rect, camera: &Camera) {
    let ss = 25.0 * camera.zoom(); //screen spacing - at 1 zoom have a spacing of 25 px
    let color = Color32::from_rgba_unmultiplied(80, 80, 90, 60);

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
