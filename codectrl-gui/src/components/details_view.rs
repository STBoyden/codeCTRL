use super::details_view_components::draw_information_grid;
use crate::data::AppState;

use egui::Context;
use std::sync::Arc;

pub fn details_view(app_state: &mut AppState, ctx: &Context) {
    let style = Arc::clone(&ctx.style());

    egui::TopBottomPanel::bottom("log_information")
        .resizable(true)
        .default_height(350.0)
        .max_height(450.0)
        .min_height(250.0)
        .show(ctx, |ui| draw_information_grid(app_state, ctx, &style, ui));
}
