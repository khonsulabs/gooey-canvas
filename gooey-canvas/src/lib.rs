use std::fmt::Debug;

#[cfg(feature = "frontend-browser")]
use browser::BrowserRenderer;
use gooey::{
    core::{
        assets::Image,
        figures::{DisplayScale, Displayable, Point, Rect, Size},
        styles::{Color, SystemTheme},
        Context, KeyedStorage, Pixels, Scaled, StyledWidget, Widget,
    },
    frontends::rasterizer::ContentArea,
    renderer::{Renderer, StrokeOptions, TextOptions},
    widgets::component::{Behavior, ComponentBuilder, Content, ContentBuilder},
    App,
};

#[cfg(feature = "frontend-kludgine")]
mod kludgine;

#[cfg(feature = "frontend-kludgine")]
use gooey::frontends::renderers::kludgine::Kludgine;

#[cfg(feature = "frontend-browser")]
mod browser;

pub struct Canvas {
    renderable: Box<dyn Renderable>,
}

impl Debug for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Canvas").finish_non_exhaustive()
    }
}

impl Canvas {
    pub fn new<R: Renderable>(renderable: R) -> StyledWidget<Self> {
        StyledWidget::from(Self {
            renderable: Box::new(renderable),
        })
    }

    pub fn refresh(&self, context: &Context<Self>) {
        context.send_command(Command::Refresh);
    }
}

pub trait Renderable: Send + Sync + 'static {
    fn render(&mut self, renderer: CanvasRenderer, content_area: &ContentArea);

    #[allow(unused_variables)]
    fn mouse_down(&mut self, location: Point<f32, Scaled>, content_area: &ContentArea) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn mouse_up(&mut self, location: Option<Point<f32, Scaled>>, content_area: &ContentArea) {}
}

impl<F: FnMut(CanvasRenderer, &ContentArea) + Send + Sync + 'static> Renderable for F {
    fn render(&mut self, renderer: CanvasRenderer, content_area: &ContentArea) {
        self(renderer, content_area)
    }
}

#[derive(Debug)]
pub enum Event {}

#[derive(Debug)]
pub enum Command {
    Refresh,
}

impl Widget for Canvas {
    type Command = Command;
    type Event = ();

    const CLASS: &'static str = "gooey-canvas";
    const FOCUSABLE: bool = false;
}

#[derive(Debug)]
pub struct CanvasTransmogrifier;

pub trait AppExt {
    fn with_canvas(self) -> Self;
}

impl AppExt for App {
    fn with_canvas(self) -> Self {
        self.with(CanvasTransmogrifier)
    }
}

#[derive(Debug)]
pub enum CanvasRenderer {
    #[cfg(feature = "frontend-kludgine")]
    RasterizerRenderer(Kludgine),
    #[cfg(feature = "frontend-browser")]
    BrowserRenderer(BrowserRenderer),
}

impl Renderer for CanvasRenderer {
    fn theme(&self) -> SystemTheme {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.theme(),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.theme(),
        }
    }

    fn size(&self) -> Size<f32, Scaled> {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.size(),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.size(),
        }
    }

    fn clip_to(&self, bounds: Rect<f32, Scaled>) -> Self {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => {
                Self::RasterizerRenderer(renderer.clip_to(bounds))
            }
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => Self::BrowserRenderer(renderer.clip_to(bounds)),
        }
    }

    fn clip_bounds(&self) -> Rect<f32, Scaled> {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.clip_bounds(),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.clip_bounds(),
        }
    }

    fn scale(&self) -> DisplayScale<f32> {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.scale(),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.scale(),
        }
    }

    fn render_text(
        &self,
        text: &str,
        baseline_origin: impl Displayable<f32, Pixels = Point<f32, Pixels>>,
        options: &TextOptions,
    ) {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => {
                renderer.render_text(text, baseline_origin, options)
            }
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.render_text(text, baseline_origin, options),
        }
    }

    fn measure_text(
        &self,
        text: &str,
        options: &TextOptions,
    ) -> gooey::renderer::TextMetrics<Scaled> {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.measure_text(text, options),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.measure_text(text, options),
        }
    }

    fn stroke_rect(
        &self,
        rect: &impl Displayable<f32, Pixels = Rect<f32, Pixels>>,
        options: &StrokeOptions,
    ) {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.stroke_rect(rect, options),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.stroke_rect(rect, options),
        }
    }

    fn fill_rect(&self, rect: &impl Displayable<f32, Pixels = Rect<f32, Pixels>>, color: Color) {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.fill_rect(rect, color),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.fill_rect(rect, color),
        }
    }

    fn stroke_line<P: Displayable<f32, Pixels = Point<f32, Pixels>>>(
        &self,
        point_a: P,
        point_b: P,
        options: &StrokeOptions,
    ) {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.stroke_line(point_a, point_b, options),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.stroke_line(point_a, point_b, options),
        }
    }

    fn draw_image(
        &self,
        image: &Image,
        location: impl Displayable<f32, Pixels = Point<f32, Pixels>>,
    ) {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.draw_image(image, location),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.draw_image(image, location),
        }
    }
}

impl<B: Behavior<Widgets = ()>> Content<B> for Canvas {
    type Builder = Builder<ComponentBuilder<B>>;
}

#[derive(Debug)]
pub struct Builder<S: KeyedStorage<()>> {
    storage: S,
    canvas: Option<Canvas>,
}

impl<S: KeyedStorage<()>> Builder<S> {
    pub fn on_render<R: Renderable>(mut self, renderable: R) -> Self {
        self.canvas = Some(Canvas {
            renderable: Box::new(renderable),
        });
        self
    }

    pub fn finish(self) -> StyledWidget<Canvas> {
        StyledWidget::from(self.canvas.unwrap())
    }
}

impl<S: KeyedStorage<()>> ContentBuilder<(), S> for Builder<S> {
    fn storage(&self) -> &gooey::core::WidgetStorage {
        self.storage.storage()
    }

    fn related_storage(&self) -> Option<Box<dyn gooey::core::RelatedStorage<()>>> {
        self.storage.related_storage()
    }

    fn new(storage: S) -> Self {
        Builder {
            storage,
            canvas: None,
        }
    }
}
