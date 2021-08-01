use std::boxed::Box;

use gooey::{
    core::{euclid::Size2D, Points, Transmogrifier, TransmogrifierContext},
    frontends::{
        rasterizer::{Rasterizer, RegisteredTransmogrifier, Renderer, WidgetRasterizer},
        renderers::kludgine::Kludgine,
    },
};

use crate::{Canvas, CanvasRenderer, CanvasTransmogrifier, Command};

impl Transmogrifier<Rasterizer<Kludgine>> for CanvasTransmogrifier {
    type State = ();
    type Widget = Canvas;

    fn receive_command(
        &self,
        command: Command,
        context: &mut TransmogrifierContext<'_, Self, Rasterizer<Kludgine>>,
    ) {
        let Command::Refresh = command;
        context.frontend.set_needs_redraw();
    }
}

impl WidgetRasterizer<Kludgine> for CanvasTransmogrifier {
    fn render(&self, context: TransmogrifierContext<'_, Self, Rasterizer<Kludgine>>) {
        if let Some(scene) = context.frontend.renderer() {
            context
                .widget
                .renderable
                .render(CanvasRenderer::RasterizerRenderer(scene.clone()));
        }
    }

    fn content_size(
        &self,
        context: TransmogrifierContext<'_, Self, Rasterizer<Kludgine>>,
        constraints: Size2D<Option<f32>, Points>,
    ) -> Size2D<f32, Points> {
        let size = context
            .frontend
            .renderer()
            .map_or_else(Size2D::default, |scene| scene.size());
        Size2D::new(
            constraints.width.unwrap_or(size.width),
            constraints.height.unwrap_or(size.height),
        )
    }
}

impl From<CanvasTransmogrifier> for RegisteredTransmogrifier<Kludgine> {
    fn from(transmogrifier: CanvasTransmogrifier) -> Self {
        Self(Box::new(transmogrifier))
    }
}
