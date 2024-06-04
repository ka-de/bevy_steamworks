use bevy::prelude::*;
use bevy_steamworks::*;

fn steam_system(steam_client: Res<Client>) {
    for friend in steam_client.friends().get_friends(FriendFlags::IMMEDIATE) {
        println!("Friend: {:?} - {}({:?})", friend.id(), friend.name(), friend.state());
    }
}

fn main() {
    let steamworks_plugin = match SteamworksPlugin::init_app(981370) {
        Ok(plugin) => plugin,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    App::new()
        .add_plugins(steamworks_plugin)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, steam_system)
        .run();
}
