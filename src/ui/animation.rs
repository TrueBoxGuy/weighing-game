use dioxus::prelude::*;
use crate::logic::*;

pub fn use_animation_controller(
    target_path: Memo<Vec<(Weighing, Outcome)>>,
    mut display_path: Signal<Vec<(Weighing, Outcome)>>,
    mut visible_steps: Signal<usize>
) {
    let mut generation = use_signal(|| 0usize);

    use_effect(move || {
        let target = target_path();
        let current_gen = *generation.peek() + 1;
        generation.set(current_gen);

        spawn(async move {
            let start_vis = visible_steps();
            if start_vis > 0 {
                for i in (0..start_vis).rev() {
                    if generation() != current_gen { return; }
                    visible_steps.set(i);
                    gloo_timers::future::TimeoutFuture::new(300).await;
                }
            }
            
            if generation() != current_gen { return; }
            visible_steps.set(0); 
            
            display_path.set(target.clone());
            
            if !target.is_empty() {
                gloo_timers::future::TimeoutFuture::new(200).await;
                for i in 1..=target.len() { 
                    if generation() != current_gen { return; }
                    visible_steps.set(i);
                    gloo_timers::future::TimeoutFuture::new(800).await;
                }
            }
        });
    });
}
