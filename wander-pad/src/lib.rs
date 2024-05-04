// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use eframe::egui;
use wander::preludes::common;
use wander::{run, NoHostType};

pub fn start_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "WanderPad",
        options,
        Box::new(|cc| {
            // This gives us image support:
            Box::<WanderPad>::default()
        }),
    )
}

struct WanderPad {
    script: String,
    result: String,
}

impl Default for WanderPad {
    fn default() -> Self {
        Self {
            script: "".to_owned(),
            result: "".to_owned(),
        }
    }
}

impl eframe::App for WanderPad {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Run").clicked() {
                let script = self.script.clone();
                self.result = match run(&script, &mut common::<NoHostType>()) {
                    Ok(value) => value.to_string(),
                    Err(err) => err.0,
                }
            }
            ui.text_edit_multiline(&mut self.script);
            ui.label(&self.result);
        });
    }
}
