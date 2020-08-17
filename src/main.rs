use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

const MAX_LEVEL: usize = 11;

struct Racket {
    name: String,
    store: usize,
    key1: KeyCode,
    key2: KeyCode,
}

struct Ball {
    velocity: Vec3,
}

enum Collider {
    Solid,
    Player,
}
#[derive(PartialEq, Eq)]
enum Player {
    User1 = 0x01,
    User2 = 0x02,
}

struct EventTriggerState {
    event_timer: Timer,
}

impl Default for EventTriggerState {
    fn default() -> Self {
        EventTriggerState {
            event_timer: Timer::from_seconds(2.0),
        }
    }
}

// use std::time::Duration;

fn main() {
    App::build()
        .add_default_plugins()
        .add_event::<MyEvent>()
        .init_resource::<EventTriggerState>()
        .init_resource::<EventListenerState>()
        .add_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        // .add_resource(GreetTimer(Timer::from_seconds(5.0)))
        .add_startup_system(setup.system())
        .add_system(racket_movement_system.system())
        .add_system(ball_movement_system.system())
        .add_system(scoreboard_system.system())
        .add_system(ball_collision_system.system())
        .add_system(event_listener_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {

    let font_kuhei = asset_server.load("assets/fonts/站酷酷黑体.ttf").unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteComponents{
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite { 
                size: Vec2::new(5.0, 5.0),
            },
            ..Default::default()
        })
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            translation: Translation(Vec3::new(-380.0, 10.0, 0.0)),
            sprite: Sprite { 
                size: Vec2::new(30.0, 120.0),
            },
            ..Default::default()
        })
        .with(Racket{
            name:"用户A".to_string(),
            store:0,
            key1: KeyCode::A,
            key2: KeyCode::D,
        })
        .with(Collider::Player)
        .with(Player::User1)
        // 用户2
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            translation: Translation(Vec3::new(380.0, 10.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(30.0, 120.0),
            },
            ..Default::default()
        })
        .with(Racket{
            name:"用户B".to_string(),
            store:0,
            key1: KeyCode::Left,
            key2: KeyCode::Right,
        })
        .with(Collider::Player)
        .with(Player::User2)
        // 球
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
            translation: Translation(Vec3::new(0.0, -50.0, 1.0)),
            sprite: Sprite {
                size: Vec2::new(30.0, 30.0),
            },
            ..Default::default()
        })
        .with(Ball {
            // velocity:Vec3::new(0.0,0.0,0.0).normalize(),
            velocity: 400.0 * Vec3::new(-0.5, 0.5, 0.0).normalize(),
        })
        // 记分板
        .spawn(TextComponents {
            text: Text {
                font: font_kuhei,
                value: "Score:".to_string(),
                style: TextStyle {
                    color: Color::rgb(0.2, 0.2, 0.8).into(),
                    font_size: 40.0,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Player::User1)
        .spawn(TextComponents {
            text: Text {
                font: font_kuhei, 
                value: "Score:".to_string(),
                style: TextStyle {
                    color: Color::rgb(1.0, 0.2, 0.8).into(),
                    font_size: 40.0,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Player::User2)
        // ...
        ;

    // Add walls
    let wall_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    commands
        // left
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(-bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Solid)
        // right
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Solid)
        // bottom
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, -bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Solid)
        // top
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Solid);
}

// 球拍移动
fn racket_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Racket, &mut Translation)>,
) {
    for (racket, mut translation) in &mut query.iter() {
        let mut direction = 0.0;
        if keyboard_input.pressed(racket.key1) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(racket.key2) {
            direction += 1.0;
        }

        *translation.0.y_mut() += time.delta_seconds * direction * 500.0;

        // bound the racket within the walls
        *translation.0.y_mut() = f32::max(-180.0, f32::min(180.0, translation.0.y()));
    }
}

// 球 移动
fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Translation)>) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds);

    for (ball, mut translation) in &mut ball_query.iter() {
        translation.0 += ball.velocity * delta_seconds;
    }
}

// 球 碰撞

fn ball_collision_system(
    mut my_events: ResMut<Events<MyEvent>>,
    mut ball_query: Query<(&mut Ball, &Translation, &Sprite)>,
    mut collider_query: Query<(Entity, &Collider, &Translation, &Sprite)>,
) {
    // let delta_seconds = f32::min(0.2, time.delta_seconds);
    for (mut ball, ball_translation, sprite) in &mut ball_query.iter() {
        let ball_size = sprite.size;
        let velocity = &mut ball.velocity;

        // check collision with walls
        for (_collider_entity, collider, translation, sprite) in &mut collider_query.iter() {
            let collision = collide(ball_translation.0, ball_size, translation.0, sprite.size);
            if let Some(collision) = collision {
                // scorable colliders should be despawned and increment the scoreboard on collision
                if let &Collider::Solid = collider {
                    // 标记1分 event
                    match collision {
                        Collision::Left => my_events.send(MyEvent::Reset(Player::User1)),
                        Collision::Right => my_events.send(MyEvent::Reset(Player::User2)),
                        _ => {}
                    }
                };

                // 弹跳处理

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the collision
                match collision {
                    Collision::Left => reflect_x = velocity.x() > 0.0,
                    Collision::Right => reflect_x = velocity.x() < 0.0,
                    Collision::Top => reflect_y = velocity.y() < 0.0,
                    Collision::Bottom => reflect_y = velocity.y() > 0.0,
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    *velocity.x_mut() = -velocity.x();
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    *velocity.y_mut() = -velocity.y();
                }

                break;
            }
        }
    }
}

enum MyEvent {
    Reset(Player),
    Over,
}

#[derive(Default)]
struct EventListenerState {
    my_event_reader: EventReader<MyEvent>,
}

// prints events as they come in
fn event_listener_system(
    mut listener: ResMut<EventListenerState>,
    my_events: Res<Events<MyEvent>>,
    mut players: Query<(&mut Racket, &Player)>,
    mut balls: Query<(&mut Ball, &mut Translation)>,
) {
    for my_event in listener.my_event_reader.iter(&my_events) {
        println!("listen");
        // println!("{}", my_event.message);
        for (mut racket, _) in &mut players.iter() {
            match my_event {
                MyEvent::Reset(p) => {
                    racket.store += 1;
                    if racket.store == 3 {
                        // todo over
                    }
                    let mut y = rand::random::<f32>();
                    let mut x = rand::random::<f32>();
                    if *p == Player::User1 {
                        y = -y;
                        x = -x;
                    }
                    for (mut ball, mut trans) in &mut balls.iter() {
                        trans.0 = Vec3::new(0.0, 50.0, 0.0);
                        ball.velocity = 400.0 * Vec3::new(x, y, 0.0).normalize();
                    }
                }
                MyEvent::Over => {
                    println!("over");
                }
            }
        }
    }
}

// fn reset_system(
//     time: Res<Time>,
//     mut trigger: ResMut<EventTriggerState>,
//     mut listener: ResMut<EventListenerState>,
//     my_events: Res<Events<MyEvent>>,
//     mut balls: Query<&mut Ball>,
// ) {
//     for my_event in listener.my_event_reader.iter(&my_events) {
//         println!("over");
//         trigger.event_timer.tick(time.delta_seconds);
//         if trigger.event_timer.finished {
//             for mut ball in &mut balls.iter() {
//                 // if my_event. == Player::User1 {
//                 //     ball.velocity = 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize();
//                 // } else {
//                 //     ball.velocity = 400.0 * Vec3::new(0.5, 0.5, 0.0).normalize();
//                 // }
//             }
//         }
//     }
// }

// 积分
fn scoreboard_system(
    mut q_racket: Query<(&Racket, &Player)>,
    mut q_text: Query<(&mut Text, &Player)>,
) {
    for (mut text, p1) in &mut q_text.iter() {
        for (racket, p2) in &mut q_racket.iter() {
            if p1 == p2 {
                text.value = format!("{}:{}", racket.name, racket.store);
            }
        }
    }
}
