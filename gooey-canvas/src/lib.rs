use std::fmt::Debug;

use browser::BrowserRenderer;
use gooey::{
    core::{
        styles::{Color, SystemTheme},
        KeyedStorage, StyledWidget, Widget,
    },
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
}

pub trait Renderable: Send + Sync + 'static {
    fn render(&mut self, renderer: CanvasRenderer);
}

impl<F: FnMut(CanvasRenderer) + Send + Sync + 'static> Renderable for F {
    fn render(&mut self, renderer: CanvasRenderer) {
        self(renderer)
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

    fn size(&self) -> gooey::core::euclid::Size2D<f32, gooey::core::Points> {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.size(),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.size(),
        }
    }

    fn clip_to(&self, bounds: gooey::core::euclid::Rect<f32, gooey::core::Points>) -> Self {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => {
                Self::RasterizerRenderer(renderer.clip_to(bounds))
            }
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => Self::BrowserRenderer(renderer.clip_to(bounds)),
        }
    }

    fn clip_bounds(&self) -> gooey::core::euclid::Rect<f32, gooey::core::Points> {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.clip_bounds(),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.clip_bounds(),
        }
    }

    fn scale(&self) -> gooey::core::euclid::Scale<f32, gooey::core::Points, gooey::core::Pixels> {
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
        baseline_origin: gooey::core::euclid::Point2D<f32, gooey::core::Points>,
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
    ) -> gooey::renderer::TextMetrics<gooey::core::Points> {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.measure_text(text, options),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.measure_text(text, options),
        }
    }

    fn stroke_rect(
        &self,
        rect: &gooey::core::euclid::Rect<f32, gooey::core::Points>,
        options: &StrokeOptions,
    ) {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.stroke_rect(rect, options),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.stroke_rect(rect, options),
        }
    }

    fn fill_rect(&self, rect: &gooey::core::euclid::Rect<f32, gooey::core::Points>, color: Color) {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.fill_rect(rect, color),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.fill_rect(rect, color),
        }
    }

    fn stroke_line(
        &self,
        point_a: gooey::core::euclid::Point2D<f32, gooey::core::Points>,
        point_b: gooey::core::euclid::Point2D<f32, gooey::core::Points>,
        options: &StrokeOptions,
    ) {
        match self {
            #[cfg(feature = "frontend-kludgine")]
            Self::RasterizerRenderer(renderer) => renderer.stroke_line(point_a, point_b, options),
            #[cfg(feature = "frontend-browser")]
            Self::BrowserRenderer(renderer) => renderer.stroke_line(point_a, point_b, options),
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
