use bevy::prelude::*;
use pokimans_common::game::map;


#[derive(Resource)]
pub struct GrassTextureAtlas(Handle<TextureAtlas>);

pub fn setup_map_renderer(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    
    let image = assets.load("grass.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(16.0), 3, 3, None, None);

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(GrassTextureAtlas(atlas_handle));
}

pub fn render_map(mut commands: Commands, grass_tex: Res<GrassTextureAtlas>, map: Res<map::Map>) {
    let chunk = map.chunks.get(0).unwrap(); //only 1 chunk for now
    
    for cell in chunk {
	let transform = Transform::from_translation(Vec3::new(cell.position.x as f32, cell.position.y as f32, 0.0));
	commands.spawn(SpriteSheetBundle { 
	    texture_atlas: grass_tex.0.clone(),
	    sprite: TextureAtlasSprite {
		index: compute_atlas_index(&cell.position),
		anchor: bevy::sprite::Anchor::BottomLeft,
		custom_size: Some(Vec2::splat(1.0)),
		..default()
	    },
	    transform,
	    ..default()
	});
    } 
}
fn compute_atlas_index(pos: &pokimans_common::utils::Position) -> usize {
    let x = pos.x;
    let y = pos.y;
    if y == 0 {
	if x == 0 { 6 }
	else if x == map::CHUNK_SIZE - 1 { 8 } 
	else { 7 }
    } else if y == map::CHUNK_SIZE - 1 {
	if x == 0 { 0 }
	else if x == map::CHUNK_SIZE - 1 { 2 } 
	else { 1 }
    } else {
	if x == 0 { 3 }
	else if x == map::CHUNK_SIZE - 1 { 5 } 
	else { 4 }
    }
}