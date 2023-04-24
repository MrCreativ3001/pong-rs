use crate::game_state::play::PlayState;

#[test]
fn test_box_box_collision() {
    assert_eq!(
        PlayState::is_box_colliding_with_box(0.0, 0.0, 1.0, 1.0, 0.25, 0.25, 0.5, 0.5),
        true
    );

    assert_eq!(
        PlayState::is_box_colliding_with_box(2.0, 2.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0),
        false
    );
}
