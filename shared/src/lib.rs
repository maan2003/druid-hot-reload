#[derive(druid::Lens, druid::Data, Clone, Default)]
#[repr(C)]
pub struct AppState {
    text: String,
    checked: bool,
    float: f64,
}
