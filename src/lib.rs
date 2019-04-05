use std::f64;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use rainbow_unicorn::*;
use rainbow_unicorn::primitives::*;

pub struct HtmlCanvasPainter {
    context: web_sys::CanvasRenderingContext2d
}

impl HtmlCanvasPainter {
    pub fn new() -> HtmlCanvasPainter {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("rainbow-unicorn-canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        HtmlCanvasPainter {
            context: context
        }
    }

    fn draw_round_rect(&self, data: &RectangleData, styles: &Styles) {
        self.context.begin_path();
        self.context.move_to(
            (data.left + data.rx) as f64,
            data.top as f64
        );
        self.context.line_to(
            (data.left + data.width - data.rx) as f64,
            data.top as f64
        );
        self.context.quadratic_curve_to(
            (data.left + data.width) as f64,
            data.top as f64,
            (data.left + data.width) as f64,
            (data.top + data.ry) as f64
        );
        self.context.line_to(
            (data.left + data.width) as f64,
            (data.top + data.height - data.ry) as f64
        );
        self.context.quadratic_curve_to(
            (data.left + data.width) as f64,
            (data.top + data.height) as f64,
            (data.left + data.width - data.rx) as f64,
            (data.top + data.height) as f64
        );
        self.context.line_to(
            (data.left + data.rx) as f64,
            (data.top + data.height) as f64
        );
        self.context.quadratic_curve_to(
            data.left as f64,
            (data.top + data.height) as f64,
            data.left as f64,
            (data.top + data.height - data.ry) as f64
        );
        self.context.line_to(
            data.left as f64,
            (data.top + data.ry) as f64
        );
        self.context.quadratic_curve_to(
            data.left as f64,
            data.top as f64,
            (data.left + data.rx) as f64,
            data.top as f64
        );

        HtmlCanvasPainter::fill_or_stroke(&self.context, styles);

        self.context.close_path();
    }

    fn fill_or_stroke(context: &web_sys::CanvasRenderingContext2d, styles: &Styles) {
        match styles {
            Styles::Fill { color } => {
                HtmlCanvasPainter::apply_fill(context, color);
                context.fill();
            }
            Styles::Stroke { color, width } => {
                HtmlCanvasPainter::apply_stroke(context, color, *width as f64);
                context.stroke();
            }
        }
    }

    fn apply_fill(context: &web_sys::CanvasRenderingContext2d, color: &String) {
        let js_color = JsValue::from_str(&color[..]);
        context.set_fill_style(&js_color);
    }

    fn apply_stroke(context: &web_sys::CanvasRenderingContext2d, color: &String, width: f64) {
        let js_color = JsValue::from_str(&color[..]);
        context.set_stroke_style(&js_color);
        context.set_line_width(width);
    }

    fn draw_path_segment(context: &web_sys::CanvasRenderingContext2d, segment: &PathSegment) {
        match segment {
            PathSegment::MoveTo { to_x, to_y } => {
                context.move_to(*to_x as f64, *to_y as f64);
            }
            PathSegment::LineTo { to_x, to_y } => {
                context.line_to(*to_x as f64, *to_y as f64);
            }
            PathSegment::BezierCurveTo {
                cp1_x,
                cp1_y,
                cp2_x,
                cp2_y,
                to_x,
                to_y
            } => {
                context.bezier_curve_to(
                    *cp1_x as f64,
                    *cp1_y as f64,
                    *cp2_x as f64,
                    *cp2_y as f64,
                    *to_x as f64,
                    *to_y as f64
                );
            }
            PathSegment::QuadraticCurveTo {
                cp_x,
                cp_y,
                to_x,
                to_y
            } => {
                context.quadratic_curve_to(
                    *cp_x as f64,
                    *cp_y as f64,
                    *to_x as f64,
                    *to_y as f64
                );
            }
        }
    }
}

impl Painter for HtmlCanvasPainter {

    fn clear(&self) {
        let canvas = self.context.canvas().unwrap();
        self.context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    }

    fn draw_rect(&self, data: &RectangleData, styles: &Styles) {
        if data.rx > 0 || data.ry > 0 {
            self.draw_round_rect(data, styles);
        } else {
            match styles {
                Styles::Fill { color } => {
                    HtmlCanvasPainter::apply_fill(&self.context, color);
                    self.context.fill_rect(
                        data.left as f64,
                        data.top as f64,
                        data.width as f64,
                        data.height as f64
                    );
                }
                Styles::Stroke { color, width } => {
                    HtmlCanvasPainter::apply_stroke(&self.context, color, *width as f64);
                    self.context.stroke_rect(
                        data.left as f64,
                        data.top as f64,
                        data.width as f64,
                        data.height as f64
                    );
                }
            }
        }
    }

    fn draw_line(&self, data: &LineData, styles: &Styles) {
        self.context.begin_path();
        self.context.move_to(data.x1 as f64, data.y1 as f64);
        self.context.line_to(data.x2 as f64, data.y2 as f64);
        HtmlCanvasPainter::fill_or_stroke(&self.context, styles);
        self.context.close_path();
    }

    fn draw_ellipse(&self, data: &EllipseData, styles: &Styles) {
        self.context.begin_path();
        self.context.ellipse(
            data.cx as f64,
            data.cy as f64,
            data.rx as f64,
            data.ry as f64,
            0.0,
            0.0,
            2.0 * f64::consts::PI
        );
        HtmlCanvasPainter::fill_or_stroke(&self.context, styles);
        self.context.close_path();
    }

    fn draw_text(&self, data: &TextData, styles: &Styles) {
        self.context.set_font(&data.font[..]);

        match styles {
            Styles::Fill { color } => {
                HtmlCanvasPainter::apply_fill(&self.context, color);
                self.context.fill_text(&data.text[..], data.x as f64, data.y as f64);
            }
            Styles::Stroke { color, width } => {
                HtmlCanvasPainter::apply_stroke(&self.context, color, *width as f64);
                self.context.stroke_text(&data.text[..], data.x as f64, data.y as f64);
            }
        }
    }

    fn draw_polygon(&self, points: &Vec<Point>, styles: &Styles) {
        self.context.begin_path();
        self.context.move_to(points[0].x as f64, points[0].y as f64);
        
        for p in points.iter() {
            self.context.line_to(p.x as f64, p.y as f64);
        }
        self.context.line_to(points[0].x as f64, points[0].y as f64);

        HtmlCanvasPainter::fill_or_stroke(&self.context, styles);

        self.context.close_path();
    }

    fn draw_path(&self, data: &PathData, styles: &Styles) {
        self.context.begin_path();
        self.context.move_to(data.start_x as f64, data.start_y as f64);

        for segment in data.segments.iter() {
            HtmlCanvasPainter::draw_path_segment(&self.context, segment);
        }

        HtmlCanvasPainter::fill_or_stroke(&self.context, styles);
        self.context.close_path();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}
