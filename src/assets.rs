use ggez::graphics::{DrawMode, Drawable, Mesh, MeshBuilder};
use ggez::graphics::Point2;
use ggez::Context;

#[derive(Clone, Copy)]
pub enum DrawableAsset {
    Player,
    Block,
    Bullet,
}

pub struct Assets {
    drawable: DrawableAssets,
}

impl Assets {
    pub fn get_drawable(&self, drawable_asset: DrawableAsset) -> &Drawable {
        self.drawable.get_drawable(drawable_asset)
    }

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            drawable: DrawableAssets::new(ctx),
        }
    }
}

struct DrawableAssets {
    player: Mesh,
    block: Mesh,
    bullet: Mesh,
}

impl DrawableAssets {
    pub fn new(ctx: &mut Context) -> DrawableAssets {
        DrawableAssets {
            player: Self::rect(ctx, 10.0, 10.0),
            block: Self::rect(ctx, 20.0, 30.0),
            bullet: Self::rect(ctx, 2.0, 2.0),
        }
    }

    fn circle(ctx: &mut Context, radius: f32) -> Mesh {
        let mut builder = MeshBuilder::new();
        builder.circle(
            DrawMode::Fill,
            Point2::new(-1.0 * radius, -1.0 * radius),
            radius,
            radius / 200.0,
        );
        builder.build(ctx).unwrap()
    }

    fn rect(ctx: &mut Context, x: f32, y: f32) -> Mesh {
        let mut builder = MeshBuilder::new();
        builder.polygon(
            DrawMode::Fill,
            &[
                Point2::new(0.0, 0.0),
                Point2::new(x, 0.0),
                Point2::new(x, -1.0*y),
                Point2::new(0.0, -1.0*y),
            ],
        );
        builder.build(ctx).unwrap()
    }

    pub fn get_drawable(&self, drawable_asset: DrawableAsset) -> &Drawable {
        match drawable_asset {
            DrawableAsset::Player => &self.player,
            DrawableAsset::Block => &self.block,
            DrawableAsset::Bullet => &self.bullet,
        }
    }
}
