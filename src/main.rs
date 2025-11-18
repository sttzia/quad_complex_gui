mod quadratic;

use eframe::{egui, NativeOptions};
use num_complex::Complex;
use quadratic::quadratic_roots;

#[derive(Default)]
struct QuadraticApp {
    a_text: String,
    b_text: String,
    c_text: String,
    discriminant: Option<f64>,
    output: String,
    error_message: String,
    root_type: String,
    dark_mode: bool,
}

impl QuadraticApp {
    fn parse_coeffs(&self) -> Result<(f64, f64, f64), String> {
        let parse = |s: &str| s.trim().parse::<f64>().map_err(|e| e.to_string());
        Ok((parse(&self.a_text)?, parse(&self.b_text)?, parse(&self.c_text)?))
    }

    fn format_complex(c: Complex<f64>) -> String {
        let re_near_zero = c.re.abs() < 1e-12;
        let im_near_zero = c.im.abs() < 1e-12;
        if im_near_zero {
            format!("{:.6}", c.re)
        } else if re_near_zero {
            format!("{:.6}i", c.im)
        } else {
            format!("{:.6} {:+.6}i", c.re, c.im)
        }
    }

    fn compute(&mut self) {
        self.error_message.clear();
        match self.parse_coeffs() {
            Ok((a, b, c)) => {
                let disc = b * b - 4.0 * a * c;
                self.discriminant = Some(disc);
                
                // Determine root type
                if disc > 1e-9 {
                    self.root_type = String::from("Two Real Distinct Roots");
                } else if disc.abs() < 1e-9 {
                    self.root_type = String::from("One Real Repeated Root");
                } else {
                    self.root_type = String::from("Two Complex Conjugate Roots");
                }
                
                match quadratic_roots(a, b, c) {
                    None => {
                        self.output = String::from("Degenerate equation: a == 0 and b == 0.");
                        self.root_type.clear();
                    }
                    Some((r1, r2)) => {
                        let s1 = QuadraticApp::format_complex(r1);
                        let s2 = QuadraticApp::format_complex(r2);
                        self.output = format!("x1 = {}\nx2 = {}", s1, s2);
                    }
                }
            }
            Err(e) => {
                self.discriminant = None;
                self.error_message = format!("⚠ Parse error: {}", e);
                self.output.clear();
                self.root_type.clear();
            }
        }
    }

    fn clear(&mut self) {
        self.a_text.clear();
        self.b_text.clear();
        self.c_text.clear();
        self.discriminant = None;
        self.output.clear();
        self.error_message.clear();
        self.root_type.clear();
    }

    fn load_example(&mut self, a: f64, b: f64, c: f64) {
        self.a_text = a.to_string();
        self.b_text = b.to_string();
        self.c_text = c.to_string();
        self.compute();
    }
}

impl eframe::App for QuadraticApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set theme based on dark_mode toggle
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("Quadratic Solver").size(24.0));
                ui.label("ax² + bx + c = 0");
                ui.add_space(5.0);
            });

            ui.separator();
            ui.add_space(10.0);

            // Input fields with Enter key support
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("a:").size(18.0));
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.a_text)
                        .font(egui::TextStyle::Button)
                        .desired_width(200.0)
                );
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.compute();
                }
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("b:").size(18.0));
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.b_text)
                        .font(egui::TextStyle::Button)
                        .desired_width(200.0)
                );
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.compute();
                }
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("c:").size(18.0));
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.c_text)
                        .font(egui::TextStyle::Button)
                        .desired_width(200.0)
                );
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.compute();
                }
            });

            ui.add_space(10.0);

            // Compute and Clear buttons
            ui.horizontal(|ui| {
                let compute_button = egui::Button::new(egui::RichText::new("Compute").size(18.0))
                    .min_size(egui::vec2(150.0, 40.0));
                if ui.add(compute_button).clicked() {
                    self.compute();
                }

                ui.add_space(10.0);

                let clear_button = egui::Button::new(egui::RichText::new("Clear").size(18.0))
                    .min_size(egui::vec2(150.0, 40.0));
                if ui.add(clear_button).clicked() {
                    self.clear();
                }
            });

            // Error message display
            if !self.error_message.is_empty() {
                ui.add_space(10.0);
                ui.label(egui::RichText::new(&self.error_message).size(16.0).color(egui::Color32::RED));
            }

            ui.separator();

            // Results display
            if !self.root_type.is_empty() {
                ui.label(egui::RichText::new(&self.root_type).size(16.0).color(egui::Color32::from_rgb(0, 150, 200)));
                ui.add_space(5.0);
            }

            if let Some(disc) = self.discriminant {
                let color = if disc >= 0.0 { egui::Color32::from_rgb(0, 180, 0) } else { egui::Color32::from_rgb(200, 100, 0) };
                ui.label(egui::RichText::new(format!("D = {:.6}", disc)).size(20.0).color(color));
            }
            
            if !self.output.is_empty() {
                ui.add_space(5.0);
                for line in self.output.lines() {
                    ui.label(egui::RichText::new(line).size(20.0));
                }
            }

            ui.separator();

            // Example equations
            ui.label(egui::RichText::new("Quick Examples:").size(14.0));
            ui.horizontal_wrapped(|ui| {
                if ui.button("x² - 5x + 6 = 0").clicked() {
                    self.load_example(1.0, -5.0, 6.0);
                }
                if ui.button("x² + 1 = 0").clicked() {
                    self.load_example(1.0, 0.0, 1.0);
                }
                if ui.button("x² - 4 = 0").clicked() {
                    self.load_example(1.0, 0.0, -4.0);
                }
                if ui.button("x² - 2x + 1 = 0").clicked() {
                    self.load_example(1.0, -2.0, 1.0);
                }
            });

            ui.add_space(10.0);
            ui.separator();

            // Theme toggle at bottom
            ui.horizontal(|ui| {
                ui.label("Theme:");
                if ui.button(if self.dark_mode { "🌙 Dark" } else { "☀ Light" }).clicked() {
                    self.dark_mode = !self.dark_mode;
                }
            });
        });
    }
}

fn main() {
    let app = QuadraticApp::default();
    let native_options = NativeOptions::default();
    let _ = eframe::run_native("Quadratic Solver", native_options, Box::new(|_cc| Box::new(app)));
}
