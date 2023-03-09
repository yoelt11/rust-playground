use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub is_visible: bool,
}

#[function_component]
fn Title() -> Html {
    html! { <h1>{"Hello world"}</h1> }
}

#[function_component]
fn Post(props: &PostProps) -> Html {
    if props.is_visible.clone() {
    html! {  <>
        <div>
            <h3>{"Title_1"}</h3>
            <p>{"summary"}</p>
            <p>{"full_text"}</p>
            {props.is_visible.clone()}
        </div>
        </>
        }
    } else {
        html! {}
    }
}

#[function_component(App)]
fn app() -> Html {
    html!{
        <>        
        <Title/>
        <Post is_visible={true}/>
        <Post is_visible={false}/>
        <Post is_visible={false}/>
        </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
