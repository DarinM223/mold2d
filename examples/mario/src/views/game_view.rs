use crate::actions::{Actor, ActorAction, ActorMessage, ActorType};
use crate::actions::{actor_from_token, handle_collision, handle_message, resolve_collision};
use crate::views::background_view::BackgroundView;
use mold2d::font;
use mold2d::level;
use mold2d::{ActorManager, Context, Quadtree, Sprite, View, ViewAction, Viewport};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::error::Error;

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
        let level_result =
            level::load_level(path, actor_from_token, &mut context.canvas, &context.window);
        let (actors, viewport) = level_result.unwrap();

        if context.score.score("GAME_SCORE").is_none() {
            context.score.add_score("GAME_SCORE");
        }

        GameView {
            actors,
            viewport,
            level_path: path.to_owned(),
            cached_score: None,
            cached_font_sprite: None,
        }
    }
}

impl View for GameView {
    #[inline]
    fn render(&mut self, context: &mut Context, elapsed: f64) -> Result<(), Box<dyn Error>> {
        // start off with a black screen
        context.canvas.set_draw_color(Color::RGB(135, 206, 250));
        context.canvas.clear();

        // render contained actors
        for actor in self.actors.values_mut() {
            if self.viewport.rect_in_viewport(&actor.data().rect) {
                actor.render(context, &mut self.viewport, elapsed)?;
            }
        }

        // render score
        if let Some(score) = context.score.score("GAME_SCORE") {
            let score_text = format!("Score: {}", score);
            let mut had_cached_score = false;

            if let Some(ref prev_score) = self.cached_score {
                if *prev_score == score_text {
                    if let Some(ref font_sprite) = self.cached_font_sprite {
                        font::render_text(&mut context.canvas, font_sprite, (100, 100))?;
                    }
                    had_cached_score = true;
                }
            }

            if !had_cached_score {
                let font_sprite = font::text_sprite(
                    &context.canvas,
                    &score_text[..],
                    "assets/belligerent.ttf",
                    32,
                    Color::RGB(0, 255, 0),
                )
                .unwrap();
                font::render_text(&mut context.canvas, &font_sprite, (100, 100))?;
                self.cached_score = Some(score_text);
                self.cached_font_sprite = Some(font_sprite);
            }
        }

        Ok(())
    }

    #[inline]
    fn update(&mut self, context: &mut Context, elapsed: f64) -> Option<ViewAction> {
        if context.events.event_called("QUIT") || context.events.event_called("ESC") {
            return Some(ViewAction::Quit);
        }

        if context.events.event_called_once("ENTER") {
            return Some(ViewAction::ChangeView(Box::new(BackgroundView)));
        }

        let window_rect = Rect::new(0, 0, context.window.width, context.window.height);
        let viewport_clone = self.viewport.clone();
        let mut quadtree = Quadtree::new(window_rect, &viewport_clone);
        let mut keys = Vec::with_capacity(self.actors.len());

        for (key, actor) in &mut self.actors.iter_mut() {
            let data = actor.data();

            if self.viewport.rect_in_viewport(&data.rect) {
                keys.push(key);
                quadtree.insert(data);
            }
        }

        for key in keys {
            let mut collisions = [None; 6];
            let mut collision_idx = 0;
            if let Some(actor) = self.actors.get_mut(key) {
                let data = actor.data();

                // update the actor
                let pos_change = actor.update(context, elapsed);
                actor.handle_message(&ActorMessage::ActorAction {
                    send_id: data.index,
                    recv_id: data.index,
                    action: ActorAction::ChangePosition(pos_change),
                });

                if data.collision_filter != 0 && data.actor_type != ActorType::Block {
                    // only check collisions for nearby actors
                    let nearby_actors = quadtree
                        .retrieve(&data.rect)
                        .into_iter()
                        .cloned()
                        .collect::<Vec<_>>();
                    for other in nearby_actors {
                        if let Some(direction) = actor.collides_with(&other) {
                            resolve_collision(actor, &other, direction);
                            collisions[collision_idx] = Some((data, other, direction));
                            collision_idx += 1;
                        }
                    }
                }

                if data.actor_type == ActorType::Player {
                    self.viewport.set_position((data.rect.x(), data.rect.y()));
                }
            }

            for collision in collisions.iter() {
                if let Some((actor, other, direction)) = *collision {
                    handle_collision(
                        &actor,
                        &other,
                        direction,
                        &handle_message,
                        &mut self.actors,
                        &mut self.viewport,
                        context,
                    );
                } else {
                    break;
                }
            }
        }

        None
    }
}
