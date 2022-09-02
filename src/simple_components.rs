use stylist::css;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, InputEvent};

pub fn draw_slider<
    A: 'static + std::fmt::Display + std::str::FromStr + Copy,
    C: Component<Message = Msg>,
    Msg: 'static,
>(
    ctx: &Context<C>,
    label: &str,
    label_value: A,
    min: A,
    max: A,
    default: A,
    mk_event: fn(A) -> Msg,
) -> Html {
    html! {
        <div>
            <div>
                { format!("{}: {}", label, label_value) }
            </div>
            <input
                type="range"
                oninput={ctx.link().batch_callback(move |e: InputEvent| {
                    let target = e.target();
                    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                    input.map(move |input| mk_event(input.value().parse().unwrap_or(default)))
                })}
                value={format!("{}", label_value)}
                min={format!("{}", min)}
                max={format!("{}", max)}
            />
        </div>
    }
}

pub fn draw_checkbox<C: Component<Message = Msg>, Msg: 'static>(
    ctx: &Context<C>,
    label: &str,
    is_checked: bool,
    mk_event: fn() -> Msg,
) -> Html {
    html! {
        <div>
            <div> {label} </div>
            <input
                type="checkbox"
                oninput={ctx.link().callback(move |_| mk_event())}
                checked={is_checked}
            />
        </div>
    }
}

pub fn draw_button<C: Component<Message = Msg>, Msg: 'static>(
    ctx: &Context<C>,
    button_text: &str,
    mk_event: fn() -> Msg,
) -> Html {
    html! {
        <button onclick={ctx.link().callback(move |_| mk_event())} class={css!("margin: 10px;")}>
            {button_text}
        </button>
    }
}
