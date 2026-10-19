use crate::graphic::PathCommand;
#[derive(Debug)]
pub struct PathCommandRaw {
    pub command: char,
    pub values: Vec<f64>,
}

#[derive(Debug, Default)]
pub struct SeparatorCounts {
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
pub fn parse_svg_path(data: &str) -> Result<(Vec<PathCommandRaw>, SeparatorCounts), String> {
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

pub fn svg_to_path(data: &str) -> Result<Vec<PathCommand>, String> {
    let svg_commands = parse_svg_path(data)?.0;

    let mut path_commands = Vec::new();

    let mut last_point = (00f32, 0f32);
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for command in svg_commands.iter() {
        match command.command {
            'M' => {
                let point = (command.values[0] as f32, command.values[1] as f32);
                if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 < min_y {
                    min_y = point.1;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
                path_commands.push(PathCommand::MoveTo(point));
                last_point = point;
            }
            'm' => {
                let point = (
                    (command.values[0] as f32 + last_point.0),
                    (command.values[1] as f32 + last_point.1),
                );
                if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 < min_y {
                    min_y = point.1;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
                path_commands.push(PathCommand::MoveTo(point));
                last_point = point;
            }
            'L' => {
                let point = (command.values[0] as f32, command.values[1] as f32);
                if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 < min_y {
                    min_y = point.1;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
                path_commands.push(PathCommand::LineTo(point));
                last_point = point;
            }
            'l' => {
                let point = (
                    (command.values[0] as f32 + last_point.0),
                    (command.values[1] as f32 + last_point.1),
                );
                if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 < min_y {
                    min_y = point.1;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
                path_commands.push(PathCommand::LineTo(point));
                last_point = point;
            }
            'H' => {
                let point = (command.values[0] as f32, last_point.1);
                if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 < min_y {
                    min_y = point.1;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
                path_commands.push(PathCommand::LineTo(point));
                last_point = point;
            }
            'h' => {
                let point = ((last_point.0 + command.values[0] as f32), last_point.1);
                if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 < min_y {
                    min_y = point.1;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
                path_commands.push(PathCommand::LineTo(point));
                last_point = point;
            }
            // V take only one value which is y
            'V' => {
                let point = (last_point.0, command.values[0] as f32);
                if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 < min_y {
                    min_y = point.1;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
                path_commands.push(PathCommand::LineTo(point));
                last_point = point;
            }
            'v' => {
                let point = (last_point.0, command.values[0] as f32 + last_point.1);
                if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 < min_y {
                    min_y = point.1;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
                path_commands.push(PathCommand::LineTo(point));
                last_point = point;
            }
            'Z' | 'z' => {
                let mut most_recent_move_to = (0f32, 0f32);
                for path_command in path_commands.iter().rev() {
                    if let PathCommand::MoveTo(point) = path_command {
                        most_recent_move_to = *point;
                        break;
                    }
                }
                path_commands.push(PathCommand::Close);
                last_point = most_recent_move_to;
                if last_point.0 < min_x {
                    min_x = last_point.0;
                }
                if last_point.1 < min_y {
                    min_y = last_point.1;
                }
                if last_point.1 > max_y {
                    max_y = last_point.1;
                }
            }
            'Q' => {
                let point = (command.values[2] as f32, command.values[3] as f32);
                path_commands.push(PathCommand::QuadraticTo(
                    (command.values[0] as f32, command.values[1] as f32),
                    (command.values[2] as f32, command.values[3] as f32),
                ));
                if point.0 < min_x {
                    min_x = point.0;
                }
                last_point = point;
                if last_point.1 < min_y {
                    min_y = last_point.1;
                }
                if last_point.1 > max_y {
                    max_y = last_point.1;
                }
            }
            'q' => {
                let point = (
                    last_point.0 + command.values[2] as f32,
                    last_point.1 + command.values[3] as f32,
                );
                if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 < min_y {
                    min_y = point.1;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
                path_commands.push(PathCommand::QuadraticTo(
                    (
                        last_point.0 + command.values[0] as f32,
                        last_point.1 + command.values[1] as f32,
                    ),
                    point,
                ));
                last_point = point;
            }
            'C' => {
                let point = (command.values[4] as f32, command.values[5] as f32);
                if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 < min_y {
                    min_y = point.1;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
                path_commands.push(PathCommand::CubicTo(
                    point,
                    (command.values[0] as f32, command.values[1] as f32),
                    (command.values[2] as f32, command.values[3] as f32),
                ));
                last_point = point;
            }
            'c' => {
                let point = (
                    (last_point.0 + command.values[4] as f32),
                    (last_point.1 + command.values[5] as f32),
                );
                if point.0 < min_x {
                    min_x = point.0;
                }
                if point.1 < min_y {
                    min_y = point.1;
                }
                if point.1 > max_y {
                    max_y = point.1;
                }
                path_commands.push(PathCommand::CubicTo(
                    point,
                    (
                        (last_point.0 + command.values[0] as f32),
                        (last_point.1 + command.values[1] as f32),
                    ),
                    (
                        (last_point.0 + command.values[2] as f32),
                        (last_point.1 + command.values[3] as f32),
                    ),
                ));
                last_point = point;
            }
            'S' => {
                let end_point = (command.values[2] as f32, command.values[3] as f32);
                let cp2 = (command.values[0] as f32, command.values[1] as f32);
                let cp1 = match path_commands.last() {
                    Some(PathCommand::CubicTo(_, _, last_cp2)) => (
                        2.0 * last_point.0 - last_cp2.0,
                        2.0 * last_point.1 - last_cp2.1,
                    ),
                    _ => last_point,
                };
                path_commands.push(PathCommand::CubicTo(end_point, cp1, cp2));
                last_point = end_point;
                if last_point.0 < min_x {
                    min_x = last_point.0;
                }
                if last_point.1 < min_y {
                    min_y = last_point.1;
                }
                if last_point.1 > max_y {
                    max_y = last_point.1;
                }
            }
            's' => {
                let end_point = (
                    last_point.0 + command.values[2] as f32,
                    last_point.1 + command.values[3] as f32,
                );
                let cp2 = (
                    last_point.0 + command.values[0] as f32,
                    last_point.1 + command.values[1] as f32,
                );
                let cp1 = match path_commands.last() {
                    Some(PathCommand::CubicTo(_, _, last_cp2)) => (
                        2.0 * last_point.0 - last_cp2.0,
                        2.0 * last_point.1 - last_cp2.1,
                    ),
                    _ => last_point,
                };
                path_commands.push(PathCommand::CubicTo(end_point, cp1, cp2));
                last_point = end_point;
                if last_point.0 < min_x {
                    min_x = last_point.0;
                }
                if last_point.1 < min_y {
                    min_y = last_point.1;
                }
                if last_point.1 > max_y {
                    max_y = last_point.1;
                }
            }

            // BOTH THOSE WERE WRITTEN BY AI
            'A' => {
                let radius_x = command.values[0] as f32;
                let radius_y = command.values[1] as f32;
                let rotation = command.values[2] as f32;
                let large_arc = command.values[3] as f32;
                let sweep = command.values[4] as f32;

                let end_point = (command.values[5] as f32, command.values[6] as f32);

                append_svg_arc(
                    &mut path_commands,
                    last_point,
                    radius_x,
                    radius_y,
                    rotation,
                    large_arc,
                    sweep,
                    end_point,
                );

                last_point = end_point;
                if last_point.0 < min_x {
                    min_x = last_point.0;
                }
                if last_point.1 < min_y {
                    min_y = last_point.1;
                }
                if last_point.1 > max_y {
                    max_y = last_point.1;
                }
            }
            'a' => {
                let radius_x = command.values[0] as f32;
                let radius_y = command.values[1] as f32;
                let rotation = command.values[2] as f32;
                let large_arc = command.values[3] as f32;
                let sweep = command.values[4] as f32;

                // Lowercase 'a' uses a relative endpoint.
                let end_point = (
                    last_point.0 + command.values[5] as f32,
                    last_point.1 + command.values[6] as f32,
                );

                append_svg_arc(
                    &mut path_commands,
                    last_point,
                    radius_x,
                    radius_y,
                    rotation,
                    large_arc,
                    sweep,
                    end_point,
                );

                last_point = end_point;
                if last_point.0 < min_x {
                    min_x = last_point.0;
                }
                if last_point.1 < min_y {
                    min_y = last_point.1;
                }
                if last_point.1 > max_y {
                    max_y = last_point.1;
                }
            }

            _ => println!("Unknown command: {}", command.command),
        }
    }
    let height = max_y + min_y;
    println!("min_x: {}, min_y: {}, height: {}", min_x, min_y, height);
    let mut path_commands_flipped = vec![];
    let scale = 1.0;
    let translate_x = -min_x;
    let translate_y = -min_y;

    let transform = |x: f32, y: f32| -> (f32, f32) {
        (
            (x + translate_x) * scale,
            (height - y + translate_y) * scale,
        )
    };

    for command in path_commands {
        match command {
            PathCommand::LineTo((x, y)) => {
                path_commands_flipped.push(PathCommand::LineTo(transform(x, y)));
            }
            PathCommand::CubicTo((x1, y1), (x2, y2), (x3, y3)) => {
                path_commands_flipped.push(PathCommand::CubicTo(
                    transform(x1, y1),
                    transform(x2, y2),
                    transform(x3, y3),
                ));
            }
            PathCommand::QuadraticTo((x1, y1), (x2, y2)) => {
                path_commands_flipped.push(PathCommand::QuadraticTo(
                    transform(x1, y1),
                    transform(x2, y2),
                ));
            }
            PathCommand::MoveTo((x, y)) => {
                path_commands_flipped.push(PathCommand::MoveTo(transform(x, y)));
            }
            _ => path_commands_flipped.push(command),
        }
    }

    Ok(path_commands_flipped)
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
