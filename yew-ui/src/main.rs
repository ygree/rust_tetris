use gloo::timers::callback::Interval;
use yew::{classes, html, Component, Context, Html, KeyboardEvent};
use core::glass::{Glass, MoveDirection};

enum Msg {
    Start,
    Left,
    Rotate,
    Right,
    Drop,
    Tick,
}

struct App {
    glass: Glass,
    _interval: Interval,
}

impl App {
    fn render_rows(&self) -> Vec<Html> {
        let figure_coordinates = self.glass.figure_coordinates();

        //TODO avoid heap allocation
        let mut rows = Vec::with_capacity(self.glass.height);
        for y in 0..self.glass.height {
            rows.push(self.render_row(y, figure_coordinates));
        }
        rows
    }

    fn render_row(&self, y: usize, figure_coordinates: Option<[(i32, i32); 4]>) -> Html {
        let row = &self.glass[y];

        let cells: Vec<_> = row.iter().enumerate().map(|(x, v)| {
            let mut color = *v;
            if !color {
                //TODO make this check part of the Glass interface
                if let Some(points) = figure_coordinates {
                    color = points.iter().find(|(px, py)| x == *px as usize && y == *py as usize).is_some();
                }
            }

            let cellule_status = {
                if color {
                    "cellule-live"
                } else {
                    "cellule-dead"
                }
            };
            let idx = y * self.glass.width + x;
            html! {
                <div key={idx} class={classes!("game-cellule", cellule_status)}>
                </div>
            }
        }).collect();

        html! {
            <div key={y} class="game-row">
                { for cells }
            </div>
        }
    }

    fn new_glass() -> Glass {
        Glass::new(12, 26)
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut glass = Self::new_glass();

        glass[25][0] = true;
        let callback = ctx.link().callback(|_| Msg::Tick);
        let _interval = Interval::new(670, move || callback.emit(()));

        Self {
            glass,
            _interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                if !self.glass.relocate_figure(MoveDirection::Down) {
                    self.glass.freeze_figure();
                    if self.glass.next_figure() {
                        //TODO game over?
                    }
                }
                self.glass.clean_filled_rows();
            },
            Msg::Start => {
                self.glass = Self::new_glass();
                self.glass.next_figure();
            },
            Msg::Left => {
                self.glass.relocate_figure(MoveDirection::Left);
            },
            Msg::Rotate => {
                self.glass.rotate_figure();
            },
            Msg::Right => {
                self.glass.relocate_figure(MoveDirection::Right);
            },
            Msg::Drop => {
                if !self.glass.relocate_figure(MoveDirection::Down) {
                    self.glass.freeze_figure();
                    if self.glass.next_figure() {
                        //TODO game over?
                    }
                }
                self.glass.clean_filled_rows();
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_rows = self.render_rows();

        let onkeydown = ctx.link().batch_callback(|event: KeyboardEvent| {
            match event.key_code() {
                37 => Some(Msg::Left),
                38 => Some(Msg::Rotate),
                39 => Some(Msg::Right),
                40 => Some(Msg::Drop),
                _ => None,
            }
        });

        html! {
            <div tabindex="0" {onkeydown}> // tabindex is needed to listen to keydown events
                <section>
                    <button onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                </section>
                <section class="game-container">
                    <section class="game-area">
                        <div class="game-of-life">
                            { for cell_rows }
                        </div>
                    </section>
                </section>
                <section>
                    <button onclick={ctx.link().callback(|_| Msg::Left)}>{ "Left" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::Rotate)}>{ "Rotate" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::Right)}>{ "Right" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::Drop)}>{ "Drop" }</button>
                </section>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
