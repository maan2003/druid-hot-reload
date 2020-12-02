use druid::widget::*;
use druid::{Widget, WidgetExt};
use shared::AppState;

#[no_mangle]
pub fn testing() -> Box<dyn Widget<AppState>> {
    Flex::column()
        .with_child(
            Label::new("hello World")
                .with_text_size(36.0)
                .lens(AppState::text),
        )
        .with_child(
            Flex::row()
                .with_child(Checkbox::new("Helllo world").padding(10.0))
                .lens(AppState::checked),
        )
        .with_child(Slider::new().lens(AppState::float))
        .with_child(
            Checkbox::new("New one")
                .lens(AppState::checked)
                .padding(20.0),
        )
        .boxed()
}
