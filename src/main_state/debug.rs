use std::collections::HashMap;
use ggez::graphics::Point2;
use ggez::Context;
use ggez::graphics;
use ggez::graphics::Image;

const SEPERATOR: &str = ":";

pub struct DebugTable {
    position: Point2,
    data: HashMap<String, String>,
    images: HashMap<String, Image>,
}

impl DebugTable {
    pub fn new(ctx: &mut Context, position: Point2) -> DebugTable {
        let mut t = DebugTable { 
            position,
            data: HashMap::new(),
            images: HashMap::new(),
        };
        let colon_image = DebugTable::make_image_from(ctx, SEPERATOR);
        t.images.insert(SEPERATOR.to_owned(), colon_image);
        t
    }
    pub fn load(&mut self, label: String, data: String) {
        self.data.insert(label, data); 
    }

    pub fn render(&mut self, ctx: &mut Context) {
        let mut cursor = self.position.clone();
        let mut lables = Vec::new();
        for label in self.data.keys() {
            lables.push(label.clone());
        }
        for label in lables {
            self.draw_text(ctx, &label[..], &mut cursor, false);
            self.draw_text(ctx, SEPERATOR, &mut cursor, false);
            let value = &self.data.get(&label).unwrap().clone()[..];
            self.draw_text(ctx, value, &mut cursor, true);
            cursor.x = self.position.x;
        }
    }

    fn draw_text(&mut self, ctx: &mut Context, string: &str, cursor: &mut Point2, new_line: bool) {
        if let Some(image) = self.images.get(string) {
            DebugTable::draw(ctx, image, cursor, new_line);
            return
        }
        self.images.insert(String::from(string), DebugTable::make_image_from(ctx, string));
        if let Some(image) = self.images.get(string) {
            DebugTable::draw(ctx, image, cursor, new_line);
        }
    }

    fn draw(ctx: &mut Context, image: &Image, cursor: &mut Point2, new_line: bool) {
        graphics::draw_ex(
            ctx,
            image,
            graphics::DrawParam {
                dest: *cursor,
                ..Default::default()
            },
        ).unwrap();
        cursor.x += image.width() as f32;
        if new_line {
            cursor.y += image.height() as f32;
        }
    }

    fn make_image_from(ctx: &mut Context, string: &str) -> Image {
        graphics::Text::new(ctx, string, &graphics::Font::default_font().unwrap())
            .unwrap()
            .into_inner()
    }
}
