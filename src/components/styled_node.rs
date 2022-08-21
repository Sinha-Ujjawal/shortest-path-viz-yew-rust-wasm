use stylist::{css, StyleSource, YieldStyle};
use web_sys::MouseEvent;
use yew::{
    prelude::{classes, html, Children, Component, Context, Html, Properties},
    Callback, Classes,
};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub classes: Classes,
    pub children: Children,
    pub onclick: Option<Callback<MouseEvent>>,
}

pub struct StyledNode;

impl YieldStyle for StyledNode {
    fn style_from(&self) -> StyleSource<'static> {
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
}

impl Component for StyledNode {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        match props.onclick.clone() {
            Some(cb) => {
                html! {
                    <div id={props.id.clone()} onclick={cb} class={classes!(self.style(), props.classes.clone())}>
                        { props.children.clone() }
                    </div>
                }
            }
            None => {
                html! {
                    <div id={props.id.clone()} class={classes!(self.style(), props.classes.clone())}>
                        { props.children.clone() }
                    </div>
                }
            }
        }
    }
}
