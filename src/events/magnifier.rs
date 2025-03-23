// run  := cargo run
// dir  := .
// kid  :=

use std::error::Error;
use std::sync::Arc;

use crate::global_state::GlobalState;
use crate::hyprsocket::Hyprsocket;

pub struct Magnifier;

impl Magnifier {
    pub async fn handle(
        args: &[String], state: Arc<GlobalState>, client: Arc<Hyprsocket>
    ) -> Result<String, Box<dyn Error>> {
        if args.len() != 1 {
            return Err("magnifier requires exactly 1 argument".into());
        }

        let scale: f64 = args[0].parse()?;

        {
            let mut current_scale = state.current_magnifier_scale.lock().await;

            if (*current_scale - 1.0).abs() < f64::EPSILON {
                client.sends_silent(&[&format!("keyword cursor:zoom_factor {}", scale)]).await?;

                *current_scale = scale;
            }
            else {
                client.sends_silent(&["keyword cursor:zoom_factor 1"]).await?;
                *current_scale = 1.0;
            }
        }

        Ok("sublime text not found, start sublime text".to_string())
    }
}
