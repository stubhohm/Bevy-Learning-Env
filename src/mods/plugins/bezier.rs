use bevy::prelude::*;

use crate::mods::components::*;
use crate::mods::resources::*;
use crate::AlienStates;

pub struct BezierCurvePlugin;

impl Plugin for BezierCurvePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_bezier_path);
        app.add_systems(Update,debug_draw_bezier);
    }
}

fn follow_bezier_path(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BezierPath, &mut AlienStates), (With<CAlien>, With<CShip>)>,
) {
    for (mut transform, mut path, state) in query.iter_mut() {
        if *state == AlienStates::Idling {
            continue;
        }
        path.elapsed += time.delta_seconds();

        let t_total = (path.elapsed / path.duration).clamp(0.0, 1.0);
        let num_segments = path.segments.len();
        if num_segments == 0 {
            continue;
        }

        let seg_f = t_total * num_segments as f32;
        let seg_index = seg_f.floor() as usize;
        let seg_t = seg_f.fract();

        if seg_index >= num_segments {
            continue;
        }

        let tangent = path.segments[seg_index].tangent_at(seg_t).normalize();
        let angle = tangent.y.atan2(tangent.x) + std::f32::consts::FRAC_PI_2;

        let BezierSegment { p0, p1, p2, p3 } = &path.segments[seg_index];
        let point = cubic_bezier(*p0, *p1, *p2, *p3, seg_t);

        // Apply the new position (2D)
        transform.translation.x = point.x;
        transform.translation.y = point.y;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn cubic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let u = 1.0 - t;
    u * u * u * p0 +
    3.0 * u * u * t * p1 +
    3.0 * u * t * t * p2 +
    t * t * t * p3
}

fn debug_draw_bezier(
    debug: Res<RDebugBezier>,
    paths: Query<&BezierPath>,
    mut gizmos: Gizmos,
) {
    if !debug.0 { return; }

    for path in paths.iter() {
        for curve in path.segments.iter() {
            let p0 = curve.p0.extend(0.0);
            let p1 = curve.p1.extend(0.0);
            let p2 = curve.p2.extend(0.0);
            let p3 = curve.p3.extend(0.0);

            // Draw control lines
            gizmos.line(p0, p1, Color::GRAY);
            gizmos.line(p2, p3, Color::GRAY);

            // Draw curve
            let segments = 20;
            let mut prev = p0;
            for i in 1..=segments {
                let t = i as f32 / segments as f32;
                let next = curve.sample(t).extend(0.0);
                gizmos.line(prev, next, Color::YELLOW);
                prev = next;
            }

            // Draw points
            gizmos.circle(p0, Direction3d::Z, 5.0, Color::GREEN);
            gizmos.circle(p1, Direction3d::Z, 5.0, Color::BLUE);
            gizmos.circle(p2, Direction3d::Z, 5.0, Color::BLUE);
            gizmos.circle(p3, Direction3d::Z, 5.0, Color::RED);
        }
    }
}