use gooey::{
    core::{
        assets::{Asset, Image},
        euclid::Point2D,
        styles::Color,
        Callback, Context,
    },
    renderer::Renderer,
    widgets::component::{Behavior, Component, ComponentCommand},
    App,
};
use gooey_canvas::{AppExt, Canvas, CanvasRenderer, Command};

#[cfg(all(test, not(target_arch = "wasm32")))]
mod harness;

fn app() -> App {
    App::from_root(|storage| {
        Component::new(
            Basic {
                image: Image::from(Asset::build().path(vec!["rolls.jpg"]).finish()),
            },
            storage,
        )
    })
    .with_canvas()
    .with_component::<Basic>()
}

fn main() {
    app().run()
}

#[derive(Debug)]
struct Basic {
    image: Image,
}

impl Behavior for Basic {
    type Content = Canvas;
    type Event = ();
    type Widgets = ();

    fn initialize(component: &mut Component<Self>, context: &Context<Component<Self>>) {
        let callback_context = context.clone();
        component.behavior.image.load(
            component.map_event(move |_| {
                callback_context.send_command(ComponentCommand::Widget(Command::Refresh))
            }),
            Callback::new(|err| panic!("error loading asset: {}", err)),
            context.frontend.as_ref(),
        )
    }

    fn build_content(
        &mut self,
        builder: <Self::Content as gooey::widgets::component::Content<Self>>::Builder,
        _events: &gooey::widgets::component::EventMapper<Self>,
    ) -> gooey::core::StyledWidget<Self::Content> {
        let image = self.image.clone();
        builder
            .on_render(move |renderer: CanvasRenderer| {
                renderer.fill_rect(&renderer.bounds().inflate(-64., -64.), Color::RED);
                renderer.draw_image(&image, Point2D::new(128., 128.));
            })
            .finish()
    }

    fn receive_event(
        _component: &mut gooey::widgets::component::Component<Self>,
        _event: Self::Event,
        _context: &gooey::core::Context<gooey::widgets::component::Component<Self>>,
    ) {
    }

    fn classes() -> Option<gooey::core::styles::style_sheet::Classes> {
        None
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
