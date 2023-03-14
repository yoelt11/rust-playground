# Yew Tutorial

## 1.  A simple hello world app

### Install requirements

```shell
cargo install trunk
rustup target add wasm32-unkown-unknown
```

### Initialize project

```shell
cargo new yew-app
```

### Create/modify files

```toml
# Cargo.toml
[package]
name = "yew-app"
version = "0.1.0"
edition = "2021"

[dependencies]
yew = { version = "0.20", features = ["csr"] } 
# csr: client-side rendering
# ssr: server-side rendering
# hydration: hydration support
```

```rust
// src/main.rs

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

```html
<!-- index.html -->
<!DOCTYPE html>
<html lang="en">
    <head> </head>
    <body></body>
</html>
```

### Run server

```shell
trunk serve --open
```

---

## General file structure

```q
1. index.html: main html file for app.
2. src/main.rs: entry point of the app.
3. src/lib.rs: Rust module that defines the root components
4. src/components/: folder containing all the yew components for the app
5. style.css: css file that styles web app
7. Cargo.toml: project dependencies
-- optional --
8. src/api.rs: contains functions that make http request to the API and return a response data to the app
9. src/controller.rs: handles requests and responses between the client and server, e.g:
   A controller might parse incoming requests, perform business logic, and return 
   appropiate responses.

10. src/state.rs: contains the definition of the application state and related functions and methods.
11. src/model.rs: contains the structs of component states
```

****

## Components (simple)

Components are building blocks that can take arguments in form of properties, can have their own state, and compute pieces of HTML visible to the user (DOM).

```rust
use yew::{function_component, html, Html};

// Reusable component
#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// Then somewhere else you can use the component inside `html!`
#[function_component]
fn App() -> Html {
    html! { <HelloWorld /> }
}
```

### Properties

THese are component arguments that yew can keep track on.

A type has to implement the `Properties` trait before it can be used as the properties of a component.

```rust
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! { <>{"Am I loading? - "}{props.is_loading.clone()}</> }
}

// Then supply the prop
#[function_component]
fn App() -> Html {
    html! {<HelloWorld is_loading={true} />}
}

```



## Complementary libraries

Since wasm is not complete, we rely on javascript where necessary

**wasm-bindgen**: a library and tool that enables calss to javascript tfrom rust and vice-versa.

**web-sys**: provides bindings gor Web APIs.

## Glosary

**Server-side rendering (SSR)**

Is the traditional approach to rendering web pages. In SSR, the server generates the HTML content of a web page and sends it to the client as a complete, ready-to-render document. The client simply displays the HTML as-is, without having to perform any additional processing.

One of the main advantages of SSR is that it allows the initial page load to be faster, as the client receives a pre-rendered HTML document from the server. This can be especially important for content-heavy sites or sites with slow connections, where the delay in rendering the page can lead to poor user experience.

****

**Client-side rendering (CSR)**

on the other hand, involves sending a minimal HTML page to the client, along with JavaScript code that is responsible for rendering the page content. When the JavaScript code is executed by the client's browser, it retrieves data from the server (usually via an API) and dynamically updates the HTML content of the page.

One of the main advantages of CSR is that it allows for a more dynamic user experience, as the page can update without requiring a full page reload. This can be especially important for applications that require real-time updates or have complex user interactions.

****

**Hydration**

Hydration refers to the process of taking an HTML page that was initially rendered by the server (i.e. server-side rendered), and adding JavaScript interactivity to it.

During hydration, the JavaScript code that was responsible for rendering the page on the server is executed again on the client-side. The JavaScript code then attaches event handlers to the elements on the page and establishes the interactive functionality of the page. In addition, the JavaScript code also re-renders the page with any changes made by the user in response to events.

The process of hydration is typically used in client-side rendering (CSR) frameworks such as React or Vue.js. These frameworks initially send a minimal HTML page to the client, along with JavaScript code that is responsible for rendering the page content. Once the JavaScript code is executed on the client-side, it "hydrates" the HTML page, adding the interactive functionality to the static HTML content that was initially sent by the server.

Hydration allows for a faster initial page load by rendering some parts of the page on the server-side and then adding interactivity on the client-side, thus providing the benefits of both server-side rendering (SSR) and client-side rendering (CSR).


















