use heron::prelude::*;

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Player,
    World,
    Tower,
    Bullet,
}
