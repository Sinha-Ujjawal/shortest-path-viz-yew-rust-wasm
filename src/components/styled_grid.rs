use yew::prelude::{classes, html, Children, Classes, Component, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub width: u8,
    pub classes: Classes,
    pub children: Children,
}

pub struct StyledGrid(u8);

impl StyledGrid {
    fn style(&self) -> String {
        format!(
            r#"
                display: grid;
                grid-template-columns: repeat({}, 30px);
                grid-template-rows: auto;
                grid-gap: 10px;
            "#,
            self.0
        )
    }
}

impl Component for StyledGrid {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self(ctx.props().width)
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        self.0 = ctx.props().width;
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props: &Props = ctx.props();
        html! {
            <div style={self.style()} class={classes!(props.classes.clone())}>
                { props.children.clone() }
            </div>
        }
    }
}
