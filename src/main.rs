//! Displays a single [`Sprite`], created from an image.

use bevy::{prelude::*, render::{render_resource::*, texture::ImageSampler}};
use rand::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_image)
        .add_system(game_of_life)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let size = Extent3d {
        width: 480,
        height: 480,
        ..default()
    };    

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        sampler_descriptor: ImageSampler::Descriptor(SamplerDescriptor {
            mag_filter: FilterMode::Nearest,
            ..default()
        }),
        ..default()
    };

    image.resize(size);

    let mut v = vec![false; (size.width * size.height) as usize];

    for i in 0..v.len() {
        v[i] = random();
    }

    let image_handle = images.add(image);

    commands.insert_resource(GameOfLife {image: image_handle.clone(), x: size.width as usize, /*y: size.height as usize,*/ state: v, previous_state: vec![false; (size.width * size.height) as usize]});

    commands.spawn_bundle(SpriteBundle {
        texture: image_handle,
        transform: Transform::from_scale((2.0, 2.0, 1.0).into()),
        ..default()
    });

}

struct GameOfLife {
    image: Handle<Image>, x: usize, /*y: usize,*/ state: Vec<bool>, previous_state: Vec<bool>,
}

fn update_image(mut images: ResMut<Assets<Image>>, mut texture: ResMut<GameOfLife>) {
    let data = &mut images.get_mut(&texture.image).unwrap().data;

    for i in 0..(data.len() / 4) {
        if texture.state[i] {
            data[i*4] = 255;
            data[i*4+1] = 255;
            data[i*4+2] = 255;
            data[i*4+3] = 255;
        } else {
            data[i*4] = 0;
            data[i*4+1] = 0;
            data[i*4+2] = 0;
            data[i*4+3] = 255;
        }
    }

    texture.previous_state = texture.state.clone();
}

fn game_of_life(mut texture: ResMut<GameOfLife>) {

    for i in 0..texture.state.len() {
        let mut count: u8 = 0;

        let is_y_under_safe = i >= texture.x;
        let is_y_over_safe = i < texture.state.len() - texture.x;
        let is_x_under_safe = i%texture.x != 0;
        let is_x_over_safe = (i+1)%texture.x != 0;

        if is_y_under_safe {
            if is_x_under_safe {
                if texture.previous_state[i-texture.x-1] {
                    count += 1;
                }
            }

            if texture.previous_state[(i-texture.x)] {
                count += 1;
            }

            if is_x_over_safe {
                if texture.previous_state[i-texture.x + 1] {
                    count += 1;
                }
            }
        }

        //checking at top
        if is_y_over_safe {
            if is_x_under_safe {
                if texture.previous_state[i + texture.x - 1] {
                    count += 1;
                }   
            }

            if texture.previous_state[i + texture.x] {
                count += 1;
            }

            if is_x_over_safe {
                if texture.previous_state[i + texture.x + 1] {
                    count += 1;
                }
            }
        }

        if is_x_under_safe {
            if texture.previous_state[i - 1] {
                count += 1;
            }
        } 

        if is_x_over_safe {
            if texture.previous_state[i + 1] {
                count += 1;
            }
        } 


        if texture.previous_state[i] {
            if count <= 1 || count >= 4 {
                texture.state[i] = false;
            }
        } else {
            if count == 3 {
                texture.state[i] = true;
            }
        }
    }
}
