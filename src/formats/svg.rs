#[derive(Debug)]
pub(crate) struct SVGData {
    pub elements: Vec<Graphic>,
}

fn evaluate_svg_transform(transforms: &Vec<String>) -> Result<(f32, f32, f32), String> {
    let (mut scale, mut tx, mut ty) = (1.0, 0.0, 0.0);

    for input in transforms {
        let mut remaining = input.trim();

        while !remaining.is_empty() {
            remaining = remaining.trim_start_matches(|c: char| c.is_whitespace() || c == ',');

            if remaining.is_empty() {
                break;
            }

            let open = remaining.find('(').ok_or("Expected '('")?;
            let close = remaining[open + 1..].find(')').ok_or("Expected ')'")? + open + 1;

            let name = remaining[..open].trim();

            let args: Vec<f32> = remaining[open + 1..close]
                .split(|c: char| c == ',' || c.is_whitespace())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<f32>())
                .collect::<Result<_, _>>()
                .map_err(|_| "Invalid number")?;

            match name {
                "translate" if (1..=2).contains(&args.len()) => {
                    tx += args[0] * scale;
                    ty += args.get(1).copied().unwrap_or(0.0) * scale;
                }

                "scale" if args.len() == 1 => {
                    scale *= args[0];
                }

                _ => return Err(format!("Unsupported or invalid transform: {name}")),
            }

            remaining = &remaining[close + 1..];
        }
    }

    Ok((scale, tx, ty))
}

pub(crate) fn read_svg_from_path(path: &str) -> Result<SVGData, Error> {
    let mut svg_data = SVGData {
        elements: Vec::new(),
    };
    let file = std::fs::read(path).unwrap();
    let svg = std::str::from_utf8(&file).unwrap();
    let doc = roxmltree::Document::parse(&svg).unwrap();

    let root = doc.root_element();

    fn recure(root: &Node, svg_data: &mut SVGData, transforms: &mut Vec<String>) {
        for node in root.children() {
            let tag = node.tag_name().name();

            match tag {
                "g" => {
                    if let Some(transform) = node.attribute("transform") {
                        transforms.push(transform.to_string());
                        recure(&node, svg_data, transforms);
                        transforms.pop();
                    } else {
                        recure(&node, svg_data, transforms);
                    }
                }
                _ => {
                    let mut element_has_transform = false;
                    if let Some(transform) = node.attribute("transform") {
                        transforms.push(transform.to_string());
                        element_has_transform = true;
                    }
                    let (scale, translate_x, translate_y) =
                        evaluate_svg_transform(transforms).unwrap();

                    match tag {
                        "path" => {
                            let mut fill_color = None;
                            if let Some(c) = node.attribute("fill") {
                                fill_color = Some(string_hex_to_u32(c));
                            }
                            let mut stroke_color = None;
                            if let Some(c) = node.attribute("stroke") {
                                stroke_color = Some(string_hex_to_u32(c));
                            }
                            let mut stroke_width = 1;
                            if let Some(w) = node.attribute("stroke-width") {
                                stroke_width = w.parse::<f32>().unwrap() as usize;
                            }
                            let d = node.attribute("d").unwrap();
                            let commands =
                                svg_to_path(&d, scale, (translate_x, translate_y)).unwrap();

                            let graphic = Graphic {
                                fill_color,
                                stroke_color,
                                stroke_width,
                                graphic_type: GraphicType::Path { commands },
                            };
                            svg_data.elements.push(graphic);
                        }
                        "rect" => {
                            let x = node.attribute("x").unwrap_or("0").parse::<f32>().unwrap();
                            let y = node.attribute("y").unwrap_or("0").parse::<f32>().unwrap();
                            let width = node.attribute("width").unwrap().parse::<f32>().unwrap();
                            let height = node.attribute("height").unwrap().parse::<f32>().unwrap();
                            let mut fill_color = None;
                            if let Some(c) = node.attribute("fill") {
                                fill_color = Some(string_hex_to_u32(c));
                            }
                            let mut stroke_color = None;
                            if let Some(c) = node.attribute("stroke") {
                                stroke_color = Some(string_hex_to_u32(c));
                            }
                            let mut stroke_width = 1;
                            if let Some(w) = node.attribute("stroke-width") {
                                stroke_width = w.parse::<f32>().unwrap() as usize;
                            }

                            // Stored as a path so the rectangle's position is preserved,
                            // since GraphicType::Rectangle only carries width and height.
                            let commands = vec![
                                PathCommand::MoveTo((x, y)),
                                PathCommand::LineTo((x + width, y)),
                                PathCommand::LineTo((x + width, y + height)),
                                PathCommand::LineTo((x, y + height)),
                                PathCommand::Close,
                            ];

                            let graphic = Graphic {
                                fill_color,
                                stroke_color,
                                stroke_width,
                                graphic_type: GraphicType::Path { commands },
                            };
                            svg_data.elements.push(graphic);
                        }
                        "circle" => {
                            let cx = node.attribute("cx").unwrap_or("0").parse::<f32>().unwrap();
                            let cy = node.attribute("cy").unwrap_or("0").parse::<f32>().unwrap();
                            let r = node.attribute("r").unwrap().parse::<f32>().unwrap();
                            let mut fill_color = None;
                            if let Some(c) = node.attribute("fill") {
                                fill_color = Some(string_hex_to_u32(c));
                            }
                            let mut stroke_color = None;
                            if let Some(c) = node.attribute("stroke") {
                                stroke_color = Some(string_hex_to_u32(c));
                            }
                            let mut stroke_width = 1;
                            if let Some(w) = node.attribute("stroke") {
                                stroke_width = w.parse::<f32>().unwrap() as usize;
                            }

                            // Stored as a path (two half-circle arcs) so the circle's
                            // position is preserved, since GraphicType::Circle only
                            // carries a radius.
                            let mut commands = vec![PathCommand::MoveTo((
                                (cx - r) * scale + translate_x,
                                cy * scale + translate_y,
                            ))];
                            append_svg_arc(
                                &mut commands,
                                ((cx - r) * scale + translate_x, cy * scale + translate_y),
                                r * scale,
                                r * scale,
                                0.,
                                1.,
                                1.,
                                ((cx + r) * scale + translate_x, cy * scale + translate_y),
                            );
                            append_svg_arc(
                                &mut commands,
                                ((cx + r) * scale + translate_x, cy * scale + translate_y),
                                r * scale,
                                r * scale,
                                0.,
                                1.,
                                1.,
                                ((cx - r) * scale + translate_x, cy * scale + translate_y),
                            );

                            commands.push(PathCommand::Close);

                            let graphic = Graphic {
                                fill_color,
                                stroke_color,
                                stroke_width,
                                graphic_type: GraphicType::Path { commands },
                            };
                            svg_data.elements.push(graphic);
                        }
                        "ellipse" => {
                            let cx = node.attribute("cx").unwrap_or("0").parse::<f32>().unwrap();
                            let cy = node.attribute("cy").unwrap_or("0").parse::<f32>().unwrap();
                            let rx = node.attribute("rx").unwrap().parse::<f32>().unwrap();
                            let ry = node.attribute("ry").unwrap().parse::<f32>().unwrap();
                            let mut fill_color = None;
                            if let Some(c) = node.attribute("fill") {
                                fill_color = Some(string_hex_to_u32(c));
                            }
                            let mut stroke_color = None;
                            if let Some(c) = node.attribute("stroke") {
                                stroke_color = Some(string_hex_to_u32(c));
                            }
                            let mut stroke_width = 1;
                            if let Some(w) = node.attribute("stroke-width") {
                                stroke_width = w.parse::<f32>().unwrap() as usize;
                            }

                            let mut commands = vec![PathCommand::MoveTo((cx - rx, cy))];
                            append_svg_arc(
                                &mut commands,
                                (cx - rx, cy),
                                rx,
                                ry,
                                0.,
                                1.,
                                1.,
                                (cx + rx, cy),
                            );
                            append_svg_arc(
                                &mut commands,
                                (cx + rx, cy),
                                rx,
                                ry,
                                0.,
                                1.,
                                1.,
                                (cx - rx, cy),
                            );
                            commands.push(PathCommand::Close);

                            let graphic = Graphic {
                                fill_color,
                                stroke_color,
                                stroke_width,
                                graphic_type: GraphicType::Path { commands },
                            };
                            svg_data.elements.push(graphic);
                        }
                        "line" => {
                            let x1 = node.attribute("x1").unwrap().parse::<f32>().unwrap();
                            let y1 = node.attribute("y1").unwrap().parse::<f32>().unwrap();
                            let x2 = node.attribute("x2").unwrap().parse::<f32>().unwrap();
                            let y2 = node.attribute("y2").unwrap().parse::<f32>().unwrap();
                            let mut stroke_color = None;
                            if let Some(c) = node.attribute("stroke") {
                                stroke_color = Some(string_hex_to_u32(c));
                            }
                            let mut stroke_width = 1;
                            if let Some(w) = node.attribute("stroke-width") {
                                stroke_width = w.parse::<f32>().unwrap() as usize;
                            }

                            let graphic = Graphic {
                                fill_color: None,
                                stroke_color,
                                stroke_width,
                                graphic_type: GraphicType::Line {
                                    start: (x1 as usize, y1 as usize),
                                    end: (x2 as usize, y2 as usize),
                                },
                            };
                            svg_data.elements.push(graphic);
                        }
                        "text" => {
                            let size = node
                                .attribute("font-size")
                                .unwrap_or("16")
                                .parse::<f32>()
                                .unwrap() as usize;
                            let text = node.text().unwrap_or("").trim().to_string();
                            let mut fill_color = None;
                            if let Some(c) = node.attribute("fill-color") {
                                fill_color = Some(string_hex_to_u32(c));
                            }

                            let graphic = Graphic {
                                fill_color,
                                stroke_color: None,
                                stroke_width: 1,
                                graphic_type: GraphicType::Text { size, text },
                            };
                            svg_data.elements.push(graphic);
                        }
                        "polygon" | "polyline" => {
                            let points = node.attribute("points").unwrap();
                            let values: Vec<f32> = points
                                .split(|c: char| c.is_whitespace() || c == ',')
                                .filter(|s| !s.is_empty())
                                .map(|v| v.parse::<f32>().unwrap())
                                .collect();
                            let mut fill_color = None;
                            if let Some(c) = node.attribute("fill") {
                                fill_color = Some(string_hex_to_u32(c));
                            }
                            let mut stroke_color = None;
                            if let Some(c) = node.attribute("stroke") {
                                stroke_color = Some(string_hex_to_u32(c));
                            }
                            let mut stroke_width = 1;
                            if let Some(w) = node.attribute("stroke-width") {
                                stroke_width = w.parse::<f32>().unwrap() as usize;
                            }

                            let mut commands = Vec::new();
                            for pair in values.chunks(2) {
                                let point = (pair[0], pair[1]);
                                if commands.is_empty() {
                                    commands.push(PathCommand::MoveTo(point));
                                } else {
                                    commands.push(PathCommand::LineTo(point));
                                }
                            }
                            // A polygon is closed; a polyline is left open.
                            if tag == "polygon" {
                                commands.push(PathCommand::Close);
                            }

                            let graphic = Graphic {
                                fill_color,
                                stroke_color,
                                stroke_width,
                                graphic_type: GraphicType::Path { commands },
                            };
                            svg_data.elements.push(graphic);
                        }
                        _ => (),
                    }
                    if element_has_transform {
                        transforms.pop();
                    }
                }
            }
        }
    }

    let mut transforms = Vec::new();
    recure(&root, &mut svg_data, &mut transforms);
    // dbg!(&svg_data);
    Ok(svg_data)
}

use std::io::Error;

use roxmltree::Node;

use crate::{
    graphic::{Graphic, GraphicType, PathCommand, path_to_edges, render_edges},
    utils::string_hex_to_u32,
};
#[derive(Debug)]
struct PathCommandRaw {
    pub command: char,
    pub values: Vec<f64>,
}

#[derive(Debug, Default)]
struct SeparatorCounts {
    commas: usize,
    spaces: usize,
    tabs: usize,
    newlines: usize,
    implicit: usize,
}

// THIS FUNCTION WAS WRITTING BY AI
fn is_command(c: u8) -> bool {
    matches!(
        c as char,
        'M' | 'm'
            | 'L'
            | 'l'
            | 'H'
            | 'h'
            | 'V'
            | 'v'
            | 'C'
            | 'c'
            | 'S'
            | 's'
            | 'Q'
            | 'q'
            | 'T'
            | 't'
            | 'A'
            | 'a'
            | 'Z'
            | 'z'
    )
}

// THIS FUNCTION WAS WRITTING BY AI
fn is_number_start(c: u8) -> bool {
    c.is_ascii_digit() || c == b'+' || c == b'-' || c == b'.'
}

// THIS FUNCTION WAS WRITTING BY AI
fn parse_number(bytes: &[u8], i: &mut usize) -> Result<f64, String> {
    let start = *i;
    let len = bytes.len();

    if *i < len && (bytes[*i] == b'+' || bytes[*i] == b'-') {
        *i += 1;
    }

    let mut digits = 0;

    while *i < len && bytes[*i].is_ascii_digit() {
        digits += 1;
        *i += 1;
    }

    if *i < len && bytes[*i] == b'.' {
        *i += 1;

        while *i < len && bytes[*i].is_ascii_digit() {
            digits += 1;
            *i += 1;
        }
    }

    if digits == 0 {
        return Err(format!("Invalid number at byte {start}"));
    }

    if *i < len && (bytes[*i] == b'e' || bytes[*i] == b'E') {
        *i += 1;

        if *i < len && (bytes[*i] == b'+' || bytes[*i] == b'-') {
            *i += 1;
        }

        let exponent_start = *i;

        while *i < len && bytes[*i].is_ascii_digit() {
            *i += 1;
        }

        if *i == exponent_start {
            return Err(format!("Invalid exponent at byte {start}"));
        }
    }

    let text = std::str::from_utf8(&bytes[start..*i]).map_err(|e| e.to_string())?;

    text.parse::<f64>()
        .map_err(|e| format!("Invalid number '{text}': {e}"))
}

// THIS FUNCTION WAS WRITTING BY AI
fn parse_svg_path(data: &str) -> Result<(Vec<PathCommandRaw>, SeparatorCounts), String> {
    let bytes = data.as_bytes();
    let mut i = 000;

    let mut commands: Vec<PathCommandRaw> = Vec::new();
    let mut separators = SeparatorCounts::default();

    let group_size = |c: char| -> usize {
        match c {
            'M' | 'm' | 'L' | 'l' | 'T' | 't' => 2,
            'H' | 'h' | 'V' | 'v' => 1,
            'C' | 'c' => 6,
            'S' | 's' | 'Q' | 'q' => 4,
            'A' | 'a' => 7,
            'Z' | 'z' => 0,
            _ => 0,
        }
    };

    while i < bytes.len() {
        let c = bytes[i];

        // Count explicit separators.
        match c {
            b',' => {
                separators.commas += 1;
                i += 1;
                continue;
            }
            b' ' => {
                separators.spaces += 1;
                i += 1;
                continue;
            }
            b'\t' => {
                separators.tabs += 1;
                i += 1;
                continue;
            }
            b'\n' | b'\r' => {
                separators.newlines += 1;
                i += 1;
                continue;
            }
            _ => {}
        }

        // Start a new SVG command.
        if is_command(c) {
            commands.push(PathCommandRaw {
                command: c as char,
                values: Vec::new(),
            });

            i += 1;
            continue;
        }

        // Parse a number belonging to the current command.
        if is_number_start(c) {
            if commands.is_empty() {
                return Err(format!("Number found before any command at byte {i}"));
            }

            // A sign can separate numbers without whitespace: M10-20.
            if (c == b'+' || c == b'-')
                && i > 0
                && !bytes[i - 1].is_ascii_whitespace()
                && bytes[i - 1] != b','
                && (bytes[i - 1].is_ascii_digit() || bytes[i - 1] == b'.')
            {
                separators.implicit += 1;
            }

            let value = parse_number(bytes, &mut i)?;
            let current_command = commands.last().unwrap().command;
            let size = group_size(current_command);

            if size == 0 {
                return Err(format!(
                    "Command '{}' cannot accept numeric parameters",
                    current_command
                ));
            }

            let current = commands.last_mut().unwrap();
            current.values.push(value);

            if current.values.len() == size {
                continue;
            }

            if current.values.len() > size {
                // Split the next complete parameter group into its own command.
                let values = std::mem::take(&mut current.values);
                let (complete, remainder) = values.split_at(size);

                current.values = complete.to_vec();

                // Subsequent M/m coordinate pairs are L/l commands.
                let next_command = match current_command {
                    'M' => 'L',
                    'm' => 'l',
                    other => other,
                };

                commands.push(PathCommandRaw {
                    command: next_command,
                    values: remainder.to_vec(),
                });

                separators.implicit += 1;
            }

            continue;
        }

        return Err(format!("Unexpected character '{}' at byte {i}", c as char));
    }

    // Validate parameter group sizes.
    for command in &commands {
        let size = group_size(command.command);

        if size > 0 && command.values.len() != size {
            return Err(format!(
                "Command '{}' has an incomplete parameter group: expected {}, got {}",
                command.command,
                size,
                command.values.len()
            ));
        }
    }

    Ok((commands, separators))
}

fn svg_to_path(data: &str, scale: f32, offset: (f32, f32)) -> Result<Vec<PathCommand>, String> {
    let svg_commands = parse_svg_path(data)?.0;

    let mut path_commands = Vec::new();

    let mut last_point = (0.0f32, 0.0f32);
    let mut most_recent_move = (0.0f32, 0.0f32);
    let mut last_cp2 = (0.0f32, 0.0f32);

    let transform_point = |point: (f32, f32)| -> (f32, f32) {
        ((point.0 + offset.0) * scale, (point.1 + offset.1) * scale)
    };

    for command in svg_commands.iter() {
        match command.command {
            'M' => {
                let point = (command.values[0] as f32, command.values[1] as f32);
                most_recent_move = point;
                path_commands.push(PathCommand::MoveTo(transform_point(point)));
                last_point = point;
            }
            'm' => {
                let point = (
                    command.values[0] as f32 + last_point.0,
                    command.values[1] as f32 + last_point.1,
                );
                most_recent_move = point;
                path_commands.push(PathCommand::MoveTo(transform_point(point)));
                last_point = point;
            }
            'L' => {
                let point = (command.values[0] as f32, command.values[1] as f32);
                path_commands.push(PathCommand::LineTo(transform_point(point)));
                last_point = point;
            }
            'l' => {
                let point = (
                    command.values[0] as f32 + last_point.0,
                    command.values[1] as f32 + last_point.1,
                );
                path_commands.push(PathCommand::LineTo(transform_point(point)));
                last_point = point;
            }
            'H' => {
                let point = (command.values[0] as f32, last_point.1);
                path_commands.push(PathCommand::LineTo(transform_point(point)));
                last_point = point;
            }
            'h' => {
                let point = (last_point.0 + command.values[0] as f32, last_point.1);
                path_commands.push(PathCommand::LineTo(transform_point(point)));
                last_point = point;
            }
            'V' => {
                let point = (last_point.0, command.values[0] as f32);
                path_commands.push(PathCommand::LineTo(transform_point(point)));
                last_point = point;
            }
            'v' => {
                let point = (last_point.0, command.values[0] as f32 + last_point.1);
                path_commands.push(PathCommand::LineTo(transform_point(point)));
                last_point = point;
            }
            'Z' | 'z' => {
                path_commands.push(PathCommand::Close);
                last_point = most_recent_move;
            }
            'Q' => {
                let ctrl = (command.values[0] as f32, command.values[1] as f32);
                let point = (command.values[2] as f32, command.values[3] as f32);
                path_commands.push(PathCommand::QuadraticTo(
                    transform_point(point),
                    transform_point(ctrl),
                ));
                last_point = point;
            }
            'q' => {
                let ctrl = (
                    last_point.0 + command.values[0] as f32,
                    last_point.1 + command.values[1] as f32,
                );
                let point = (
                    last_point.0 + command.values[2] as f32,
                    last_point.1 + command.values[3] as f32,
                );
                path_commands.push(PathCommand::QuadraticTo(
                    transform_point(point),
                    transform_point(ctrl),
                ));
                last_point = point;
            }
            'C' => {
                let cp1 = (command.values[0] as f32, command.values[1] as f32);
                let cp2 = (command.values[2] as f32, command.values[3] as f32);
                last_cp2 = cp2;
                let point = (command.values[4] as f32, command.values[5] as f32);
                path_commands.push(PathCommand::CubicTo(
                    transform_point(point),
                    transform_point(cp1),
                    transform_point(cp2),
                ));
                last_point = point;
            }
            'c' => {
                let cp1 = (
                    last_point.0 + command.values[0] as f32,
                    last_point.1 + command.values[1] as f32,
                );
                let cp2 = (
                    last_point.0 + command.values[2] as f32,
                    last_point.1 + command.values[3] as f32,
                );
                last_cp2 = cp2;
                let point = (
                    last_point.0 + command.values[4] as f32,
                    last_point.1 + command.values[5] as f32,
                );
                path_commands.push(PathCommand::CubicTo(
                    transform_point(point),
                    transform_point(cp1),
                    transform_point(cp2),
                ));
                last_point = point;
            }
            'S' => {
                let cp2 = (command.values[0] as f32, command.values[1] as f32);
                let end_point = (command.values[2] as f32, command.values[3] as f32);
                let cp1 = (
                    2.0 * last_point.0 - last_cp2.0,
                    2.0 * last_point.1 - last_cp2.1,
                );
                path_commands.push(PathCommand::CubicTo(
                    transform_point(end_point),
                    transform_point(cp1),
                    transform_point(cp2),
                ));
                last_point = end_point;
            }
            's' => {
                let cp2 = (
                    last_point.0 + command.values[0] as f32,
                    last_point.1 + command.values[1] as f32,
                );
                let end_point = (
                    last_point.0 + command.values[2] as f32,
                    last_point.1 + command.values[3] as f32,
                );
                let cp1 = (
                    2.0 * last_point.0 - last_cp2.0,
                    2.0 * last_point.1 - last_cp2.1,
                );
                path_commands.push(PathCommand::CubicTo(
                    transform_point(end_point),
                    transform_point(cp1),
                    transform_point(cp2),
                ));
                last_point = end_point;
            }
            'A' => {
                let radius_x = command.values[0] as f32 * scale;
                let radius_y = command.values[1] as f32 * scale;
                let rotation = command.values[2] as f32;
                let large_arc = command.values[3] as f32;
                let sweep = command.values[4] as f32;

                let start_point = last_point;
                let end_point = (command.values[5] as f32, command.values[6] as f32);

                append_svg_arc(
                    &mut path_commands,
                    transform_point(start_point),
                    radius_x,
                    radius_y,
                    rotation,
                    large_arc,
                    sweep,
                    transform_point(end_point),
                );

                last_point = end_point;
            }
            'a' => {
                let radius_x = command.values[0] as f32 * scale;
                let radius_y = command.values[1] as f32 * scale;
                let rotation = command.values[2] as f32;
                let large_arc = command.values[3] as f32;
                let sweep = command.values[4] as f32;

                let start_point = last_point;
                let end_point = (
                    start_point.0 + command.values[5] as f32,
                    start_point.1 + command.values[6] as f32,
                );

                append_svg_arc(
                    &mut path_commands,
                    transform_point(start_point),
                    radius_x,
                    radius_y,
                    rotation,
                    large_arc,
                    sweep,
                    transform_point(end_point),
                );

                last_point = end_point;
            }
            _ => println!("Unknown command: {}", command.command),
        }
    }

    Ok(path_commands)
}

// THIS FUNCTION WAS WRITTIN BY AI
fn append_svg_arc(
    path_commands: &mut Vec<PathCommand>,
    start: (f32, f32),
    mut rx: f32,
    mut ry: f32,
    rotation: f32,
    large_arc: f32,
    sweep: f32,
    end: (f32, f32),
) {
    use std::f32::consts::PI;

    // An arc with identical endpoints is omitted.
    if start == end {
        return;
    }

    // A zero radius produces a straight line.
    rx = rx.abs();
    ry = ry.abs();

    if rx == 0.0 || ry == 0.0 {
        path_commands.push(PathCommand::LineTo(end));
        return;
    }

    let phi = rotation.to_radians();
    let cos_phi = phi.cos();
    let sin_phi = phi.sin();

    let mid_x = (start.0 + end.0) / 2.0;
    let mid_y = (start.1 + end.1) / 2.0;

    // Step 1: Transform the endpoints into the ellipse's
    // rotated local coordinate system.
    let dx = (start.0 - end.0) / 2.0;
    let dy = (start.1 - end.1) / 2.0;

    let x1p = cos_phi * dx + sin_phi * dy;
    let y1p = -sin_phi * dx + cos_phi * dy;

    // Step 2: Correct the radii if they are too small.
    let rx2 = rx * rx;
    let ry2 = ry * ry;
    let x1p2 = x1p * x1p;
    let y1p2 = y1p * y1p;

    let lambda = x1p2 / rx2 + y1p2 / ry2;

    if lambda > 1.0 {
        let scale = lambda.sqrt();
        rx *= scale;
        ry *= scale;
    }

    let rx2 = rx * rx;
    let ry2 = ry * ry;

    // Step 3: Calculate the center in the local coordinate system.
    let numerator = (rx2 * ry2 - rx2 * y1p2 - ry2 * x1p2).max(0.0);
    let denominator = rx2 * y1p2 + ry2 * x1p2;

    let sign = if (large_arc != 0.0) == (sweep != 0.0) {
        -1.0
    } else {
        1.0
    };

    let coefficient = if denominator == 0.0 {
        0.0
    } else {
        sign * (numerator / denominator).sqrt()
    };

    let cxp = coefficient * (rx * y1p / ry);
    let cyp = coefficient * (-ry * x1p / rx);

    // Step 4: Transform the center back into the original
    // coordinate system.
    let cx = cos_phi * cxp - sin_phi * cyp + mid_x;
    let cy = sin_phi * cxp + cos_phi * cyp + mid_y;

    // Step 5: Calculate the start angle and sweep angle.
    let ux = (x1p - cxp) / rx;
    let uy = (y1p - cyp) / ry;

    let vx = (-x1p - cxp) / rx;
    let vy = (-y1p - cyp) / ry;

    let start_angle = uy.atan2(ux);

    let mut sweep_angle = (ux * vy - uy * vx).atan2(ux * vx + uy * vy);

    if sweep == 0.0 && sweep_angle > 0.0 {
        sweep_angle -= 2.0 * PI;
    } else if sweep != 0.0 && sweep_angle < 0.0 {
        sweep_angle += 2.0 * PI;
    }

    // Calculate a point on the rotated ellipse.
    let ellipse_point = |theta: f32| -> (f32, f32) {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        (
            cx + rx * cos_theta * cos_phi - ry * sin_theta * sin_phi,
            cy + rx * cos_theta * sin_phi + ry * sin_theta * cos_phi,
        )
    };

    // Calculate the derivative (tangent) at an ellipse angle.
    let ellipse_derivative = |theta: f32| -> (f32, f32) {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        (
            -rx * sin_theta * cos_phi - ry * cos_theta * sin_phi,
            -rx * sin_theta * sin_phi + ry * cos_theta * cos_phi,
        )
    };

    // Step 6: Split the arc into segments of at most 90 degrees.
    let segment_count = (sweep_angle.abs() / (PI / 2.0)).ceil().max(1.0) as usize;

    let segment_angle = sweep_angle / segment_count as f32;
    let mut theta = start_angle;

    for i in 0..segment_count {
        let next_theta = if i + 1 == segment_count {
            start_angle + sweep_angle
        } else {
            theta + segment_angle
        };

        let p0 = if i == 0 { start } else { ellipse_point(theta) };

        let p3 = if i + 1 == segment_count {
            end
        } else {
            ellipse_point(next_theta)
        };

        let alpha = (4.0 / 3.0) * (segment_angle / 4.0).tan();

        let d0 = ellipse_derivative(theta);
        let d1 = ellipse_derivative(next_theta);

        let control1 = (p0.0 + alpha * d0.0, p0.1 + alpha * d0.1);

        let control2 = (p3.0 - alpha * d1.0, p3.1 - alpha * d1.1);

        // Your CubicTo stores: endpoint, control point 1, control point 2.
        path_commands.push(PathCommand::CubicTo(p3, control1, control2));

        theta = next_theta;
    }
}

use crate::Resolution;
use crate::canvas::LayerPosition;
use crate::graphic::StraightLineFromToFn;
use crate::image::Image;

impl SVGData {
    pub(crate) fn render(
        &self,
        position: &LayerPosition,
        scale: f32,
        opacity: f32,
        final_image: &mut Image,
        resolution: &Resolution,
        straight_line_from_to: StraightLineFromToFn,
    ) {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);

        let mut edges = Vec::new();

        for element in &self.elements {
            match &element.graphic_type {
                GraphicType::Path { commands } => {
                    let edges_inner = path_to_edges(commands);
                    for &(p0, p1) in edges_inner.iter() {
                        min_y = min_y.min(p0.1).min(p1.1);
                        max_y = max_y.max(p0.1).max(p1.1);
                        min_x = min_x.min(p0.0).min(p1.0);
                        max_x = max_x.max(p0.0).max(p1.0);
                    }
                    edges.extend_from_slice(&vec![edges_inner]);
                }
                _ => (),
            }
        }

        let element_width = (max_x - min_x) * scale;
        let element_height = (max_y - min_y) * scale;

        let (target_x, target_y) =
            position.to_translation(element_width, element_height, resolution);

        let translation_x = target_x - min_x * scale;
        let translation_y = target_y - min_y * scale;

        for (i, element) in self.elements.iter().enumerate() {
            let edges = &edges[i];
            render_edges(
                edges,
                element.stroke_width,
                element.stroke_color,
                element.fill_color,
                position,
                scale,
                opacity,
                final_image,
                resolution,
                straight_line_from_to,
                Some((translation_x, translation_y)),
                // None,
            );
        }
    }
}
