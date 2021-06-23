use dit;
use dit::app;

fn main() {
    let app_m = app::get_app().get_matches();
    app::handle_matches(app_m);
}
