use bevy::prelude::Vec2;

use crate::mods::components::*;
use crate::{HEIGHT, WIDTH};


static bezier_segment_1:BezierSegment = BezierSegment {
    p0: Vec2::new(-WIDTH*0.6, -HEIGHT * 0.45),   // Start left-bottom
    p1: Vec2::new(-WIDTH*0.25, -HEIGHT * 0.45),    // Curve up
    p2: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.25),     // Curve up
    p3: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.125),    // End mid left-bottom-going up
};

static bezier_segment_2:BezierSegment = BezierSegment {
    p0: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.125),   // Start mid-left-going up
    p1: Vec2::new(-WIDTH*0.125, 0.0),    // Curve up
    p2: Vec2::new(-300.0, 0.0),     // Curve down
    p3: Vec2::new(-300.0, -HEIGHT * 0.125),    // End far-left-going down
};

static bezier_segment_3:BezierSegment = BezierSegment {
    p0: Vec2::new(-300.0, -HEIGHT * 0.125),   // Start far-left-going down
    p1: Vec2::new(-300.0, -HEIGHT * 0.25),    // Curve down
    p2: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.25),     // Curve up
    p3: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.125),    // End mid left-bottom-going up
};

static bezier_segment_4:BezierSegment = BezierSegment {
    p0: Vec2::new(-WIDTH*0.125, -HEIGHT * 0.125),   // Start mid-left-going up
    p1: Vec2::new(-WIDTH*0.125, 0.0),    // Curve up
    p2: Vec2::new(-50.0, 50.0),     // Curve up and straighten
    p3: Vec2::new(-50.0, 150.0),    // End center going up
};

pub fn lower_left_looping_entry_path() -> BezierPath {
    BezierPath {
    segments: vec![bezier_segment_1, bezier_segment_2, bezier_segment_3, bezier_segment_4],
    duration: 4.0,
    elapsed: 0.0,
    }
}