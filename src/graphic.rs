#[derive(Debug)]
pub(crate) struct Graphic {
    pub(crate) graphic_type: GraphicType,
    pub(crate) stroke_width: usize,
    pub(crate) stroke_color: Option<u32>,
    pub(crate) fill_color: Option<u32>,
}

#[derive(Debug)]
pub(crate) enum GraphicType {
    Line {
        start: (usize, usize),
        end: (usize, usize),
    },
    Rectangle {
        width: usize,
        height: usize,
        border_radius: f32,
    },
    Path {
        commands: Vec<PathCommand>,
    },
    Text {
        size: usize, // pixelss
        text: String,
    },
    Circle {
        radius: f32,
    },
}

use crate::canvas::LayerPosition;
use crate::graphic::PathCommand::MoveTo;
use crate::image::Image;
use crate::text::GlyphPath;
use crate::utils::distance_between_point_and_ray;
use crate::{Pixel, Resolution};
use ttf_parser::Face;

type StraightLineFromToFn =
    fn((usize, usize), (usize, usize), usize, u32, f32, &mut Image, &Resolution);
type PlotPixelFn = fn(usize, usize, Pixel, Option<usize>, &mut Image, &Resolution);

impl GraphicType {
    pub(crate) fn render(
        &self,
        stroke_width: usize,
        stroke_color: Option<u32>,
        fill_color: Option<u32>,
        position: &LayerPosition,
        scale: f32,
        opacity: f32,
        final_image: &mut Image,
        resolution: &Resolution,
        straight_line_from_to: StraightLineFromToFn,
        plot_pixel: PlotPixelFn,
    ) {
        match &self {
            // Basic Bresenham Algorithm
            GraphicType::Line { start, end } => {
                let edges = vec![(
                    (start.0 as f32, start.1 as f32),
                    (end.0 as f32, end.1 as f32),
                )];
                render_edges(
                    edges,
                    stroke_width,
                    stroke_color,
                    fill_color,
                    position,
                    scale,
                    opacity,
                    final_image,
                    resolution,
                    straight_line_from_to,
                    plot_pixel,
                );
            }
            GraphicType::Rectangle {
                width,
                height,
                border_radius,
            } => {
                let mut edges = vec![];
                let r = *border_radius;
                let w = *width as f32;
                let h = *height as f32;
                let commands = vec![
                    MoveTo((r, 0.)),
                    // Top
                    PathCommand::LineTo((w - r, 0.)),
                    // Top right
                    PathCommand::QuadraticTo((w, r), (w, 0.)),
                    // Right
                    PathCommand::LineTo((w, h - r)),
                    // Bottom Right
                    PathCommand::QuadraticTo((w - r, h), (w, h)),
                    // Bottom
                    PathCommand::LineTo((r, h)),
                    // Bottom Left
                    PathCommand::QuadraticTo((0., h - r), (0., h)),
                    // Left
                    PathCommand::LineTo((0., r)),
                    // Top Left
                    PathCommand::QuadraticTo((r, 0.), (0., 0.)),
                    PathCommand::Close,
                ];
                edges = path_to_edges(commands);

                render_edges(
                    edges,
                    stroke_width,
                    stroke_color,
                    fill_color,
                    position,
                    scale,
                    opacity,
                    final_image,
                    resolution,
                    straight_line_from_to,
                    plot_pixel,
                );
            }
            GraphicType::Path { commands } => {
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
                                    distance_between_point_and_ray(
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
                                    distance_between_point_and_ray((ax1, ay1), (x0, y0), (x1, y1));
                                let distance_2 =
                                    distance_between_point_and_ray((ax2, ay2), (x0, y0), (x1, y1));

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
                render_edges(
                    edges,
                    stroke_width,
                    stroke_color,
                    fill_color,
                    position,
                    scale,
                    opacity,
                    final_image,
                    resolution,
                    straight_line_from_to,
                    plot_pixel,
                );
            }
            GraphicType::Text { size, text } => {
                let font_data = std::fs::read("monobld.ttf").unwrap();
                let mut line_height = 0f32;
                let mut current_x = 0f32;
                let mut current_y = 0f32;
                let mut edges = Vec::new();
                for char in text.chars() {
                    if char == '\n' {
                        current_x = 0f32;
                        current_y += line_height;
                        continue;
                    }
                    let (path, hor_advance, units_per_em) =
                        extract_glyph(&font_data, char).unwrap();

                    let text_scale = (*size as f32 / units_per_em) as f32;
                    line_height = *size as f32 * 0.75;

                    let edges_inner = path_to_edges(path.commands);

                    let edges_inner: Vec<((f32, f32), (f32, f32))> = edges_inner
                        .iter()
                        .map(|((x, y), (x1, y1))| {
                            return (
                                ((x + current_x) * text_scale, (y * text_scale) - current_y),
                                ((x1 + current_x) * text_scale, (y1 * text_scale) - current_y),
                            );
                        })
                        .collect();

                    edges.extend(edges_inner);

                    current_x += hor_advance;
                }
                let min_y = edges
                    .iter()
                    .flat_map(|&(p0, p1)| [p0.1, p1.1])
                    .fold(f32::INFINITY, f32::min)
                    .floor();

                let max_y = edges
                    .iter()
                    .flat_map(|&(p0, p1)| [p0.1, p1.1])
                    .fold(f32::NEG_INFINITY, f32::max)
                    .ceil();

                let height = (max_y - min_y) * scale;

                let edges_flipped = edges
                    .into_iter()
                    .map(|(p0, p1)| ((p0.0, height - p0.1), (p1.0, height - p1.1)))
                    .collect::<Vec<_>>();

                render_edges(
                    edges_flipped,
                    stroke_width,
                    stroke_color,
                    fill_color,
                    position,
                    scale,
                    opacity,
                    final_image,
                    resolution,
                    straight_line_from_to,
                    plot_pixel,
                );
            }

            GraphicType::Circle { radius } => {
                let (cx, cy) = (0f32, 0f32);

                let r = *radius;

                let mut commands = Vec::new();

                let k = 0.552_284_8 * r;

                commands.push(PathCommand::MoveTo((cx, cy - r)));

                commands.push(PathCommand::CubicTo(
                    (cx + r, cy),
                    (cx + k, cy - r),
                    (cx + r, cy - k),
                ));

                commands.push(PathCommand::CubicTo(
                    (cx, cy + r),
                    (cx + r, cy + k),
                    (cx + k, cy + r),
                ));

                commands.push(PathCommand::CubicTo(
                    (cx - r, cy),
                    (cx - k, cy + r),
                    (cx - r, cy + k),
                ));

                commands.push(PathCommand::CubicTo(
                    (cx, cy - r),
                    (cx - r, cy - k),
                    (cx - k, cy - r),
                ));

                let edges = path_to_edges(commands);
                render_edges(
                    edges,
                    stroke_width,
                    stroke_color,
                    fill_color,
                    position,
                    scale,
                    opacity,
                    final_image,
                    resolution,
                    straight_line_from_to,
                    plot_pixel,
                );
            }
        }
    }
}

fn path_to_edges(commands: Vec<PathCommand>) -> Vec<((f32, f32), (f32, f32))> {
    let mut edges: Vec<((f32, f32), (f32, f32))> = vec![];
    let mut current_point = (0f32, 0f32);
    let mut most_recent_move = (0f32, 0f32);
    for command in commands {
        match command {
            PathCommand::MoveTo(point) => {
                most_recent_move = point;
                current_point = point;
            }
            PathCommand::LineTo(point) => {
                edges.push((current_point, point));
                current_point = point;
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
                        distance_between_point_and_ray((anchor_x, anchor_y), (x0, y0), (x1, y1));
                    if distance_between_anchor_and_line < 0.1 || depth == 10 {
                        edges.push(((x0, y0), (x1, y1)));
                        return;
                    }
                    let first_segment_mid_point = ((x0 + anchor_x) / 2f32, (y0 + anchor_y) / 2f32);
                    let second_segment_mid_point = ((anchor_x + x1) / 2f32, (anchor_y + y1) / 2f32);
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
                    anchor_x as f32,
                    anchor_y as f32,
                    x1 as f32,
                    y1 as f32,
                    0,
                    &mut edges,
                );
                current_point = (x1, y1);
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
                    let max_depth = 5;
                    let tolerance = 0.1;

                    let distance_1 = distance_between_point_and_ray((ax1, ay1), (x0, y0), (x1, y1));
                    let distance_2 = distance_between_point_and_ray((ax2, ay2), (x0, y0), (x1, y1));

                    if (distance_1 < tolerance && distance_2 < tolerance) || depth == max_depth {
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
                    x0 as f32, y0 as f32, ax1 as f32, ay1 as f32, ax2 as f32, ay2 as f32,
                    x1 as f32, y1 as f32, 0, &mut edges,
                );

                current_point = (x1, y1);
            }
            PathCommand::Close => {
                edges.push((current_point, most_recent_move));
                current_point = most_recent_move;
            }
        }
    }
    return edges;
}

fn render_edges(
    edges: Vec<((f32, f32), (f32, f32))>,
    stroke_width: usize,
    stroke_color: Option<u32>,
    fill_color: Option<u32>,
    position: &LayerPosition,
    scale: f32,
    opacity: f32,
    mut final_image: &mut Image,
    resolution: &Resolution,
    straight_line_from_to: StraightLineFromToFn,
    plot_pixel: PlotPixelFn,
) {
    let min_y = edges
        .iter()
        .flat_map(|&(p0, p1)| [p0.1, p1.1])
        .fold(f32::INFINITY, f32::min)
        .floor()
        * scale;

    let max_y = edges
        .iter()
        .flat_map(|&(p0, p1)| [p0.1, p1.1])
        .fold(f32::NEG_INFINITY, f32::max)
        .ceil()
        * scale;

    let max_x = edges
        .iter()
        .flat_map(|&(p0, p1)| [p0.0, p1.0])
        .fold(f32::NEG_INFINITY, f32::max)
        .ceil()
        * scale;

    let min_x = edges
        .iter()
        .flat_map(|&(p0, p1)| [p0.0, p1.0])
        .fold(f32::INFINITY, f32::min)
        .floor()
        * scale;

    let width = max_x - min_x;
    let height = max_y - min_y;

    let (pixel_x, pixel_y) = position.to_translation(width, height, resolution);

    let translate_x = pixel_x - (min_x);
    let translate_y = pixel_y - (min_y);

    let mut intersections = vec![];

    let transform_point = |(x, y): (f32, f32)| {
        let x = x as f32 * scale + translate_x;
        let y = y as f32 * scale + translate_y;
        (x, y)
    };

    let edges: Vec<((f32, f32), (f32, f32))> = edges
        .iter()
        .map(|((x0, y0), (x1, y1)): &((f32, f32), (f32, f32))| {
            let (x0, y0) = transform_point((*x0, *y0));
            let (x1, y1) = transform_point((*x1, *y1));
            ((x0, y0), (x1, y1))
        })
        .collect();

    // Filling the thing
    if let Some(fill_color) = fill_color {
        for y in (min_y as i32 + translate_y as i32)..(max_y as i32 + translate_y as i32) {
            let scan_y = y as f32 + 0.5;

            intersections.clear();

            for &((x0, y0), (x1, y1)) in &edges {
                if y0 == y1 {
                    continue;
                }

                let edge_min_y = y0.min(y1);
                let edge_max_y = y0.max(y1);

                if edge_min_y <= scan_y && scan_y < edge_max_y {
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

                let mut pixel = Pixel::from_hex(fill_color);
                pixel.a = (255f32 * opacity) as u8;
                for x in start_x..end_x {
                    plot_pixel(
                        x as usize,
                        y as usize,
                        pixel,
                        None,
                        &mut final_image,
                        resolution,
                    );
                }
            }
        }
    }

    // Draw the edges
    if let Some(stroke_color) = stroke_color {
        for edge in edges {
            straight_line_from_to(
                (edge.0.0 as usize, edge.0.1 as usize),
                (edge.1.0 as usize, edge.1.1 as usize),
                stroke_width as usize,
                stroke_color,
                opacity,
                &mut final_image,
                resolution,
            );
        }
    }
}

fn extract_glyph(font_data: &[u8], character: char) -> Option<(GlyphPath, f32, f32)> {
    let face = Face::parse(font_data, 0).ok()?;

    let glyph_id = face.glyph_index(character)?;

    let advance_width = face.glyph_hor_advance(glyph_id)? as f32;

    let mut path = GlyphPath::new();

    face.outline_glyph(glyph_id, &mut path);

    Some((path, advance_width, face.units_per_em() as f32))
}

#[derive(Debug)]
pub(crate) enum PathCommand {
    MoveTo((f32, f32)),
    LineTo((f32, f32)),
    QuadraticTo((f32, f32), (f32, f32)),
    CubicTo((f32, f32), (f32, f32), (f32, f32)),
    Close,
}
