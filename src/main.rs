use ggez::event::{self, EventHandler};
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
        graphics::clear(ctx, Color::BLACK);

        let mut text = Text::new("Hello Centered World");
        let point = mint::Point2 { x: 400.0, y: 400.0 };
        text.set_bounds(point, Align::Center);
        graphics::draw(
            ctx,
            &text,
            DrawParam::new().dest(mint::Point2 { x: 20.0, y: 20.0 }),
        )?;

        let square_size = Rect::new(0.0, 0.0, 400.0, 400.0);
        let red_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            square_size,
            Color::new(0.8, 0.1, 0.1, 0.5),
        )
        .unwrap();
        graphics::draw(
            ctx,
            &red_square,
            DrawParam::new().dest(mint::Point2 { x: 20.0, y: 20.0 }),
        )?;
        graphics::present(ctx)?;

        Ok(())
    }
}
