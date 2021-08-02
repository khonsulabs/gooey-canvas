use gooey::{
    core::styles::Color,
    renderer::Renderer,
    widgets::component::{Behavior, Component},
    App,
};
use gooey_canvas::{AppExt, Canvas, CanvasRenderer};

#[cfg(all(test, not(target_arch = "wasm32")))]
mod harness;

fn app() -> App {
    App::from_root(|storage| Component::new(Basic::default(), storage))
        .with_canvas()
        .with_component::<Basic>()
}

fn main() {
    app().run()
}

#[derive(Debug, Default)]
struct Basic {}

impl Behavior for Basic {
    type Content = Canvas;
    type Event = ();
    type Widgets = ();

    fn build_content(
        &mut self,
        builder: <Self::Content as gooey::widgets::component::Content<Self>>::Builder,
        _events: &gooey::widgets::component::EventMapper<Self>,
    ) -> gooey::core::StyledWidget<Self::Content> {
        builder
            .on_render(|renderer: CanvasRenderer| {
                renderer.fill_rect(&renderer.bounds().inflate(-64., -64.), Color::RED);
            })
            .finish()
    }

    fn receive_event(
        _component: &mut gooey::widgets::component::Component<Self>,
        _event: Self::Event,
        _context: &gooey::core::Context<gooey::widgets::component::Component<Self>>,
    ) {
        unimplemented!()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {

    use gooey::core::{euclid::Size2D, styles::SystemTheme};

    use super::*;

    #[tokio::test]
    async fn demo() -> anyhow::Result<()> {
        for theme in [SystemTheme::Dark, SystemTheme::Light] {
            let headless = app().headless();
            let snapshot = headless
                .screenshot(Size2D::new(320, 240), theme, None)
                .await?
                .to_rgb8();

            assert_ne!(snapshot.get_pixel(160, 120), snapshot.get_pixel(1, 1));
            assert_eq!(snapshot.get_pixel(160, 120).0, [255_u8, 0, 0]);

            snapshot.save(harness::snapshot_path(
                "basic",
                &format!("Demo-{:?}.png", theme),
            )?)?;
        }
        Ok(())
    }
}
