use std::collections::HashSet;
use stylist::{css, StyleSource};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod components;

const MIN_WIDTH: u8 = 10;
const MAX_WIDTH: u8 = 50;
const MIN_HEIGHT: u8 = 10;
const MAX_HEIGHT: u8 = 50;

#[derive(Debug, Clone, PartialEq)]
enum SymbolType {
    Start,
    End,
    Obstacle,
}

impl SymbolType {
    pub fn to_string(&self) -> String {
        use SymbolType::*;
        match self {
            Start => "🟢".to_owned(),
            End => "🔴".to_owned(),
            Obstacle => "🚧".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ClickButtonType {
    ST(SymbolType),
    Delete,
}

impl ClickButtonType {
    pub fn to_string(&self) -> String {
        use ClickButtonType::*;
        match self {
            ST(st) => format!("Click on grid cell to put {}", st.to_string()),
            Delete => "Click on grid cell to clear the symbol".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
struct Position {
    y: u8,
    x: u8,
}

#[derive(Debug)]
struct Model {
    start: Option<Position>,
    end: Option<Position>,
    obstacles: HashSet<Position>,
    path: HashSet<Position>,
    click_button_type: ClickButtonType,
    width: u8,
    height: u8,
    allow_diagonal: bool,
    disable_input: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum Msg {
    SwitchClickButtonType(ClickButtonType),
    ApplyClickButtonTypeOnCell(Position),
    ComputePath,
    Reset,
    SetWidth(u8),
    SetHeight(u8),
    ToggleDiagonal,
    // SetPath(HashSet<Position>),
}

impl Model {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            start: None,
            end: None,
            obstacles: HashSet::new(),
            path: HashSet::new(),
            click_button_type: ClickButtonType::ST(SymbolType::Start),
            width,
            height,
            allow_diagonal: false,
            disable_input: false,
        }
    }
}

impl Model {
    fn draw_button(is_disabled: bool, button_text: &str, onclick: Callback<MouseEvent>) -> Html {
        html! {
            <button onclick={onclick} class={css!("margin: 10px;")} disabled={is_disabled}>
                {button_text}
            </button>
        }
    }

    fn draw_start_button(&self, ctx: &Context<Self>) -> Html {
        Self::draw_button(
            self.disable_input,
            &SymbolType::Start.to_string(),
            ctx.link()
                .callback(|_| Msg::SwitchClickButtonType(ClickButtonType::ST(SymbolType::Start))),
        )
    }

    fn draw_end_button(&self, ctx: &Context<Self>) -> Html {
        Self::draw_button(
            self.disable_input,
            &SymbolType::End.to_string(),
            ctx.link()
                .callback(|_| Msg::SwitchClickButtonType(ClickButtonType::ST(SymbolType::End))),
        )
    }

    fn draw_obstacle_button(&self, ctx: &Context<Self>) -> Html {
        Self::draw_button(
            self.disable_input,
            &SymbolType::Obstacle.to_string(),
            ctx.link().callback(|_| {
                Msg::SwitchClickButtonType(ClickButtonType::ST(SymbolType::Obstacle))
            }),
        )
    }

    fn draw_delete_button(&self, ctx: &Context<Self>) -> Html {
        Self::draw_button(
            self.disable_input,
            "x",
            ctx.link()
                .callback(|_| Msg::SwitchClickButtonType(ClickButtonType::Delete)),
        )
    }

    fn draw_current_click_button_type_message(&self) -> Html {
        html! {
            <div>
                { self.click_button_type.to_string() }
            </div>
        }
    }

    fn draw_compute_path_button(&self, ctx: &Context<Self>) -> Html {
        Self::draw_button(
            self.disable_input,
            "Compute Path",
            ctx.link().callback(|_| Msg::ComputePath),
        )
    }

    fn draw_reset_button(&self, ctx: &Context<Self>) -> Html {
        Self::draw_button(
            self.disable_input,
            "Reset",
            ctx.link().callback(|_| Msg::Reset),
        )
    }

    fn draw_slider<A: std::fmt::Display>(
        ctx: &Context<Self>,
        is_disabled: bool,
        label: &str,
        label_value: A,
        min: A,
        max: A,
        mk_event: fn(String) -> Msg,
    ) -> Html {
        let link = ctx.link();

        html! {
            <div>
                <div>
                    { format!("{}: {}", label, label_value) }
                </div>
                <input
                    type="range"
                    oninput={link.batch_callback(move |e: InputEvent| {
                        let target = e.target();
                        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                        input.map(|input| mk_event(input.value()))
                    })}
                    value={format!("{}", label_value)}
                    min={format!("{}", min)}
                    max={format!("{}", max)}
                    disabled={is_disabled}
                />
            </div>
        }
    }

    fn draw_set_width_slider(&self, ctx: &Context<Self>) -> Html {
        Self::draw_slider(
            ctx,
            self.disable_input,
            "Width",
            self.width,
            MIN_WIDTH,
            MAX_WIDTH,
            |data| Msg::SetWidth(data.parse::<u8>().unwrap_or(0)),
        )
    }

    fn draw_set_height_slider(&self, ctx: &Context<Self>) -> Html {
        Self::draw_slider(
            ctx,
            self.disable_input,
            "Height",
            self.height,
            MIN_HEIGHT,
            MAX_HEIGHT,
            |data| Msg::SetHeight(data.parse::<u8>().unwrap_or(0)),
        )
    }

    fn draw_checkbox(
        is_disabled: bool,
        label: &str,
        is_checked: bool,
        onclick: Callback<InputEvent>,
    ) -> Html {
        html! {
            <div>
                <div> {label} </div>
                <input
                    type="checkbox"
                    oninput={onclick}
                    checked={is_checked}
                    disabled={is_disabled}
                />
            </div>
        }
    }

    fn draw_allow_diagonal_checkbox(&self, ctx: &Context<Self>) -> Html {
        Self::draw_checkbox(
            self.disable_input,
            "Allow Diagonal",
            self.allow_diagonal,
            ctx.link().callback(|_| Msg::ToggleDiagonal),
        )
    }

    fn cell_style() -> StyleSource<'static> {
        css!(
            r#"
                border: 1px solid rgba(0, 0, 0, 0.8);
                height: 30px;
                width: 30px;
                justify-content: center;
                display: flex;
                align-items: center;
            "#
        )
    }

    fn draw_empty_cell(id: String, onclick: Option<Callback<MouseEvent>>) -> Html {
        html! {
            <div id={id} onclick={onclick} class={Self::cell_style()}>
            </div>
        }
    }

    fn draw_start_cell(id: String, onclick: Option<Callback<MouseEvent>>) -> Html {
        html! {
            <div id={id} onclick={onclick} class={Self::cell_style()}>
                {SymbolType::Start.to_string()}
            </div>
        }
    }

    fn draw_end_cell(id: String, onclick: Option<Callback<MouseEvent>>) -> Html {
        html! {
            <div id={id} onclick={onclick} class={Self::cell_style()}>
                {SymbolType::End.to_string()}
            </div>
        }
    }

    fn draw_obstacle_cell(id: String, onclick: Option<Callback<MouseEvent>>) -> Html {
        html! {
            <div id={id} onclick={onclick} class={Self::cell_style()}>
                {SymbolType::Obstacle.to_string()}
            </div>
        }
    }

    fn draw_path_cell(id: String, onclick: Option<Callback<MouseEvent>>) -> Html {
        html! {
            <div id={id} onclick={onclick} class={classes!(css!("background-color: black;"), Self::cell_style())}>
            </div>
        }
    }

    fn draw_cell(&self, y: u8, x: u8, ctx: &Context<Self>) -> Html {
        let pos = Position { y, x };
        let onclick = if self.disable_input {
            None
        } else {
            Some(
                ctx.link()
                    .callback(move |_| Msg::ApplyClickButtonTypeOnCell(pos)),
            )
        };
        let id = format!("{},{}", y, x);
        if Some(pos) == self.start {
            Self::draw_start_cell(id, onclick)
        } else if Some(pos) == self.end {
            Self::draw_end_cell(id, onclick)
        } else if self.obstacles.contains(&pos) {
            Self::draw_obstacle_cell(id, onclick)
        } else if self.path.contains(&pos) {
            Self::draw_path_cell(id, onclick)
        } else {
            Self::draw_empty_cell(id, onclick)
        }
    }

    fn draw_grid(&self, ctx: &Context<Self>) -> Html {
        let grid_items = ((0..self.height).into_iter().flat_map(|y| {
            (0..self.width)
                .into_iter()
                .map(move |x| self.draw_cell(y, x, ctx))
        }))
        .collect::<Html>();

        let style = format!(
            r#"
                display: grid;
                grid-template-columns: repeat({}, 30px);
                grid-template-rows: auto;
                grid-gap: 10px;
            "#,
            self.width
        );

        html! {
            <div style={style}>
                {grid_items}
            </div>
        }
    }
}

impl Model {
    fn symbol_type_at_position(&self, pos: &Position) -> Option<SymbolType> {
        if Some((*pos).clone()) == self.start {
            Some(SymbolType::Start)
        } else if Some((*pos).clone()) == self.end {
            Some(SymbolType::End)
        } else if self.obstacles.contains(pos) {
            Some(SymbolType::Obstacle)
        } else {
            None
        }
    }

    fn apply_click_button_type_on_cell(&mut self, pos: &Position) {
        match self.symbol_type_at_position(pos) {
            Some(st) => match self.click_button_type {
                ClickButtonType::Delete => match st {
                    SymbolType::Start => {
                        self.start = None;
                    }
                    SymbolType::End => {
                        self.end = None;
                    }
                    SymbolType::Obstacle => {
                        self.obstacles.remove(pos);
                    }
                },
                _ => {}
            },
            None => match self.click_button_type {
                ClickButtonType::ST(SymbolType::Start) => {
                    self.start = Some((*pos).clone());
                }
                ClickButtonType::ST(SymbolType::End) => {
                    self.end = Some((*pos).clone());
                }
                ClickButtonType::ST(SymbolType::Obstacle) => {
                    self.obstacles.insert((*pos).clone());
                }
                _ => {}
            },
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::new(MIN_WIDTH, MIN_HEIGHT)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;
        match msg {
            SwitchClickButtonType(cbt) => {
                self.click_button_type = cbt;
                true
            }
            ApplyClickButtonTypeOnCell(pos) => {
                self.apply_click_button_type_on_cell(&pos);
                true
            }
            ComputePath => true,
            Reset => {
                *self = Self::create(ctx);
                true
            }
            SetWidth(width) => {
                self.width = width;
                true
            }
            SetHeight(height) => {
                self.height = height;
                true
            }
            ToggleDiagonal => {
                self.allow_diagonal = !self.allow_diagonal;
                true
            } // SetPath(path) => {
              //     self.path = path;
              //     self.disable_input = false;
              //     true
              // }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={css!("padding: 10px;")}>
                { self.draw_grid(ctx) }
                <div class={css!("padding: 10px;")}>
                    { self.draw_start_button(ctx) }
                    { self.draw_end_button(ctx) }
                    { self.draw_obstacle_button(ctx) }
                    { self.draw_delete_button(ctx) }
                </div>
                { self.draw_current_click_button_type_message() }
                { self.draw_compute_path_button(ctx) }
                { self.draw_reset_button(ctx) }
                { self.draw_set_width_slider(ctx) }
                { self.draw_set_height_slider(ctx) }
                { self.draw_allow_diagonal_checkbox(ctx) }
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
