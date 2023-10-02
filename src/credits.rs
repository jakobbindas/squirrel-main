use bevy::prelude::*;

pub(crate) struct CreditPlugin;

// timer for credits
#[derive(Resource)]
struct CreditTimer(Timer);

// resource storing information about current state of credits
#[derive(Resource)]
struct CreditInfo {
    curr_index: usize,
    length: usize,
}

// credit components, has an id to ensure they are shown in the correct order
#[derive(Component)]
struct CreditImage {
    id: usize,
}

impl Plugin for CreditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, next_slide);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // vec of paths to each credit image
    let slide_paths: Vec<&str> = vec![
        "credits/credit_maddy.png",
        "credits/credit_kyra.png",
        "credits/credit_jakob.png",
        "credits/credit_Miles.png",
        "credits/credit_imran.png",
        "credits/credit_sam.png",
        "credits/credit_justin.png",
    ];

    // spawn credit images, only the first one is made visible
    for (i, slide) in slide_paths.iter().enumerate() {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load(*slide),
                visibility: if i == 0 {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                ..default()
            })
            .insert(CreditImage { id: i });
    }

    commands.insert_resource(CreditTimer(Timer::from_seconds(5., TimerMode::Repeating)));
    commands.insert_resource(CreditInfo {
        curr_index: 0,
        length: slide_paths.len(),
    });
}

fn next_slide(
    time: Res<Time>,
    mut timer: ResMut<CreditTimer>,
    mut credit_info: ResMut<CreditInfo>,
    mut slides: Query<(&mut Visibility, &CreditImage)>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        // go to next slide, wrapping if we reach the end
        credit_info.curr_index += 1;
        credit_info.curr_index %= credit_info.length;

        // make correct slide visible
        for (mut visibility, credit) in slides.iter_mut() {
            if credit.id == credit_info.curr_index {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
