use druid::widget::{Label, Flex, Button};
use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, Lens};
use druid::WidgetExt;  // This is needed for padding and center
use druid::piet::Color; // For setting the text color

#[derive(Clone, Data, Lens)]
struct AppState {
    categories_shown: bool,
}

fn main() -> Result<(), PlatformError> {
    // Create the main window description
    let main_window = WindowDesc::new(build_ui)
        .window_size((400.0, 300.0)) // Set the initial window size
        .title("Secure Trivia Game");

    // Launch the application with the main window and initial state
    AppLauncher::with_window(main_window)
        .launch(AppState {
            categories_shown: false,
        })
}

fn build_ui() -> impl Widget<AppState> {
    let title = Label::new("Secure Trivia Game") // Title
        .with_text_size(32.0)
        .with_text_color(Color::from_hex_str("#3EB3D0").unwrap()) // Color
        .padding((0.0, 30.0, 0.0, 0.0))
        .center();

    let play_button = Button::new("Play") // Play Button
        .on_click(|_ctx, data: &mut AppState, _env| {
            data.categories_shown = true;
        })
        .padding(20.0)
        .fix_width(200.0)
        .center();

    let categories = Flex::column() // Category Labels
        .with_child(Label::new("Geography").padding(5.0))
        .with_child(Label::new("History").padding(5.0))
        .with_child(Label::new("Science").padding(5.0))
        .with_child(Label::new("Sports").padding(5.0))
        .with_child(Label::new("Mathematics").padding(5.0))
        .center();

    // Conditional layout
    let dynamic_content = druid::widget::Either::new(
        |data: &AppState, _env| data.categories_shown,
        categories,    // Show categories when true
        play_button,   // Show play button when false
    );

    // Final layout
    Flex::column()
        .with_child(title)
        .with_spacer(30.0)
        .with_flex_child(dynamic_content, 1.0)
        .center()
}