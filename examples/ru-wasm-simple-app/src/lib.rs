use rainbow_unicorn;
use rainbow_unicorn::*;
use rainbow_unicorn::primitives::*;
use rainbow_unicorn::timelines::*;

use rainbow_unicorn_wasm::HtmlCanvasPainter;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SimpleApp {
    painter: HtmlCanvasPainter,
    state_container: AppStateContainer
}

#[wasm_bindgen]
impl SimpleApp {
    pub fn new() -> SimpleApp {
        SimpleApp {
            painter: HtmlCanvasPainter::new(),
            state_container: AppStateContainer::new()
        }
    }

    pub fn tick(&self, timestamp_millis: f64) {
        rainbow_unicorn::tick(timestamp_millis, &self.state_container, &self.painter);
    }
}

#[wasm_bindgen]
struct AppStateContainer {}

#[wasm_bindgen]
impl AppStateContainer {
    pub fn new() -> AppStateContainer {
        AppStateContainer {}
    }
}
impl StateContainer for AppStateContainer {
    fn update(&self, timestamp_millis: f64) -> Vec<GeometricPrimitive> {
        let rect_x_offset = 400.0 * oscillating_proportion(timestamp_millis, 1000.0);
        let rect_left = (50.0 + rect_x_offset) as u32;

        example_graphics_primitives(rect_left)
    }
}

fn example_graphics_primitives(moving_rect_left: u32) -> Vec<GeometricPrimitive> {
    vec![
        GeometricPrimitive::Rectangle {
            data: RectangleData {
                left: moving_rect_left,
                top: 75,
                width: 100,
                height: 75,
                rx: 0,
                ry: 0
            },
            styles: Styles::Fill {
                color: "blue".to_string()
            }
        },
        GeometricPrimitive::Rectangle {
            data: RectangleData {
                left: 600,
                top: 400,
                width: 100,
                height: 75,
                rx: 10,
                ry: 10
            },
            styles: Styles::Fill {
                color: "pink".to_string()
            }
        },
        GeometricPrimitive::Line {
            data: LineData {
                x1: 250,
                y1: 275,
                x2: 350,
                y2: 150
            },
            styles: Styles::Stroke {
                color: "green".to_string(),
                width: 5
            }
        },
        GeometricPrimitive::Text {
            data: TextData {
                x: 300,
                y: 25,
                text: "hello".to_string(),
                font: "Dialog".to_string()
            },
            styles: Styles::Fill {
                color: "#FF6347".to_string()
            }
        },
        GeometricPrimitive::Ellipse {
            data: EllipseData {
                cx: 400,
                cy: 100,
                rx: 25,
                ry: 75
            },
            styles: Styles::Stroke {
                color: "green".to_string(),
                width: 5
            }
        },
        GeometricPrimitive::Polygon {
            points: vec![
                Point { x: 20, y: 20 },
                Point { x: 30, y: 20 },
                Point { x: 25, y: 10 }
            ],
            styles: Styles::Fill {
                color: "yellow".to_string()
            }
        },
        GeometricPrimitive::RuPath {
            data: PathData {
                start_x: 300,
                start_y: 400,
                segments: vec![
                    PathSegment::LineTo { to_x: 350, to_y: 400 },
                    PathSegment::BezierCurveTo {
                        cp1_x: 450,
                        cp1_y: 450,
                        cp2_x: 700,
                        cp2_y: 100,
                        to_x: 300,
                        to_y: 250
                    },
                    PathSegment::QuadraticCurveTo {
                        cp_x: 100,
                        cp_y: 50,
                        to_x: 325,
                        to_y: 150
                    }
                ]
            },
            styles: Styles::Stroke {
                color: "purple".to_string(),
                width: 2
            }
        }
    ]
}