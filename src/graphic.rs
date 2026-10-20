#[derive(Debug)]
pub struct Graphic {
    pub graphic_type: GraphicType,
    pub stroke_width: usize,
    pub stroke_color: u32,
    pub fill_color: u32,
}

#[derive(Debug)]
pub enum GraphicType {
    Line {
        start: (usize, usize),
        end: (usize, usize),
    },
    Rectangle {
        width: usize,
        height: usize,
        start: (usize, usize),
    },
    Path {
        commands: Vec<PathCommand>,
    },
}

use crate::image::Image;
use crate::utils::distance_between_point_and_line;
use crate::{Pixel, Resolution};
type StraightLineFromToFn = fn((usize, usize), (usize, usize), usize, u32, &mut Image, &Resolution);
type PlotPixelFn = fn(usize, usize, Pixel, Option<usize>, &mut Image, &Resolution);

impl GraphicType {
    pub fn render(
        &self,
        width: usize,
        stroke_color: u32,
        fill_color: u32,
        mut final_image: &mut Image,
        resolution: &Resolution,
        straight_line_from_to: StraightLineFromToFn,
        plot_pixel: PlotPixelFn,
    ) {
        match &self {
            // Basic Bresenham Algorithm
            GraphicType::Line { start, end } => {
                let ((x0, y0), (x1, y1)) = if start.0 > end.0 {
                    (end, start)
                } else {
                    (start, end)
                };

                straight_line_from_to(
                    (*x0, *y0),
                    (*x1, *y1),
                    width,
                    stroke_color,
                    final_image,
                    resolution,
                );
            }
            GraphicType::Rectangle {
                width,
                height,
                start,
            } => {
                let (x, y) = start;
                // Fill
                for i in 1..(*width) {
                    for j in 1..(*height) {
                        plot_pixel(
                            x + i,
                            y + j,
                            Pixel::from_hex(fill_color),
                            None,
                            final_image,
                            resolution,
                        );
                    }
                }

                // Top
                straight_line_from_to(
                    (*x, *y),
                    (*x + width, *y),
                    *width,
                    stroke_color,
                    &mut final_image,
                    resolution,
                );

                // Bottom
                straight_line_from_to(
                    (*x, *y + height),
                    (*x + width, *y + height),
                    *width,
                    stroke_color,
                    &mut final_image,
                    resolution,
                );

                // Left
                straight_line_from_to(
                    (*x, *y),
                    (*x, *y + height),
                    *width,
                    stroke_color,
                    &mut final_image,
                    resolution,
                );

                // Right
                straight_line_from_to(
                    (*x + width, *y),
                    (*x + width, *y + height),
                    *width,
                    stroke_color,
                    &mut final_image,
                    resolution,
                );
            }
            GraphicType::Path { commands, .. } => {
                println!("{:?}", commands);
                let mut edges: Vec<((f32, f32), (f32, f32))> = vec![];
                let mut current_point = (0f32, 0f32);
                let mut most_recent_move = (0f32, 0f32);
                for command in commands {
                    match command {
                        PathCommand::MoveTo(point) => {
                            most_recent_move = *point;
                            current_point = *point;
                        }
                        PathCommand::LineTo(point) => {
                            edges.push((current_point, *point));
                            current_point = *point;
                        }
                        // De Casteljau's algorithm
                        PathCommand::QuadraticTo((x1, y1), (anchor_x, anchor_y)) => {
                            let x0 = current_point.0;
                            let y0 = current_point.1;

                            fn recure(
                                x0: f32,
                                y0: f32,
                                anchor_x: f32,
                                anchor_y: f32,
                                x1: f32,
                                y1: f32,
                                depth: u32,
                                edges: &mut Vec<((f32, f32), (f32, f32))>,
                            ) {
                                let distance_between_anchor_and_line =
                                    distance_between_point_and_line(
                                        (anchor_x, anchor_y),
                                        (x0, y0),
                                        (x1, y1),
                                    );
                                if distance_between_anchor_and_line < 0.5 || depth == 16 {
                                    edges.push(((x0, y0), (x1, y1)));
                                    return;
                                }
                                let first_segment_mid_point =
                                    ((x0 + anchor_x) / 2f32, (y0 + anchor_y) / 2f32);
                                let second_segment_mid_point =
                                    ((anchor_x + x1) / 2f32, (anchor_y + y1) / 2f32);
                                let mid_point = (
                                    (first_segment_mid_point.0 + second_segment_mid_point.0) / 2f32,
                                    (first_segment_mid_point.1 + second_segment_mid_point.1) / 2f32,
                                );

                                recure(
                                    x0,
                                    y0,
                                    first_segment_mid_point.0,
                                    first_segment_mid_point.1,
                                    mid_point.0,
                                    mid_point.1,
                                    depth + 1,
                                    edges,
                                );
                                recure(
                                    mid_point.0,
                                    mid_point.1,
                                    second_segment_mid_point.0,
                                    second_segment_mid_point.1,
                                    x1,
                                    y1,
                                    depth + 1,
                                    edges,
                                );
                            }

                            recure(
                                x0 as f32,
                                y0 as f32,
                                *anchor_x as f32,
                                *anchor_y as f32,
                                *x1 as f32,
                                *y1 as f32,
                                0,
                                &mut edges,
                            );
                            current_point = (*x1, *y1);
                        }
                        PathCommand::CubicTo((x1, y1), (ax1, ay1), (ax2, ay2)) => {
                            let x0 = current_point.0;
                            let y0 = current_point.1;

                            fn recure(
                                x0: f32,
                                y0: f32,
                                ax1: f32,
                                ay1: f32,
                                ax2: f32,
                                ay2: f32,
                                x1: f32,
                                y1: f32,
                                depth: u32,
                                edges: &mut Vec<((f32, f32), (f32, f32))>,
                            ) {
                                let distance_1 =
                                    distance_between_point_and_line((ax1, ay1), (x0, y0), (x1, y1));
                                let distance_2 =
                                    distance_between_point_and_line((ax2, ay2), (x0, y0), (x1, y1));

                                if (distance_1 < 0.01 && distance_2 < 0.01) || depth == 16 {
                                    edges.push(((x0, y0), (x1, y1)));
                                    return;
                                }
                                let midpoint_a = ((x0 + ax1) / 2.0, (y0 + ay1) / 2.0);
                                let midpoint_b = ((ax1 + ax2) / 2.0, (ay1 + ay2) / 2.0);
                                let midpoint_c = ((ax2 + x1) / 2.0, (ay2 + y1) / 2.0);

                                let midpoint_ab = (
                                    (midpoint_a.0 + midpoint_b.0) / 2.0,
                                    (midpoint_a.1 + midpoint_b.1) / 2.0,
                                );
                                let midpoint_bc = (
                                    (midpoint_b.0 + midpoint_c.0) / 2.0,
                                    (midpoint_b.1 + midpoint_c.1) / 2.0,
                                );

                                let midpoint_abc = (
                                    (midpoint_ab.0 + midpoint_bc.0) / 2.0,
                                    (midpoint_ab.1 + midpoint_bc.1) / 2.0,
                                );

                                recure(
                                    x0,
                                    y0,
                                    midpoint_a.0,
                                    midpoint_a.1,
                                    midpoint_ab.0,
                                    midpoint_ab.1,
                                    midpoint_abc.0,
                                    midpoint_abc.1,
                                    depth + 1,
                                    edges,
                                );
                                recure(
                                    midpoint_abc.0,
                                    midpoint_abc.1,
                                    midpoint_bc.0,
                                    midpoint_bc.1,
                                    midpoint_c.0,
                                    midpoint_c.1,
                                    x1,
                                    y1,
                                    depth + 1,
                                    edges,
                                );
                            }

                            recure(
                                x0 as f32,
                                y0 as f32,
                                *ax1 as f32,
                                *ay1 as f32,
                                *ax2 as f32,
                                *ay2 as f32,
                                *x1 as f32,
                                *y1 as f32,
                                0,
                                &mut edges,
                            );

                            current_point = (*x1, *y1);
                        }
                        PathCommand::Close => {
                            edges.push((current_point, most_recent_move));
                            current_point = most_recent_move;
                        }
                    }
                }

                let min_y = edges
                    .iter()
                    .flat_map(|&(p0, p1)| [p0.1, p1.1])
                    .fold(f32::INFINITY, f32::min)
                    .floor() as i32;

                let max_y = edges
                    .iter()
                    .flat_map(|&(p0, p1)| [p0.1, p1.1])
                    .fold(f32::NEG_INFINITY, f32::max)
                    .ceil() as i32;

                let mut intersections = vec![];

                for y in min_y..max_y {
                    let scan_y = y as f32 + 0.5;

                    intersections.clear();

                    for &((x0, y0), (x1, y1)) in &edges {
                        if y0 == y1 {
                            continue;
                        }

                        let edge_min_y = y0.min(y1);
                        let edge_max_y = y0.max(y1);

                        if edge_min_y <= scan_y && scan_y <= edge_max_y {
                            let t = (scan_y - y0) / (y1 - y0);
                            let x = x0 + t * (x1 - x0);
                            intersections.push(x);
                        }
                    }
                    intersections.sort_by(|a, b| a.total_cmp(b));
                    for pair in intersections.chunks_exact(2) {
                        let left = pair[0];
                        let right = pair[1];

                        let start_x = (left - 0.5).ceil() as i32;
                        let end_x = (right - 0.5).ceil() as i32;

                        for x in start_x..end_x {
                            plot_pixel(
                                x as usize,
                                y as usize,
                                Pixel::from_hex(fill_color),
                                None,
                                &mut final_image,
                                resolution,
                            );
                        }
                    }
                }

                for edge in edges {
                    straight_line_from_to(
                        (edge.0.0 as usize, edge.0.1 as usize),
                        (edge.1.0 as usize, edge.1.1 as usize),
                        width,
                        stroke_color,
                        &mut final_image,
                        resolution,
                    );
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum PathCommand {
    MoveTo((f32, f32)),
    LineTo((f32, f32)),
    QuadraticTo((f32, f32), (f32, f32)),
    CubicTo((f32, f32), (f32, f32), (f32, f32)),
    Close,
}
