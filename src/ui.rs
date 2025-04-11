use eframe::egui;
use crate::Poem;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

// Define the display mode enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DisplayMode {
    Cleartext,
    SemiObfuscated,
    FullyObfuscated,
}

// Define the application state
pub struct ExquisiteVerse {
    poem: Poem,
    new_line: String,
    display_mode: DisplayMode,
    import_mode: bool,
    import_text: String,
    copy_feedback_timer: Option<f32>,
}

impl ExquisiteVerse {
    pub fn new() -> Self {
        Self {
            poem: Poem::new(""),
            new_line: String::new(),
            display_mode: DisplayMode::SemiObfuscated,
            import_mode: false,
            import_text: String::new(),
            copy_feedback_timer: None,
        }
    }

    fn try_import_poem(&mut self) -> Result<(), String> {
        if self.import_text.is_empty() {
            return Ok(());
        }

        match self.display_mode {
            DisplayMode::Cleartext => {
                self.poem = Poem::new(&self.import_text);
                Ok(())
            }
            DisplayMode::SemiObfuscated => {
                match Poem::from_semi_obfuscated(&self.import_text) {
                    Ok(poem) => {
                        self.poem = poem;
                        Ok(())
                    }
                    Err(e) => Err(format!("Failed to import semi-obfuscated poem: {:?}", e))
                }
            }
            DisplayMode::FullyObfuscated => {
                match Poem::from_fully_obfuscated(&self.import_text) {
                    Ok(poem) => {
                        self.poem = poem;
                        Ok(())
                    }
                    Err(e) => Err(format!("Failed to import fully-obfuscated poem: {:?}", e))
                }
            }
        }
    }
}

impl eframe::App for ExquisiteVerse {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update copy feedback timer if it exists
        if let Some(timer) = self.copy_feedback_timer.as_mut() {
            *timer -= ctx.input(|i| i.unstable_dt);
            if *timer <= 0.0 {
                self.copy_feedback_timer = None;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Exquisite Verse");
            
            // Import mode toggle and controls
            ui.horizontal(|ui| {
                if ui.button(if self.import_mode { "Cancel Import" } else { "Import Poem" }).clicked() {
                    if self.import_mode {
                        self.import_mode = false;
                        self.import_text.clear();
                    } else {
                        self.import_mode = true;
                        // Initialize import text with current poem in selected format
                        self.import_text = match self.display_mode {
                            DisplayMode::Cleartext => self.poem.as_cleartext(),
                            DisplayMode::SemiObfuscated => self.poem.as_semi_obfuscated(),
                            DisplayMode::FullyObfuscated => self.poem.as_fully_obfuscated(),
                        };
                    }
                }

                if self.import_mode {
                    if ui.button("Import").clicked() {
                        if let Err(e) = self.try_import_poem() {
                            // Show error message
                            ui.label(format!("Error: {}", e));
                        } else {
                            self.import_mode = false;
                            self.import_text.clear();
                        }
                    }
                }
            });

            ui.add_space(10.0);
            
            // Display mode selection
            ui.horizontal(|ui| {
                ui.label("Display/Import Mode:");
                ui.radio_value(&mut self.display_mode, DisplayMode::Cleartext, "Cleartext");
                ui.radio_value(&mut self.display_mode, DisplayMode::SemiObfuscated, "Semi-Obfuscated");
                ui.radio_value(&mut self.display_mode, DisplayMode::FullyObfuscated, "Fully-Obfuscated");
            });
            
            ui.add_space(10.0);

            // Create a scrollable area for the poem
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.import_mode {
                    // Show editable text area for importing
                    ui.add(egui::TextEdit::multiline(&mut self.import_text)
                        .desired_width(f32::INFINITY)
                        .desired_rows(10)
                        .hint_text("Paste your poem here..."));
                } else {
                    // Display the poem based on the current display mode
                    let poem_text = match self.display_mode {
                        DisplayMode::Cleartext => self.poem.as_cleartext(),
                        DisplayMode::SemiObfuscated => self.poem.as_semi_obfuscated(),
                        DisplayMode::FullyObfuscated => self.poem.as_fully_obfuscated(),
                    };
                    
                    // Use a label for displaying the poem
                    ui.label(poem_text.clone());

                    // Add copy to clipboard button
                    ui.horizontal(|ui| {
                        if ui.button("📋 Copy to Clipboard").clicked() {
                            ui.ctx().copy_text(poem_text.clone());
                            self.copy_feedback_timer = Some(1.0); // Show feedback for 1 second
                        }
                        
                        // Show copy feedback
                        if self.copy_feedback_timer.is_some() {
                            ui.label("✓ Copied!");
                        }
                    });
                }
            });
            
            if !self.import_mode {
                ui.add_space(10.0);
                
                // Input for new lines
                ui.horizontal(|ui| {
                    let text_edit = egui::TextEdit::singleline(&mut self.new_line)
                        .hint_text("Enter a new line...")
                        .desired_width(300.0);
                    
                    let text_edit_response = ui.add(text_edit);
                    
                    // Handle Enter key press
                    if text_edit_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if !self.new_line.is_empty() {
                            self.poem.add_line(&self.new_line);
                            self.new_line = String::new();
                        }
                    }
                    
                    if ui.button("Add Line").clicked() {
                        if !self.new_line.is_empty() {
                            self.poem.add_line(&self.new_line);
                            self.new_line = String::new();
                        }
                    }
                    
                    if ui.button("Clear Poem").clicked() {
                        self.poem = Poem::new("");
                    }
                });
            }
        });
    }
}

// Add a function to run the application
#[cfg(not(target_arch = "wasm32"))]
pub fn run() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Exquisite Verse",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(ExquisiteVerse::new()))),
    )
}

#[cfg(target_arch = "wasm32")]
pub async fn run() -> Result<(), eframe::Error> {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("the_canvas_id")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    eframe::web::WebRunner::new()
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(|_cc| Ok(Box::new(ExquisiteVerse::new()))),
        )
        .await
        .expect("failed to start eframe");
    Ok(())
} 