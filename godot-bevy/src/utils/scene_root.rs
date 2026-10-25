//! The scene tree's root window on every Godot API level.

use godot::classes::{SceneTree, Window};
use godot::obj::Gd;

/// The scene tree's root window.
///
/// Godot-Rust returns `SceneTree::get_root` as `Option<Gd<Window>>` up to API 4.6 and as
/// `Gd<Window>` from 4.7 (the engine marks it never null). This gives the `Option` on
/// every level, so call sites don't depend on the `api-4-*` feature.
pub fn scene_root(tree: &Gd<SceneTree>) -> Option<Gd<Window>> {
    tree.get_root().into_root()
}

/// Either form `SceneTree::get_root` returns, as an `Option`.
trait IntoRoot {
    fn into_root(self) -> Option<Gd<Window>>;
}

impl IntoRoot for Gd<Window> {
    fn into_root(self) -> Option<Gd<Window>> {
        Some(self)
    }
}

impl IntoRoot for Option<Gd<Window>> {
    fn into_root(self) -> Option<Gd<Window>> {
        self
    }
}
