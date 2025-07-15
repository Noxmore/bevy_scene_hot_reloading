# Bevy Scene Hot-reloading
Tiny plugin that respawns `Scene`s that have `AssetEvent::Modified` sent out, allowing scene hot-reloading.
This does not apply for `DynamicScene`s, which already have native hot-reloading support.

I will keep this up to date with the latest Bevy version until it is no longer needed.

Current Supported Bevy Version: `0.16`

## Usage
- Add this crate to your project: `cargo add bevy_scene_hot_reloading`
- Add `SceneHotReloadingPlugin` to your app, and you are good to go!
- You can also turn off the `log` feature with `default-features = false` to not log when a scene has been respawned.
