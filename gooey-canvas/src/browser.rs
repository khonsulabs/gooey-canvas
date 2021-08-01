use gooey::{
    core::{
        euclid::{Length, Point2D, Rect, Scale, Size2D},
        styles::{ColorPair, ForegroundColor, Style, SystemTheme},
        Context, Points, TransmogrifierContext, WidgetId,
    },
    frontends::browser::{
        utils::{create_element, widget_css_id, window_document, CssRules},
        RegisteredTransmogrifier, WebSys, WebSysTransmogrifier,
    },
    renderer::{Renderer, TextMetrics},
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{Canvas, CanvasRenderer, CanvasTransmogrifier, Command};

fn canvas_element(widget_id: &WidgetId) -> Option<HtmlCanvasElement> {
    window_document()
        .get_element_by_id(&widget_css_id(widget_id.id))
        .and_then(|e| e.dyn_into::<HtmlCanvasElement>().ok())
}

impl BrowserRenderer {
    fn canvas(&self) -> Option<HtmlCanvasElement> {
        canvas_element(&self.widget)
    }

    fn rendering_context(&self) -> Option<CanvasRenderingContext2d> {
        self.canvas()?
            .get_context("2d")
            .ok()?
            .and_then(|c| c.dyn_into().ok())
    }

    fn clip(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.rect(
            self.clip.origin.x,
            self.clip.origin.y,
            self.clip.size.width,
            self.clip.size.height,
        );
        context.clip();
        context
            .translate(self.clip.origin.x, self.clip.origin.y)
            .unwrap();
    }
}

impl CanvasTransmogrifier {
    fn redraw(&self, context: &mut TransmogrifierContext<'_, CanvasTransmogrifier, WebSys>) {
        let context = Context::new(context.channels, context.frontend);
        let cb = Closure::once_into_js(move || {
            context.map_mut(|canvas, context| {
                let widget = context.widget().registration().unwrap().id().clone();
                if let Some(canvas_element) = canvas_element(&widget) {
                    let size = Size2D::new(
                        canvas_element.client_width(),
                        canvas_element.client_height(),
                    )
                    .max(Size2D::default())
                    .to_u32();
                    canvas_element.set_width(size.width);
                    canvas_element.set_height(size.height);

                    let renderer = BrowserRenderer {
                        widget,
                        clip: Rect::from_size(size.to_f64()),
                        theme: context.frontend.theme(),
                    };
                    canvas
                        .renderable
                        .render(CanvasRenderer::BrowserRenderer(renderer));
                }
            });
        });
        web_sys::window()
            .unwrap()
            .request_animation_frame(cb.dyn_ref().unwrap())
            .unwrap();
    }
}

impl gooey::core::Transmogrifier<WebSys> for CanvasTransmogrifier {
    type State = Option<CssRules>;
    type Widget = Canvas;

    fn receive_command(
        &self,
        command: Command,
        context: &mut TransmogrifierContext<'_, Self, WebSys>,
    ) {
        let Command::Refresh = command;
        self.redraw(context);
    }
}

impl WebSysTransmogrifier for CanvasTransmogrifier {
    fn transmogrify(
        &self,
        mut context: TransmogrifierContext<'_, Self, WebSys>,
    ) -> Option<web_sys::HtmlElement> {
        let element = create_element::<HtmlCanvasElement>("canvas");
        let css = self
            .initialize_widget_element(&element, &context)
            .unwrap_or_default();

        *context.state = Some(css);

        self.redraw(&mut context);

        Some(element.unchecked_into())
    }
}

#[derive(Debug)]
pub struct BrowserRenderer {
    widget: WidgetId,
    clip: Rect<f64, Points>,
    theme: SystemTheme,
}

impl Renderer for BrowserRenderer {
    fn size(&self) -> gooey::core::euclid::Size2D<f32, Points> {
        self.clip.size.to_f32()
    }

    fn clip_to(&self, bounds: Rect<f32, Points>) -> Self {
        Self {
            widget: self.widget.clone(),
            clip: self.clip.intersection(&bounds.to_f64()).unwrap_or_default(),
            theme: self.theme,
        }
    }

    fn clip_bounds(&self) -> Rect<f32, Points> {
        self.clip.to_f32()
    }

    fn scale(&self) -> gooey::core::euclid::Scale<f32, Points, gooey::core::Pixels> {
        Scale::new(web_sys::window().unwrap().device_pixel_ratio() as f32)
    }

    fn render_text<F: gooey::core::styles::FallbackComponent<Value = ColorPair>>(
        &self,
        text: &str,
        baseline_origin: Point2D<f32, Points>,
        style: &Style,
    ) {
        let color = style
            .get_with_fallback::<F>()
            .cloned()
            .unwrap_or_default()
            .themed_color(self.theme);
        if let Some(context) = self.rendering_context() {
            context.save();
            self.clip(&context);
            context.set_fill_style(&JsValue::from_str(&color.as_css_string()));
            context
                .fill_text(text, baseline_origin.x as f64, baseline_origin.y as f64)
                .unwrap();
            context.restore();
        }
    }

    fn measure_text(&self, text: &str, _style: &Style) -> TextMetrics<Points> {
        if let Some(context) = self.rendering_context() {
            let metrics = ExtendedTextMetrics::from(context.measure_text(&text).unwrap());
            TextMetrics {
                width: Length::new(metrics.width() as f32),
                ascent: Length::new(metrics.actual_bounding_box_ascent() as f32),
                descent: Length::new(metrics.actual_bounding_box_descent() as f32),
                line_gap: Length::default(),
            }
        } else {
            TextMetrics::default()
        }
    }

    fn stroke_rect(&self, rect: &Rect<f32, Points>, style: &Style) {
        let color = style
            .get_with_fallback::<ForegroundColor>()
            .cloned()
            .unwrap_or_default()
            .themed_color(self.theme);
        if let Some(context) = self.rendering_context() {
            context.save();
            self.clip(&context);
            context.set_stroke_style(&JsValue::from_str(&color.as_css_string()));
            let rect = rect.to_f64();
            context.stroke_rect(
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
            );
            context.restore();
        }
    }

    fn fill_rect<F: gooey::core::styles::FallbackComponent<Value = ColorPair>>(
        &self,
        rect: &Rect<f32, Points>,
        style: &Style,
    ) {
        let color = style
            .get_with_fallback::<F>()
            .cloned()
            .unwrap_or_default()
            .themed_color(self.theme);
        if let Some(context) = self.rendering_context() {
            context.save();
            self.clip(&context);
            context.set_fill_style(&JsValue::from_str(&color.as_css_string()));
            let rect = rect.to_f64();
            context.fill_rect(
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
            );
            context.restore();
        }
    }

    fn stroke_line(
        &self,
        point_a: Point2D<f32, Points>,
        point_b: Point2D<f32, Points>,
        style: &Style,
    ) {
        let color = style
            .get_with_fallback::<ForegroundColor>()
            .cloned()
            .unwrap_or_default()
            .themed_color(self.theme);
        if let Some(context) = self.rendering_context() {
            context.save();
            self.clip(&context);
            context.set_stroke_style(&JsValue::from_str(&color.as_css_string()));
            context.begin_path();
            let point_a = point_a.to_f64();
            context.move_to(point_a.x, point_a.y);
            let point_b = point_b.to_f64();
            context.line_to(point_b.x, point_b.y);
            context.stroke();
            context.restore();
        }
    }
}

impl From<CanvasTransmogrifier> for RegisteredTransmogrifier {
    fn from(transmogrifier: CanvasTransmogrifier) -> Self {
        Self(Box::new(transmogrifier))
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type ExtendedTextMetrics;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxAscent)]
    pub fn actual_bounding_box_ascent(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxDescent)]
    pub fn actual_bounding_box_descent(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxLeft)]
    pub fn actual_bounding_box_left(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxRight)]
    pub fn actual_bounding_box_right(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &ExtendedTextMetrics) -> f64;
}

impl From<web_sys::TextMetrics> for ExtendedTextMetrics {
    fn from(tm: web_sys::TextMetrics) -> Self {
        tm.unchecked_into()
    }
}

impl ExtendedTextMetrics {
    pub fn height(&self) -> f64 {
        self.actual_bounding_box_ascent() + self.actual_bounding_box_descent()
    }
}
