use eframe::App;

use crate::utils;

// This struct holds the state of our calculator.
#[derive(Default)]
pub(crate) struct CalculatorApp {
    expression: String,
    result: String,
    pressed_button: Option<String>,
}

// --- Application UI Logic (The `App` trait) ---
// This is the core of the egui application.
impl App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Requirement 4: "Liquid Glass" Look (Dark Theme) ---
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_unmultiplied(40, 40, 50, 180);
        visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
        visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;

        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(60, 60, 75, 200);
        visuals.widgets.hovered.rounding = egui::Rounding::same(5.0);
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(100, 100, 120, 150),
        );

        visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(80, 80, 100, 255);
        visuals.widgets.active.rounding = egui::Rounding::same(5.0);
        visuals.widgets.active.bg_stroke = egui::Stroke::NONE;

        visuals.panel_fill = egui::Color32::from_rgba_unmultiplied(20, 20, 25, 230);

        ctx.set_visuals(visuals);

        // --- Requirement 3: Padding ---
        let panel_frame = egui::Frame {
            inner_margin: egui::Margin::same(1.0),
            fill: ctx.style().visuals.panel_fill,
            ..egui::Frame::default()
        };

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(1.0, 1.0);

                // --- 1. DISPLAY AREA ---
                egui::Frame::default()
                    .inner_margin(egui::Margin::same(5.0))
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.expression)
                                    .font(egui::FontId::proportional(28.0))
                                    .frame(false)
                                    .horizontal_align(egui::Align::Max)
                                    .desired_width(ui.available_width())
                                    .char_limit(100),
                            );

                            // --- Requirement 2: Constant input listening ---
                            response.request_focus();

                            // --- Requirement 4 & 6: Filter input to only allow valid characters ---
                            self.expression.retain(|c| {
                                matches!(
                                    c,
                                    '0'..='9' | '.' | '+' | '-' | '*' | '/' | '^' | '(' | ')' | '%'
                                )
                            });

                            let result_text = if self.result.is_empty() {
                                " "
                            } else {
                                &self.result
                            };
                            ui.label(
                                egui::RichText::new(result_text)
                                    .font(egui::FontId::proportional(20.0))
                                    .color(egui::Color32::LIGHT_GRAY),
                            );
                        });
                    });

                ui.separator();

                // --- 2. BUTTONS ---
                let remaining_height = ui.available_height();
                let remaining_width = ui.available_width();

                let button_height = (remaining_height - 4.0 * 1.0) / 5.0;
                let button_width = (remaining_width - 3.0 * 1.0) / 4.0;
                let button_size = egui::vec2(button_width, button_height);

                let mut add_button = |ui: &mut egui::Ui, text: &str| {
                    let is_pressed = self.pressed_button.as_deref() == Some(text);
                    let mut button = egui::Button::new(
                        egui::RichText::new(text).font(egui::FontId::proportional(20.0)),
                    );

                    // If this button was just pressed via keyboard, show it as active
                    if is_pressed {
                        button =
                            button.fill(egui::Color32::from_rgba_unmultiplied(80, 80, 100, 255));
                    }

                    let response = ui.add_sized(button_size, button);
                    if response.clicked() {
                        self.handle_button_click(text);
                    }
                };

                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        add_button(ui, "C");
                        add_button(ui, "(");
                        add_button(ui, ")");
                        add_button(ui, "/");
                    });
                    ui.horizontal(|ui| {
                        add_button(ui, "7");
                        add_button(ui, "8");
                        add_button(ui, "9");
                        add_button(ui, "*");
                    });
                    ui.horizontal(|ui| {
                        add_button(ui, "4");
                        add_button(ui, "5");
                        add_button(ui, "6");
                        add_button(ui, "-");
                    });
                    ui.horizontal(|ui| {
                        add_button(ui, "1");
                        add_button(ui, "2");
                        add_button(ui, "3");
                        add_button(ui, "+");
                    });
                    ui.horizontal(|ui| {
                        add_button(ui, "0");
                        add_button(ui, ".");
                        add_button(ui, "^");
                        add_button(ui, "=");
                    });
                });
            });

        // --- Requirement 2, 5, 7: Keyboard Input ---
        ctx.input(|i| {
            self.handle_key_presses(i);
        });

        // Clear the pressed button after one frame
        self.pressed_button = None;
    }
}

// --- Application Helper Functions ---
impl CalculatorApp {
    /// Handles clicks from the on-screen buttons
    fn handle_button_click(&mut self, text: &str) {
        match text {
            "C" => {
                self.expression.clear();
                self.result.clear();
            }
            "=" => {
                // --- Requirement 6: Use handle_equation ---
                // We must catch panics, as the new function panics on errors
                // like division by zero.
                if self.expression.is_empty() {
                    self.result.clear();
                    return;
                }

                let expr_clone = self.expression.clone();
                let calculation_result = std::panic::catch_unwind(|| {
                    utils::equation_handler::handle_equation(expr_clone)
                });

                match calculation_result {
                    Ok(val) => {
                        // Check for infinity or NaN which can still happen
                        if val.is_infinite() {
                            self.result = "Infinity".to_string();
                        } else if val.is_nan() {
                            self.result = "NaN".to_string();
                        } else {
                            // Format to avoid trailing .0
                            if val.fract() == 0.0 {
                                self.result = (val as i64).to_string();
                            } else {
                                self.result = val.to_string();
                            }
                        }
                    }
                    Err(_) => {
                        // A panic occurred, likely division by zero or malformed expression
                        self.result = "Error".to_string();
                    }
                }
            }
            _ => {
                self.expression.push_str(text);
            }
        }
    }

    /// Handles keyboard input events
    fn handle_key_presses(&mut self, i: &egui::InputState) {
        for event in &i.events {
            match event {
                // Handle special *keys* (non-text)
                egui::Event::Key {
                    key, pressed: true, ..
                } => match key {
                    egui::Key::Enter => {
                        self.pressed_button = Some("=".to_string());
                        self.handle_button_click("=");
                    }
                    egui::Key::Escape => {
                        self.pressed_button = Some("C".to_string());
                        self.handle_button_click("C");
                    }
                    _ => {}
                },
                // Track text input for button highlighting
                egui::Event::Text(text) => {
                    for ch in text.chars() {
                        let button_text = match ch {
                            '0'..='9' => ch.to_string(),
                            '.' => ".".to_string(),
                            '+' => "+".to_string(),
                            '-' => "-".to_string(),
                            '*' => "*".to_string(),
                            '/' => "/".to_string(),
                            '^' => "^".to_string(),
                            '(' => "(".to_string(),
                            ')' => ")".to_string(),
                            '%' => "%".to_string(),
                            _ => continue,
                        };
                        self.pressed_button = Some(button_text);
                    }
                }
                _ => {}
            }
        }
    }
}
