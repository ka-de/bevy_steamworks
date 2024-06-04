use bevy::prelude::*;
use bevy_steamworks::*;

fn steam_system(steam_client: Res<Client>) {
    for friend in steam_client.friends().get_friends(FriendFlags::IMMEDIATE) {
        println!(
            "Friend: {:?} - {}({:?})",
            friend.id(),
            friend.name(),
            friend.state()
        );
    }
}

fn main() {
   let steamworks_plugin = match SteamworksPlugin::init_app(480) {
        Ok(plugin) => plugin,
        Err(err) => {
            eprintln!("Failed to initialize Steam: {}", err);
            return;
        }
    };

    App::new()
        .add_plugins(steamworks_plugin)
        .add_plugins(DefaultPlugins)
        .run();
}