use std::boxed::Box;

use gooey::{
    core::{
        figures::{Point, Size},
        Scaled, Transmogrifier, TransmogrifierContext,
    },
    frontends::{
        rasterizer::{
            winit::event::MouseButton, ContentArea, EventStatus, Rasterizer,
            RegisteredTransmogrifier, Renderer, WidgetRasterizer,
        },
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
    fn render(
        &self,
        context: &mut TransmogrifierContext<'_, Self, Rasterizer<Kludgine>>,
        content_area: &ContentArea,
    ) {
        if let Some(scene) = context.frontend.renderer() {
            context.widget.renderable.render(
                CanvasRenderer::RasterizerRenderer(scene.clone()),
                content_area,
            );
        }
    }

    fn measure_content(
        &self,
        context: &mut TransmogrifierContext<'_, Self, Rasterizer<Kludgine>>,
        constraints: Size<Option<f32>, Scaled>,
    ) -> Size<f32, Scaled> {
        let size = context
            .frontend
            .renderer()
            .map_or_else(Size::default, |scene| scene.size());
        Size::new(
            constraints.width.unwrap_or(size.width),
            constraints.height.unwrap_or(size.height),
        )
    }

    fn mouse_down(
        &self,
        context: &mut TransmogrifierContext<'_, Self, Rasterizer<Kludgine>>,
        _button: MouseButton,
        location: Point<f32, Scaled>,
        area: &ContentArea,
    ) -> EventStatus {
        if context.widget.renderable.mouse_down(location, area) {
            EventStatus::Processed
        } else {
            EventStatus::Ignored
        }
    }

    // fn mouse_drag(
    //     &self,
    //     context: &mut TransmogrifierContext<'_, Self, Rasterizer<Kludgine>>,
    //     _button: MouseButton,
    //     location: Point<f32, Scaled>,
    //     area: &ContentArea,
    // ) {
    //     context.widget.renderable.mouse_drag(location, area)
    // }

    fn mouse_up(
        &self,
        context: &mut TransmogrifierContext<'_, Self, Rasterizer<Kludgine>>,
        _button: MouseButton,
        location: Option<Point<f32, Scaled>>,
        area: &ContentArea,
    ) {
        context.widget.renderable.mouse_up(location, area)
    }
}

impl From<CanvasTransmogrifier> for RegisteredTransmogrifier<Kludgine> {
    fn from(transmogrifier: CanvasTransmogrifier) -> Self {
        Self(Box::new(transmogrifier))
    }
}
