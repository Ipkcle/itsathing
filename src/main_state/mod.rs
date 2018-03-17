use ggez::{Context, GameResult};
use ggez::graphics::{Point2, Vector2};
use ggez::graphics::Image;
use ggez::graphics;
use ggez::event::*;
use ggez::timer;
use assets::Assets;
use game_object;
use game_object::*;
use game_object::block::Block;
use game_object::mob::*;
use game_object::bullet::Bullet;

mod debug;
mod input;

use self::input::*;
use self::input::Axis;
pub struct MainState {
    screen_w: u32,
    screen_h: u32,
    input: Input,
    assets: Assets,
    player_mob: player::Player,
    //  player_gun:
    mobs: Vec<Dummy>,
    blocks: Vec<Block>,
    projectiles: Vec<Bullet>,
    camera: Vector2,
    debug_display: debug::DebugTable,
}

impl MainState {
    pub fn new(ctx: &mut Context, screen_w: u32, screen_h: u32) -> GameResult<MainState> {
        let mut state = MainState {
            screen_w,
            screen_h,
            input: Input::new(),
            assets: Assets::new(ctx),
            player_mob: player::Player::new(),
            mobs: Vec::new(),
            blocks: Vec::new(),
            projectiles: Vec::new(),
            camera: Vector2::new(0.0, 0.0),
            debug_display: debug::DebugTable::new(ctx, Point2::new(0.0, 0.0)),
        };
        state.blocks.push(Block::wallh(-170.0, 200.0));
        state.blocks.push(Block::wallh(-190.0, -200.0));
        state.blocks.push(Block::wallv(210.0, -200.0));
        state.blocks.push(Block::wallv(-190.0, -180.0));
        Ok(state)
    }

    fn handle_player_input(&mut self) {
        self.player_mob
            .set_movement(self.input.move_stack.get_direction_recent());
        self.player_mob
            .set_shoot_direction(self.input.shoot_stack.get_direction_recent());
        if self.input.shoot_stack.is_active() {
            match self.player_mob.shoot() {
                Some(projectile) => self.projectiles.push(projectile),
                None => (),
            }
        }
    }

    fn calculate_step(&mut self, dt: f32) {
        self.player_mob.step(dt);
        for object in &mut self.mobs {
            object.step(dt);
        }
        for object in &mut self.blocks {
            object.step(dt);
        }
        for object in &mut self.projectiles {
            object.step(dt);
        }
        //load player hp to debug display
        self.debug_display.load("hp".to_owned(), self.player_mob.get_health().to_string());
    }

    fn clear_objects(&mut self) {
        self.projectiles
            .retain(|ref projectile| !projectile.should_delete());
        self.mobs.retain(|ref mob| !mob.should_delete());
        if self.player_mob.should_delete() {
            self.player_mob = player::Player::new();
            self.mobs.drain(..);
            self.projectiles.drain(..);
        }
    }

    fn reset(&mut self) {
        self.player_mob = player::Player::new();
        self.mobs.drain(..);
        self.projectiles.drain(..);
        self.mobs.push(Dummy::new(Point2::new(100.0, 100.0)));
        self.mobs.push(Dummy::new(Point2::new(100.0, -100.0)));
        self.mobs.push(Dummy::new(Point2::new(-100.0, -100.0)));
        self.mobs.push(Dummy::new(Point2::new(150.0, 150.0)));
        self.mobs.push(Dummy::new(Point2::new(150.0, -150.0)));
        self.mobs.push(Dummy::new(Point2::new(-150.0, -150.0)));
    }

    fn calculate_ai(&mut self) {
        for object in &mut self.mobs {
            object.set_target(self.player_mob.get_position());
        }
    }

    fn calculate_physics(&mut self, dt: f32) {
        collision::vec_vec_physics(dt, &mut self.mobs, &mut self.blocks);
        collision::vec_physics(dt, &mut self.mobs);
        collision::object_vec_physics(dt, &mut self.player_mob, &mut self.blocks);
        collision::object_vec_physics(dt, &mut self.player_mob, &mut self.mobs);
    }

    fn calculate_collision_events(&mut self, dt: f32) {
        game_object::vec_vec_collision_events(dt, &mut self.mobs, &mut self.projectiles);
        game_object::object_vec_collision_events(dt, &mut self.player_mob, &mut self.projectiles);
        game_object::object_vec_collision_events(dt, &mut self.player_mob, &mut self.mobs);
    }

    fn world_to_screen_coords(&self, point: Point2) -> Point2 {
        let width = self.screen_w as f32;
        let height = self.screen_h as f32;
        let x = point.x + -1.0 * self.camera.x + width / 2.0;
        let y = height - (point.y + -1.0 * self.camera.y + height / 2.0);
        Point2::new(x, y)
    }

    fn _is_on_screen(&self, point: Point2) -> bool {
        (point.x >= 0.0 && point.x <= self.screen_w as f32 && point.y > 0.0
            && point.y < self.screen_h as f32)
    }

    fn update_camera(&mut self) {
        let (x, y) = (
            self.player_mob.get_position().x,
            self.player_mob.get_position().y,
        );
        self.camera.x = x;
        self.camera.y = y;
    }

    fn draw_object<T: Renderable>(&self, ctx: &mut Context, object: &T) -> GameResult<()> {
        //Find the pixel position on screen of the object.
        let pos = self.world_to_screen_coords(object.get_position());

        //      //If the object is not on screen, do nothing.
        //      if !self.is_on_screen(pos) {
        //          return Ok(());
        //      }

        //Draw the drawable component if the object has one, else return an error.
        let d = object.get_drawable_asset();

        //get the mesh from assets
        let drawable = self.assets.get_drawable(d);

        //Set the drawing parameters.
        let drawparams = graphics::DrawParam {
            dest: pos,
            color: object.get_color(),
            ..Default::default()
        };

        //Actually draw to the context.
        graphics::draw_ex(ctx, drawable, drawparams)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            self.handle_player_input();
            self.calculate_step(seconds);
            self.calculate_collision_events(seconds);
            self.calculate_physics(seconds);
            self.calculate_ai();
            self.clear_objects();
            self.update_camera();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //clear the contex
        graphics::clear(ctx);

        //load the fps to the debug table
        let fps = timer::get_fps(ctx) as u16;
        self.debug_display.load("fps".to_owned(), fps.to_string());

        //draw the debug table
        self.debug_display.render(ctx);
        //draw the player
        if let Err(error) = self.draw_object(ctx, &self.player_mob) {
            return Err(error);
        }
        //draw objects with renderable component
        for object in &self.mobs {
            if let Err(error) = self.draw_object(ctx, object) {
                return Err(error);
            }
        }

        for object in &self.blocks {
            if let Err(error) = self.draw_object(ctx, object) {
                return Err(error);
            }
        }

        for object in &self.projectiles {
            if let Err(error) = self.draw_object(ctx, object) {
                return Err(error);
            }
        }

        //show context on screen
        graphics::present(ctx);

        //yeild cpu when not active
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::R => {
                self.reset();
            }
            Keycode::W => {
                self.input
                    .move_stack
                    .activate_direction(DirectionInputScalar::Positive, Axis::Y);
            }
            Keycode::S => {
                self.input
                    .move_stack
                    .activate_direction(DirectionInputScalar::Negative, Axis::Y);
            }
            Keycode::A => {
                self.input
                    .move_stack
                    .activate_direction(DirectionInputScalar::Negative, Axis::X);
            }
            Keycode::D => {
                self.input
                    .move_stack
                    .activate_direction(DirectionInputScalar::Positive, Axis::X);
            }
            Keycode::Up => {
                self.input
                    .shoot_stack
                    .activate_direction(DirectionInputScalar::Positive, Axis::Y);
            }
            Keycode::Down => {
                self.input
                    .shoot_stack
                    .activate_direction(DirectionInputScalar::Negative, Axis::Y);
            }
            Keycode::Left => {
                self.input
                    .shoot_stack
                    .activate_direction(DirectionInputScalar::Negative, Axis::X);
            }
            Keycode::Right => {
                self.input
                    .shoot_stack
                    .activate_direction(DirectionInputScalar::Positive, Axis::X);
            }
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::W => {
                self.input
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Positive, Axis::Y);
            }
            Keycode::S => {
                self.input
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Negative, Axis::Y);
            }
            Keycode::A => {
                self.input
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Negative, Axis::X);
            }
            Keycode::D => {
                self.input
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Positive, Axis::X);
            }
            Keycode::Up => {
                self.input
                    .shoot_stack
                    .deactivate_direction(DirectionInputScalar::Positive, Axis::Y);
            }
            Keycode::Down => {
                self.input
                    .shoot_stack
                    .deactivate_direction(DirectionInputScalar::Negative, Axis::Y);
            }
            Keycode::Left => {
                self.input
                    .shoot_stack
                    .deactivate_direction(DirectionInputScalar::Negative, Axis::X);
            }
            Keycode::Right => {
                self.input
                    .shoot_stack
                    .deactivate_direction(DirectionInputScalar::Positive, Axis::X);
            }
            _ => (), // Do nothing
        }
    }
}
