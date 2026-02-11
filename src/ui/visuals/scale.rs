use dioxus::prelude::*;
use crate::logic::*;

#[component]
pub fn Scale(weighing: Weighing, outcome: Outcome, fake_coin: usize, fake_weight: Weight, visible: bool) -> Element {
    let target_angle = match outcome {
        Outcome::Balanced => 0.0,
        Outcome::LeftHeavy => -20.0,
        Outcome::RightHeavy => 20.0,
    };

    let mut current_angle = use_signal(|| 0.0);

    use_effect(use_reactive((&outcome, &visible), move |(_outcome, visible)| {
        if visible {
            spawn(async move {
                current_angle.set(0.0);
                gloo_timers::future::TimeoutFuture::new(100).await;
                current_angle.set(target_angle);
            });
        } else {
            current_angle.set(0.0);
        }
    }));

    let angle = current_angle();

    rsx! {
        svg { width: "300", height: "220", view_box: "0 0 300 220", class: "bg-[#1f1f1f] rounded-xl border border-neutral-700 shadow-lg",
            // Base
            line { x1: "150", y1: "200", x2: "150", y2: "50", stroke: "#4b5563", stroke_width: "4" }
            line { x1: "100", y1: "200", x2: "200", y2: "200", stroke: "#4b5563", stroke_width: "4" }
            
            // Beam Group
            g { 
                style: "transition: transform 1.5s cubic-bezier(0.34, 1.56, 0.64, 1); transform-origin: 150px 50px; transform: rotate({angle}deg)",
                
                // Beam
                line { x1: "50", y1: "50", x2: "250", y2: "50", stroke: "#9ca3af", stroke_width: "6", stroke_linecap: "round" }
                circle { cx: "150", cy: "50", r: "6", fill: "#d1d5db", stroke: "#4b5563", stroke_width: "2" }

                // Left Pan
                Pan { 
                    x: 50.0, 
                    angle: -angle, 
                    coins: weighing.left, 
                    fake_coin: fake_coin, 
                    fake_weight: fake_weight 
                }

                // Right Pan
                Pan { 
                    x: 250.0, 
                    angle: -angle, 
                    coins: weighing.right, 
                    fake_coin: fake_coin, 
                    fake_weight: fake_weight 
                }
            }
        }
    }
}

#[component]
fn Pan(x: f32, angle: f32, coins: Vec<usize>, fake_coin: usize, fake_weight: Weight) -> Element {
    // Pan geometry logic (shared)
    let pan_y = 140.0;
    
    rsx! {
        g {
            style: "transition: transform 1.5s cubic-bezier(0.34, 1.56, 0.64, 1); transform-origin: {x}px 50px; transform: rotate({angle}deg)",
            
            // Hook
            circle { cx: "{x}", cy: "50", r: "3", fill: "none", stroke: "#9ca3af", stroke_width: "2" }

            // Suspension
            line { x1: "{x}", y1: "53", x2: "{x}", y2: "110", stroke: "#6b7280", stroke_width: "1" }
            line { x1: "{x}", y1: "110", x2: "{x - 30.0}", y2: "{pan_y}", stroke: "#6b7280", stroke_width: "1" }
            line { x1: "{x}", y1: "110", x2: "{x + 30.0}", y2: "{pan_y}", stroke: "#6b7280", stroke_width: "1" }
            
            // Pan Dish
            path { 
                d: "M {x - 30.0} {pan_y} Q {x} {pan_y + 40.0} {x + 30.0} {pan_y} Z", 
                fill: "#374151", stroke: "#6b7280", stroke_width: "2" 
            }
            
            // Coins
            g { transform: "translate({x}, {pan_y})",
                for (i, &c) in coins.iter().enumerate() {
                    Coin { i, c, is_fake: c == fake_coin, fake_weight }
                }
            }
        }
    }
}

#[component]
fn Coin(i: usize, c: usize, is_fake: bool, fake_weight: Weight) -> Element {
    let color = if is_fake {
        match fake_weight {
            Weight::Heavy => "#ef4444", 
            Weight::Light => "#3b82f6", 
            _ => "#fbbf24", 
        }
    } else {
        "#fbbf24"
    };
    
    let col = i % 3;
    let row = i / 3;
    let cx = (col as f32 - 1.0) * 16.0; 
    let cy = -10.0 - (row as f32 * 16.0); 

    rsx! {
        g { key: "{c}",
            circle { cx: "{cx}", cy: "{cy}", r: "7", fill: "{color}", stroke: "#b45309", stroke_width: "1" }
            text { x: "{cx}", y: "{cy + 2.5}", text_anchor: "middle", font_size: "8", fill: "#451a03", font_weight: "bold", "{c}" }
        }
    }
}
