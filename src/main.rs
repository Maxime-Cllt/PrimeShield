mod utils;

mod exponential_fast;
mod inverse_modular;
mod prime_gen;
#[cfg(test)]
mod tests;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mon Application Egui",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    ).expect("TODO: panic message");
}

struct MyApp {
    name: String,
    age: u32,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: "".to_owned(),
            age: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mon Formulaire");
            ui.horizontal(|ui| {
                ui.label("Votre nom: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.horizontal(|ui| {
                ui.label("Votre âge: ");
                ui.add(egui::Slider::new(&mut self.age, 0..=120).text("âge"));
            });
            if ui.button("Soumettre").clicked() {
                println!("Nom: {}, Âge: {}", self.name, self.age);
            }
        });
    }
}