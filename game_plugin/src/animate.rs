use crate::board::Board;
use crate::matcher::Slot;
use crate::{GameState, SystemLabels};
use bevy::prelude::*;

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(animate.system().label(SystemLabels::Animate)),
        );
    }
}

pub struct Animate {
    pub goal: Vec2,
    pub speed: f32,
}

impl Animate {
    pub fn move_to_slot(slot: &Slot) -> Self {
        Animate {
            goal: Vec2::new(slot.column as f32 * 64. + 32., slot.row as f32 * 64. + 32.),
            speed: 256.,
        }
    }
}

fn animate(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut animations: Query<(Entity, &mut Transform, &mut Vec<Animate>)>,
    time: Res<Time>,
) {
    let mut count = 0;
    let delta = time.delta().as_secs_f32();
    for (entity, mut transform, mut animations) in animations.iter_mut() {
        count += 1;
        let animate = animations.first().unwrap();
        let diff = animate.goal - Vec2::new(transform.translation.x, transform.translation.y);
        let movement = delta * animate.speed;
        if diff.length() < (delta * animate.speed) {
            transform.translation.x = animate.goal.x;
            transform.translation.y = animate.goal.y;
            if animations.len() == 1 {
                commands.entity(entity).remove::<Vec<Animate>>();
            } else {
                animations.remove(0);
            }
        } else {
            let movement = diff.normalize() * movement;
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
    board.animating = count > 0;
}
