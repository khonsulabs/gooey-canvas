use gooey::{
    core::styles::{BackgroundColor, Color, Style},
    renderer::Renderer,
    widgets::component::{Behavior, Component},
};
use gooey_canvas::{AppExt, Canvas, CanvasRenderer};

fn main() {
    gooey::App::from_root(|storage| Component::new(Basic::default(), storage))
        .with_canvas()
        .with_component::<Basic>()
        .run()
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
                renderer.fill_rect::<BackgroundColor>(
                    &renderer.bounds().inflate(-64., -64.),
                    &Style::default().with(BackgroundColor(Color::new(1., 0., 0., 1.).into())),
                );
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
