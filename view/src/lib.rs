use druid::widget::*;
use druid::{Widget, WidgetExt};

#[no_mangle]
pub fn testing() -> Box<dyn Widget<shared::AppState>> {
    Flex::column()
        .with_child(
            TextBox::new()
                .with_placeholder("Hello just added a new text box!!")
                .lens(shared::AppState::text)
                .padding(30.0),
        )
        .with_child(Label::new("Helllo World"))
        .with_child(Checkbox::new("I can hot reload.").lens(shared::AppState::checked))
        .boxed()
}
