use anyhow::Result;
use bevy::{
    prelude::{
        App, AssetServer, Camera2dBundle, Commands, Deref, DerefMut, Entity, EventReader, Handle,
        Image, Input, MouseButton, Res, ResMut, SystemSet, Transform, Vec2, Vec3, World,
    },
    render::texture::ImageSettings,
    sprite::SpriteBundle,
    time::{FixedTimestep, Time},
    transform::TransformBundle,
    window::{Window, WindowDescriptor, WindowResized, Windows},
    DefaultPlugins,
};
use bevy_rapier2d::prelude::{
    CoefficientCombineRule, Collider, RapierPhysicsPlugin, Restitution, RigidBody,
};



#[allow(dead_code)]
fn main() {
    run().unwrap();
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    run().unwrap();
}

const PIX_PER_M: f32 = 20.0;

fn spawn_stuff(mut c: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();
    c.spawn_bundle(camera);
    let texture = asset_server.load("sprites/bad_wojak_sprite.png");
    c.insert_resource(WojakTexture(texture));

    let edges = c
        .spawn()
        .insert_bundle(TransformBundle::from_transform(Transform {
            translation: Vec3::ZERO,
            ..Default::default()
        }))
        .insert(RigidBody::Fixed)
        .insert(Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Collider::polyline(
            vec![
                Vec2::new(-1.0, -1.0),
                Vec2::new(-1.0, 1.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(1.0, -1.0),
            ],
            Some(vec![[0, 1], [1, 2], [2, 3], [3, 0]]),
        ))
        .id();
    c.insert_resource(Edges { id: edges });

    let cursor = c
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert_bundle(TransformBundle::from_transform(Transform {
            translation: Vec3::ZERO,
            ..Default::default()
        }))
        .insert(Collider::ball(10.0))
        .id();
    c.insert_resource(CursorId(cursor));
}

fn wojak_spawner(
    mut c: Commands,
    _time: Res<Time>,
    mouse: Res<Input<MouseButton>>,
    tex: Res<WojakTexture>,
    windows: ResMut<Windows>,
) {
    let scale = 2.0;
    if mouse.pressed(MouseButton::Left) {
        if let Ok(cursor) = Cursor::try_from(windows.get_primary().unwrap()) {
            c.spawn_bundle(SpriteBundle {
                texture: tex.0.clone(),
                transform: Transform {
                    translation: Vec3::new(cursor.x, cursor.y, 0.0), // TODO: +=rng
                    scale: Vec3::splat(scale / PIX_PER_M * 20.0),    // spirte is 20 pix in size
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            // .insert(LockedAxes::ROTATION_LOCKED)
            // .insert(Ccd::enabled())
            .insert(Restitution::coefficient(0.9))
            .insert(Collider::ball(PIX_PER_M / 2.0)); // ? eh, should *= scale too. but no ig?
        }
    }
}

fn update_edge_colliders(
    mut c: Commands,
    mut resized: EventReader<WindowResized>,
    edges: Res<Edges>,
) {
    let edges_id = edges.id;
    let mut hv = Vec2::new(0.0, 0.0);
    for i in resized.iter() {
        hv = Vec2::new(i.width, i.height);
    }
    c.add(move |w: &mut World| {
        let mut e = w.entity_mut(edges_id);
        let pos = e.get::<Transform>().unwrap().translation;
        let pos = Vec2::new(pos.x, pos.y);
        if hv.x * hv.y > 0.0 {
            hv = hv / 2.0;
            let vertices = vec![
                pos - hv,
                Vec2::new(pos.x - hv.x, pos.y + hv.y),
                pos + hv,
                Vec2::new(pos.x + hv.x, pos.y - hv.y),
            ];
            let mut c = e.get_mut::<Collider>().unwrap();
            *c = Collider::polyline(vertices, Some(vec![[0, 1], [1, 2], [2, 3], [3, 0]]));
        }
    });
}

// TODO: also needs velocity: CursorMoved
fn update_cursor(mut c: Commands, cursor: ResMut<CursorId>, windows: ResMut<Windows>) {
    let win = windows.get_primary().unwrap();
    if let Ok(cur) = Cursor::try_from(win) {
        let id = cursor.0;
        c.add(move |w: &mut World| {
            w.entity_mut(id).get_mut::<Transform>().unwrap().translation =
                Vec3::new(cur.x, cur.y, 0.0);
        });
    }
}

// fn dbg_cursor(windows: ResMut<Windows>, mouse: Res<Input<MouseButton>>) {
//     if mouse.any_pressed([MouseButton::Right]) {
//         let c = Cursor::from(windows.get_primary().unwrap());
//         dbg!(c);
//     }
// }

pub fn run() -> Result<()> {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "cheso".to_string(),
        ..WindowDescriptor::default()
    })
    .add_system(bevy::window::close_on_esc)
    .insert_resource(ImageSettings::default_nearest())
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<()>::pixels_per_meter(PIX_PER_M))
    // .insert_resource(RapierConfiguration {
    //     gravity: Vec2::new(0.0, -98.0),
    //     ..Default::default()
    // })
    .add_startup_system(spawn_stuff)
    .add_system(update_cursor)
    // .add_system(dbg_cursor)
    .add_system(update_edge_colliders)
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.01))
            .with_system(wojak_spawner),
    )
    .run();
    Ok(())
}

#[derive(Debug, Default, DerefMut, Deref)]
pub struct Cursor(Vec2);
impl TryFrom<&Window> for Cursor {
    type Error = ();
    fn try_from(w: &Window) -> std::result::Result<Self, ()> {
        if let Some(c) = w.cursor_position() {
            Ok(Self(c - Vec2::new(w.width() / 2.0, w.height() / 2.0)))
        } else {
            Err(())
        }
    }
}

pub struct CursorId(Entity);
pub struct Edges {
    id: Entity,
}
pub struct WojakTexture(Handle<Image>);
