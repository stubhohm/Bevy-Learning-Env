use bevy::prelude::Vec2;

use crate::mods::components::*;
use crate::{HEIGHT, WIDTH};


static BEZIER_SEGMENT_1:BezierSegment = BezierSegment {
    p0: Vec2::new(-WIDTH*0.6, -HEIGHT * 0.45),   // Start left-bottom
    p1: Vec2::new(-WIDTH*0.25, -HEIGHT * 0.45),    // Curve up
    p2: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.25),     // Curve up
    p3: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.125),    // End mid left-bottom-going up
};

static BEZIER_SEGMENT_2:BezierSegment = BezierSegment {
    p0: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.125),   // Start mid-left-going up
    p1: Vec2::new(-WIDTH*0.125, 0.0),    // Curve up
    p2: Vec2::new(-WIDTH*0.45, 0.0),     // Curve down
    p3: Vec2::new(-WIDTH*0.45, -HEIGHT * 0.125),    // End far-left-going down
};

static BEZIER_SEGMENT_3:BezierSegment = BezierSegment {
    p0: Vec2::new(-WIDTH*0.45, -HEIGHT * 0.125),   // Start far-left-going down
    p1: Vec2::new(-WIDTH*0.45, -HEIGHT * 0.25),    // Curve down
    p2: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.25),     // Curve up
    p3: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.125),    // End mid left-bottom-going up
};

static BEZIER_SEGMENT_4:BezierSegment = BezierSegment {
    p0: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.125),   // Start mid-left-going up
    p1: Vec2::new(-WIDTH*0.125, 0.0),    // Curve up
    p2: Vec2::new(-WIDTH*0.065, HEIGHT*0.18),     // Curve up and straighten
    p3: Vec2::new(-WIDTH*0.065, HEIGHT*0.25),    // End center going up
};

pub fn lower_left_looping_entry_path(platoon_position:i8) -> BezierPath {
    BezierPath {
    segments: vec![BEZIER_SEGMENT_1, BEZIER_SEGMENT_2, BEZIER_SEGMENT_3, BEZIER_SEGMENT_4],
    duration: 3.0,
    elapsed: (-1.0 - (0.125 * platoon_position as f32)),
    }
}