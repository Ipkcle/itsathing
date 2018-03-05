use ggez::{Context, GameResult};
use ggez::graphics::{Point2, Vector2};
use ggez::graphics;
use ggez::event::*;
use ggez::timer;
use assets::Assets;
use game_object;
use game_object::*;
use game_object::block::Block;
use game_object::mob::Mob;
use game_object::bullet::Bullet;

enum Action {
    None,
    Item,
}

struct Input {
    action: Action,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    move_stack: Vec<Vector2>,
    shoot_direction: Vector2,
    x_axis: f32,
    y_axis: f32,
    shooting: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            action: Action::None,
            up: false,
            down: false,
            left: false,
            right: false,
            move_stack: Vec::new(),
            shoot_direction: Vector2::new(0.0, 0.0),
            x_axis: 0.0,
            y_axis: 0.0,
            shooting: false,
        }
    }
}

pub struct MainState {
    screen_w: u32,
    screen_h: u32,
    input: Input,
    assets: Assets,
    player_mob: player::Player,
    //  player_gun:
    mobs: Vec<Mob>,
    blocks: Vec<Block>,
    projectiles: Vec<Bullet>,
    camera: Vector2,
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
        };
        state.mobs.push(Mob::dummy(Point2::new(100.0, 100.0)));
        state.mobs.push(Mob::dummy(Point2::new(100.0, -100.0)));
        state.mobs.push(Mob::dummy(Point2::new(-100.0, -100.0)));
        state.mobs.push(Mob::dummy(Point2::new(150.0, 150.0)));
        state.mobs.push(Mob::dummy(Point2::new(150.0, -150.0)));
        state.mobs.push(Mob::dummy(Point2::new(-150.0, -150.0)));
        state.blocks.push(Block::wallh(-190.0, 200.0));
        state.blocks.push(Block::wallh(-190.0, -200.0));
        state.blocks.push(Block::wallv(200.0, -200.0));
        state.blocks.push(Block::wallv(-200.0, -200.0));
        Ok(state)
    }

    fn handle_player_input(&mut self) {
        match (self.input.up, self.input.down) {
            (true, false) => {
                self.input.y_axis = 1.0;
            }
            (false, true) => {
                self.input.y_axis = -1.0;
            }
            _ => {
                self.input.y_axis = 0.0;
            }
        }
        match (self.input.left, self.input.right) {
            (true, false) => {
                self.input.x_axis = -1.0;
            }
            (false, true) => {
                self.input.x_axis = 1.0;
            }
            _ => {
                self.input.x_axis = 0.0;
            }
        }
        self.player_mob
            .set_movement(Vector2::new(self.input.x_axis, self.input.y_axis));
        self.player_mob
            .set_shoot_direction(self.input.shoot_direction);
        if self.input.shooting {
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

    fn is_on_screen(&self, point: Point2) -> bool {
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
            color: Some(object.get_color()),
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

        //awful fps display
        /*
        let fps = &timer::get_fps(ctx).to_string();
        graphics::Text::new(ctx, fps, &graphics::Font::default_font().unwrap())
            .unwrap()
            .draw_ex(
                ctx,
                graphics::DrawParam {
                    dest: Point2::new(0.0, 0.0),
                    ..Default::default()
                },
            )
            .unwrap();
            */

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
            Keycode::W => {
                self.input.up = true;
            }
            Keycode::S => {
                self.input.down = true;
            }
            Keycode::A => {
                self.input.left = true;
            }
            Keycode::D => {
                self.input.right = true;
            }
            Keycode::Up => {
                self.input.shoot_direction = Vector2::new(0.0, 1.0);
                self.input.shooting = true;
            }
            Keycode::Down => {
                self.input.shoot_direction = Vector2::new(0.0, -1.0);
                self.input.shooting = true;
            }
            Keycode::Left => {
                self.input.shoot_direction = Vector2::new(-1.0, 0.0);
                self.input.shooting = true;
            }
            Keycode::Right => {
                self.input.shoot_direction = Vector2::new(1.0, 0.0);
                self.input.shooting = true;
            }
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::W => {
                self.input.up = false;
            }
            Keycode::S => {
                self.input.down = false;
            }
            Keycode::A => {
                self.input.left = false;
            }
            Keycode::D => {
                self.input.right = false;
            }
            Keycode::Up => {
                self.input.shooting = false;
            }
            Keycode::Down => {
                self.input.shooting = false;
            }
            Keycode::Left => {
                self.input.shooting = false;
            }
            Keycode::Right => {
                self.input.shooting = false;
            }
            _ => (), // Do nothing
        }
    }
}
