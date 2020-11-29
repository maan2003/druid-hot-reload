extern crate druid;
extern crate hot_reload_lib;
extern crate shared;

use std::any::Any;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use druid::widget::{Label, Padding};
use druid::{
    AppLauncher, Event, ExtEventSink, Lens, LifeCycle, LifeCycleCtx, Selector, Target, Widget,
    WidgetExt, WidgetPod, WindowDesc,
};
use hot_reload_lib::HotReloadLib;
use shared::AppState;

const RELOAD: Selector<()> = Selector::new("druid-hot-reload.reload");
fn main() {
    // describe the main window
    let sink = Arc::new(Mutex::new(None::<ExtEventSink>));
    let main_window = WindowDesc::new({
        let sink = sink.clone();
        move || {
            let lib = HotReloadLib::new("./target/debug", "view", move || {
                let sink = sink.lock().unwrap();
                let sink = sink.as_ref().unwrap();
                sink.submit_command(RELOAD, (), Target::Global).unwrap();
            });
            HotReloaderWidget { lib, inner: None }
        }
    });
    let launcher = AppLauncher::with_window(main_window);
    *sink.lock().unwrap() = Some(launcher.get_external_handle());

    launcher
        .launch(AppState::default())
        .expect("Failed to launch application");
}

struct HotReloaderWidget {
    lib: HotReloadLib,
    inner: Option<WidgetPod<AppState, Box<dyn Widget<AppState>>>>,
}

impl HotReloaderWidget {
    fn update_lib(&mut self) {
        // droping it before unloading the library
        drop(self.inner.take());
        self.lib.update();
        let load = self
            .lib
            .load_symbol::<fn() -> Box<dyn Widget<AppState>>>("testing");
        let returned_widget = load();
        self.inner = Some(WidgetPod::new(returned_widget));
    }
}

impl Widget<AppState> for HotReloaderWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &druid::Env,
    ) {
        if let Event::Command(cmd) = event {
            if cmd.is(RELOAD) {
                self.update_lib();
                ctx.children_changed();
                return;
            }
        }
        // event cause panic about inner receiving event without being laid out
        // self.inner.as_mut().unwrap().event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &LifeCycle,
        data: &AppState,
        env: &druid::Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            // just update library first time
            if self.inner.is_none() {
                self.update_lib();
                ctx.children_changed();
                ctx.request_layout();
            }
        }
        self.inner
            .as_mut()
            .unwrap()
            .lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &druid::Env,
    ) {
        self.inner.as_mut().unwrap().update(ctx, data, env)
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &AppState,
        env: &druid::Env,
    ) -> druid::Size {
        self.inner.as_mut().unwrap().layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &druid::Env) {
        self.inner.as_mut().unwrap().paint(ctx, data, env)
    }
}
