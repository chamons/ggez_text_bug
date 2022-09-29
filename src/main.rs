use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::*;
use ggez::*;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}

struct MyGame {}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {}
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let mut text = Text::new("Hello Centered World");
        text.set_bounds(Vec2::new(400.0, 400.0))
            .set_layout(TextLayout {
                h_align: TextAlign::Middle,
                v_align: TextAlign::Begin,
            });
        canvas.draw(&text, Vec2::new(20.0, 20.0));

        let square_size = Rect::new(0.0, 0.0, 400.0, 400.0);
        let red_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            square_size,
            Color::new(0.8, 0.1, 0.1, 0.5),
        )
        .unwrap();
        canvas.draw(&red_square, Vec2::new(20.0, 20.0));

        canvas.finish(ctx)
    }
}
