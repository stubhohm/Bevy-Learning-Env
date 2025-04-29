use bevy::prelude::*;
use bevy::ecs::schedule::SystemSet;

#[derive(Component, Deref, DerefMut)]
pub struct CPosition(pub Vec2);

#[derive(Component)]
pub struct CName(pub String);

#[derive(Component)]
pub struct CHealth(pub i8);

#[derive(Component)]
pub struct  CScoreboardUi;

#[derive(Component)]
pub struct CShip;

#[derive(Component)]
pub struct CMissile;

#[derive(Component)]
pub struct CPlayer;

#[derive(Component)]
pub struct CAlien;

#[derive(Component)]
pub struct CButterfly;

#[derive(Component)]
pub struct CBumblebee;

#[derive(Component)]
pub struct CGalaga;

#[derive(Component)]
pub struct CCapturedFighter;

#[derive(Component)]
pub struct CStar{pub depth: f32}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StartupSet {
    LoadAssets,
    SpawnActors,
}

#[derive(Component, PartialEq)]
pub enum AlienStates {
    Attacking,
    Entering,
    Idling,
    Abducting,
}

#[derive(Component)]
pub struct CVelocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Copy)]
pub struct BezierSegment {
    pub p0: Vec2,
    pub p1: Vec2,
    pub p2: Vec2,
    pub p3: Vec2,
}

impl BezierSegment {
    pub fn tangent_at(&self, t: f32) -> Vec2 {
        let inv_t = 1.0 - t;
        3.0 * inv_t * inv_t * (self.p1 - self.p0) +
        6.0 * inv_t * t * (self.p2 - self.p1) +
        3.0 * t * t * (self.p3 - self.p2)
    }

    pub fn sample(&self, t: f32) -> Vec2 {
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;

        uuu * self.p0
            + 3.0 * uu * t * self.p1
            + 3.0 * u * tt * self.p2
            + ttt * self.p3
    }
}

/// A full path made of one or more connected Bézier segments
#[derive(Component)]
pub struct BezierPath {
    pub segments: Vec<BezierSegment>,
    pub duration: f32,     // total time to travel the path
    pub elapsed: f32,      // current time along the path
}

impl BezierPath {
    pub fn mirror_x(&self) -> Self {
        let mirrored_segements = 
        self.segments.iter().map(|segment| {
             BezierSegment {
                p0: Vec2 { x: (-segment.p0.x), y: (segment.p0.y) },
                p1: Vec2 { x: (-segment.p1.x), y: (segment.p1.y) },
                p2: Vec2 { x: (-segment.p2.x), y: (segment.p2.y) },
                p3: Vec2 { x: (-segment.p3.x), y: (segment.p3.y) },

            }
        }).collect();
        Self { segments: (mirrored_segements), duration: (self.duration), elapsed: (self.elapsed) }
    }
}