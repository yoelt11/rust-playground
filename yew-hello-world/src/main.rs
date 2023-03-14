use gloo::console;
use yew::{html, Component, Context, Html};

// Define the possible messages which can be sent to the component
pub enum Msg {
    ChangeTheme,
}

pub struct App {
    darktheme: bool, // This will store the counter value
    title: String 
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { darktheme: false, title: String::from("hello") }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeTheme => {
                self.darktheme = !self.darktheme;
                if self.title.eq("hello") {
                    self.title = String::from("goodbye");
                } else {
                    self.title = String::from("hello");
                }
                console::log!("changed theme"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme =  match &self.darktheme {
            true => "dark-mode",
            false => "light-mode"
        };

        html! {
                <>
                    <div id={"data-card"} class={theme}> 
                        <h1 class={theme}>{ &self.title}{" world!"}</h1>
                        <button class={theme} onclick={ctx.link().callback(|_| Msg::ChangeTheme)}>{"click me"}</button>
                    </div>
                </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}