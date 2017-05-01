use actions::{Actor, ActorAction, ActorMessage, ActorType};
use actions::{actor_from_token, handle_collision, handle_message};
use mold2d::font;
use mold2d::level;
use mold2d::{ActorManager, Context, Quadtree, Sprite, View, ViewAction, Viewport};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use views::background_view::BackgroundView;

/// The main game view used for
/// the actual gameplay
pub struct GameView {
    actors: ActorManager<Actor>,
    viewport: Viewport,
    level_path: String,
    cached_score: Option<String>,
    cached_font_sprite: Option<Sprite>,
}

impl GameView {
    pub fn new(path: &str, context: &mut Context) -> GameView {
        let level_result = level::load_level(path,
                                             Box::new(actor_from_token),
                                             &mut context.renderer,
                                             &context.window);
        let (actors, viewport) = level_result.unwrap();

        if context.score.score("GAME_SCORE") == None {
            context.score.add_score("GAME_SCORE");
        }

        GameView {
            actors: actors,
            viewport: viewport,
            level_path: path.to_owned(),
            cached_score: None,
            cached_font_sprite: None,
        }
    }
}

impl View for GameView {
    #[inline]
    fn render(&mut self, context: &mut Context, elapsed: f64) {
        // start off with a black screen
        context.renderer.set_draw_color(Color::RGB(135, 206, 250));
        context.renderer.clear();

        // render contained actors
        for (_, actor) in &mut self.actors.actors {
            if let Some(_) = self.viewport.constrain_to_viewport(&actor.data().rect) {
                actor.render(context, &mut self.viewport, elapsed);
            }
        }

        // render score
        if let Some(score) = context.score.score("GAME_SCORE") {
            let score_text = format!("Score: {}", score);
            let mut had_cached_score = false;

            if let Some(ref prev_score) = self.cached_score {
                if *prev_score == score_text {
                    if let Some(ref font_sprite) = self.cached_font_sprite {
                        font::render_text(&mut context.renderer, font_sprite, (100, 100));
                    }
                    had_cached_score = true;
                }
            }

            if !had_cached_score {
                let font_sprite = font::text_sprite(&context.renderer,
                                                    &score_text[..],
                                                    "assets/belligerent.ttf",
                                                    32,
                                                    Color::RGB(0, 255, 0))
                    .unwrap();
                font::render_text(&mut context.renderer, &font_sprite, (100, 100));
                self.cached_score = Some(score_text);
                self.cached_font_sprite = Some(font_sprite);
            }
        }
    }

    #[inline]
    fn update(&mut self, context: &mut Context, elapsed: f64) -> Option<ViewAction> {
        if context.events.event_called("QUIT") || context.events.event_called("ESC") {
            return Some(ViewAction::Quit);
        }

        if context.events.event_called_once("ENTER") {
            return Some(ViewAction::ChangeView(Box::new(BackgroundView)));
        }

        {
            let window_rect = Rect::new(0, 0, context.window.width, context.window.height);
            let viewport_clone = self.viewport.clone();
            let mut quadtree = Quadtree::new(window_rect, &viewport_clone);
            let mut keys = Vec::with_capacity(self.actors.actors.len());

            for (key, actor) in &mut self.actors.actors {
                let data = actor.data().clone();

                if let Some(_) = self.viewport.constrain_to_viewport(&data.rect) {
                    keys.push(key.clone());
                    quadtree.insert(data);
                }
            }

            for key in keys {
                let actor = self.actors.temp_remove(key);
                if let Some(mut actor) = actor {
                    let data = actor.data();

                    // update the actor
                    let pos_change = actor.update(context, elapsed);
                    actor.handle_message(&ActorMessage::ActorAction {
                        send_id: data.id,
                        recv_id: data.id,
                        action: ActorAction::ChangePosition(pos_change),
                    });

                    if data.collision_filter != 0 && data.actor_type != ActorType::Block {
                        // only check collisions for nearby actors
                        let nearby_actors = quadtree.retrieve(&data.rect)
                            .into_iter()
                            .map(|act| act.clone())
                            .collect::<Vec<_>>();
                        for other in nearby_actors {
                            if let Some(direction) = actor.collides_with(&other) {
                                handle_collision(&mut actor,
                                                 &other,
                                                 direction,
                                                 Box::new(handle_message),
                                                 &mut self.actors,
                                                 &mut self.viewport,
                                                 context);
                            }
                        }
                    }

                    self.actors.temp_reinsert(actor.data().id, actor);

                    if data.actor_type == ActorType::Player {
                        self.viewport.set_position((data.rect.x(), data.rect.y()));
                    }
                }
            }
        }

        None
    }
}
