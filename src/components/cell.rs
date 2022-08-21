use crate::components::styled_node::StyledNode;
use web_sys::MouseEvent;
use yew::{
    prelude::{classes, html, Classes, Component, Properties},
    Callback,
};

#[derive(PartialEq, Properties)]
pub struct EmptyCellProps {
    pub id: String,
    pub classes: Classes,
    pub onclick: Option<Callback<MouseEvent>>,
}

pub struct EmptyCell;

impl Component for EmptyCell {
    type Message = ();
    type Properties = EmptyCellProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();
        html! {
            <StyledNode
                id={props.id.clone()}
                classes={ classes!(props.classes.clone()) }
                onclick={props.onclick.clone()}
            >
                <>
                </>
            </StyledNode>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct CellWithTextProps {
    pub id: String,
    pub text: String,
    pub classes: Classes,
    pub onclick: Option<Callback<MouseEvent>>,
}

pub struct CellWithText(String);

impl Component for CellWithText {
    type Message = ();
    type Properties = CellWithTextProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self(ctx.props().text.clone())
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();
        html! {
            <StyledNode
                id={props.id.clone()}
                onclick={props.onclick.clone()}
                classes={classes!(props.classes.clone())}
            >
                { self.0.clone() }
            </StyledNode>
        }
    }
}
