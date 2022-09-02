use std::collections::HashSet;
use stylist::{css, StyleSource};
use yew::{classes, html, Callback, Component, Context, Html, MouseEvent};

mod bfs;
mod simple_components;

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

impl std::fmt::Display for SymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SymbolType::*;
        f.write_str(match self {
            Start => "🟢",
            End => "🔴",
            Obstacle => "🚧",
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ClickButtonType {
    ST(SymbolType),
    Delete,
}

impl std::fmt::Display for ClickButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ClickButtonType::*;
        match self {
            ST(st) => f.write_str(&format!("Click on grid cell to put {}", st)),
            Delete => f.write_str("Click on grid cell to clear the symbol"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
struct Position {
    y: u8,
    x: u8,
}

impl Position {
    fn simple_neighbors(&self) -> HashSet<Position> {
        let mut ret: HashSet<Position> = HashSet::new();
        if self.y > 0 {
            ret.insert(Position {
                y: self.y - 1,
                x: self.x,
            });
        }
        if self.y < u8::MAX {
            ret.insert(Position {
                y: self.y + 1,
                x: self.x,
            });
        }
        if self.x > 0 {
            ret.insert(Position {
                y: self.y,
                x: self.x - 1,
            });
        }
        if self.x < u8::MAX {
            ret.insert(Position {
                y: self.y,
                x: self.x + 1,
            });
        }
        ret
    }

    fn neighbors_with_diagonals(&self) -> HashSet<Position> {
        let mut ret = self.simple_neighbors();
        if self.y > 0 && self.x > 0 {
            ret.insert(Position {
                y: self.y - 1,
                x: self.x - 1,
            });
        }
        if self.y > 0 && self.x < u8::MAX {
            ret.insert(Position {
                y: self.y - 1,
                x: self.x + 1,
            });
        }
        if self.y < u8::MAX && self.x > 0 {
            ret.insert(Position {
                y: self.y + 1,
                x: self.x - 1,
            });
        }
        if self.y < u8::MAX && self.x < u8::MAX {
            ret.insert(Position {
                y: self.y + 1,
                x: self.x + 1,
            });
        }
        ret
    }
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
        }
    }
}

impl Model {
    fn draw_start_button(&self, ctx: &Context<Self>) -> Html {
        simple_components::draw_button(ctx, &SymbolType::Start.to_string(), || {
            Msg::SwitchClickButtonType(ClickButtonType::ST(SymbolType::Start))
        })
    }

    fn draw_end_button(&self, ctx: &Context<Self>) -> Html {
        simple_components::draw_button(ctx, &SymbolType::End.to_string(), || {
            Msg::SwitchClickButtonType(ClickButtonType::ST(SymbolType::End))
        })
    }

    fn draw_obstacle_button(&self, ctx: &Context<Self>) -> Html {
        simple_components::draw_button(ctx, &SymbolType::Obstacle.to_string(), || {
            Msg::SwitchClickButtonType(ClickButtonType::ST(SymbolType::Obstacle))
        })
    }

    fn draw_delete_button(&self, ctx: &Context<Self>) -> Html {
        simple_components::draw_button(ctx, "x", || {
            Msg::SwitchClickButtonType(ClickButtonType::Delete)
        })
    }

    fn draw_current_click_button_type_message(&self) -> Html {
        html! {
            <div>
                { self.click_button_type.to_string() }
            </div>
        }
    }

    fn draw_compute_path_button(&self, ctx: &Context<Self>) -> Html {
        simple_components::draw_button(ctx, "Compute Path", || Msg::ComputePath)
    }

    fn draw_reset_button(&self, ctx: &Context<Self>) -> Html {
        simple_components::draw_button(ctx, "Reset", || Msg::Reset)
    }

    fn draw_set_width_slider(&self, ctx: &Context<Self>) -> Html {
        simple_components::draw_slider(
            ctx,
            "Width",
            self.width,
            MIN_WIDTH,
            MAX_WIDTH,
            0,
            Msg::SetWidth,
        )
    }

    fn draw_set_height_slider(&self, ctx: &Context<Self>) -> Html {
        simple_components::draw_slider(
            ctx,
            "Height",
            self.height,
            MIN_HEIGHT,
            MAX_HEIGHT,
            0,
            Msg::SetHeight,
        )
    }

    fn draw_allow_diagonal_checkbox(&self, ctx: &Context<Self>) -> Html {
        simple_components::draw_checkbox(ctx, "Allow Diagonal", self.allow_diagonal, || {
            Msg::ToggleDiagonal
        })
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

    fn draw_empty_cell(id: String, onclick: Callback<MouseEvent>) -> Html {
        html! {
            <div id={id} onclick={onclick} class={Self::cell_style()}>
            </div>
        }
    }

    fn draw_start_cell(id: String, onclick: Callback<MouseEvent>) -> Html {
        html! {
            <div id={id} onclick={onclick} class={Self::cell_style()}>
                {SymbolType::Start.to_string()}
            </div>
        }
    }

    fn draw_end_cell(id: String, onclick: Callback<MouseEvent>) -> Html {
        html! {
            <div id={id} onclick={onclick} class={Self::cell_style()}>
                {SymbolType::End.to_string()}
            </div>
        }
    }

    fn draw_obstacle_cell(id: String, onclick: Callback<MouseEvent>) -> Html {
        html! {
            <div id={id} onclick={onclick} class={Self::cell_style()}>
                {SymbolType::Obstacle.to_string()}
            </div>
        }
    }

    fn draw_path_cell(id: String, onclick: Callback<MouseEvent>) -> Html {
        html! {
            <div id={id} onclick={onclick} class={classes!(css!("background-color: black;"), Self::cell_style())}>
            </div>
        }
    }

    fn draw_cell(&self, y: u8, x: u8, ctx: &Context<Self>) -> Html {
        let pos = Position { y, x };
        let onclick = ctx
            .link()
            .callback(move |_| Msg::ApplyClickButtonTypeOnCell(pos));
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
        if Some(*pos) == self.start {
            Some(SymbolType::Start)
        } else if Some(*pos) == self.end {
            Some(SymbolType::End)
        } else if self.obstacles.contains(pos) {
            Some(SymbolType::Obstacle)
        } else {
            None
        }
    }

    fn apply_click_button_type_on_cell(&mut self, pos: &Position) {
        match self.symbol_type_at_position(pos) {
            Some(st) => {
                if self.click_button_type == ClickButtonType::Delete {
                    match st {
                        SymbolType::Start => {
                            self.start = None;
                        }
                        SymbolType::End => {
                            self.end = None;
                        }
                        SymbolType::Obstacle => {
                            self.obstacles.remove(pos);
                        }
                    }
                } else {
                }
            }
            None => match self.click_button_type {
                ClickButtonType::ST(SymbolType::Start) => {
                    self.start = Some(*pos);
                }
                ClickButtonType::ST(SymbolType::End) => {
                    self.end = Some(*pos);
                }
                ClickButtonType::ST(SymbolType::Obstacle) => {
                    self.obstacles.insert(*pos);
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
                self.path = HashSet::new();
                true
            }
            ComputePath => match (self.start, self.end) {
                (Some(start), Some(end)) => {
                    let obstacles = &self.obstacles;
                    let width = self.width;
                    let height = self.height;
                    let allow_diagonal = self.allow_diagonal;
                    self.path = bfs::shortest_path(start, end, &move |pos: Position| {
                        (if allow_diagonal {
                            pos.neighbors_with_diagonals()
                        } else {
                            pos.simple_neighbors()
                        })
                        .into_iter()
                        .filter(|p| (p.x < width) && (p.y < height) && (!obstacles.contains(p)))
                        .collect()
                    })
                    .into_iter()
                    .collect();
                    true
                }
                _ => false,
            },
            Reset => {
                *self = Self::create(ctx);
                true
            }
            SetWidth(width) => {
                self.width = width;
                self.path = HashSet::new();
                true
            }
            SetHeight(height) => {
                self.height = height;
                self.path = HashSet::new();
                true
            }
            ToggleDiagonal => {
                self.allow_diagonal = !self.allow_diagonal;
                self.path = HashSet::new();
                true
            }
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
