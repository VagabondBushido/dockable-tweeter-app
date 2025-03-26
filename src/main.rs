use eframe::{egui, App, CreationContext};
use webbrowser;

//for holding the twitter username (app state)
struct TwitterApp {
    twitter_username: String,
}

//cosntructor for the app state (default twitter username)
impl Default for TwitterApp {
    fn default() -> Self {
        Self {
            twitter_username: "".to_string(),
        }
    }
}
//
impl App for TwitterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Twitter Dock");
            
            ui.horizontal(|ui| {
                ui.label("Username: ");
                ui.text_edit_singleline(&mut self.twitter_username);
            });
            
            if ui.button("Open Twitter Profile").clicked() {
                let url = format!("https://twitter.com/{}", self.twitter_username);
                if let Err(e) = webbrowser::open(&url) {
                    eprintln!("Failed to open URL: {}", e);
                }
            }
        });
    }
}

//main function for running the app
fn main() {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_always_on_top()
            .with_decorations(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Twitter Dock App",
        options,
        Box::new(|_cc: &CreationContext| Box::new(TwitterApp::default())),
    ).unwrap();
}
