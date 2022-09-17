# tide-jsx

In order to render a simple HTML fragment into a `String`, use the `rsx!` macro to generate a
component tree, and call `render` on it:

```rust
use tide_jsx::{rsx, Render};

let tree = rsx! {
  <div>
    <h1>{"Hello!"}</h1>
    <p>{"Hello world!"}</p>
  </div>
};

assert_eq!(tree.render(), "<div><h1>Hello!</h1><p>Hello world!</p></div>");
```

Because this is so common, there's another macro called `html!` that calls `rsx!` to generate
a component tree, and then calls `render` on it. Most of the time, you'll find yourself using
the `rsx!` macro to compose arbitrary components, and only calling `html!` when you need a
String output, when sending a response or generating a Markdown file.

In Render, attributes and plain strings are escaped using the `render::html_escaping` module. In order to
use un-escaped values so you can dangerously insert raw HTML, use the `raw!` macro around your
string:

```rust
use tide_jsx::{html, raw};

let tree = html! {
  <div>
    <p>{"<Hello />"}</p>
    <p>{raw!("<Hello />")}</p>
  </div>
};

assert_eq!(tree, "<div><p>&lt;Hello /&gt;</p><p><Hello /></p></div>");
```

### Custom components

Render's greatest ability is to provide type-safety along with custom renderable components.
Introducing new components is as easy as defining a function that returns a `Render` value.

In order to build up components from other components or HTML nodes, you can use the `rsx!`
macro, which generates a `Render` component tree:

```rust
use tide_jsx::{component, rsx, html};

#[component]
fn Heading<'title>(title: &'title str) {
  rsx! { <h1 class={"title"}>{title}</h1> }
}

let rendered_html = html! {
  <Heading title={"Hello world!"} />
};

assert_eq!(rendered_html, r#"<h1 class="title">Hello world!</h1>"#);
```

If you pay close attention, you see that the function `Heading` is:

- declared with an uppercase. Underneath, it generates a struct with the same name, and
  implements the `Render` trait on it.
- does not have a return type. This is because everything is written to a writer, for
  performance reasons.

### Visibility & Component Libraries

Often you're going to want to store your components somewhere else in your
project tree other than the module you're working on (if not in a different
module entirely!). In these cases, the visibility applied top the function that
defines your component will flow down into all fields of that struct.

For example, if we add "pub" to the front of our Heading component above:

```rust
#[component]
pub fn Heading<'title>(title: &'title str) {
  rsx! { <h1 class={"title"}>{title}</h1> }
}
```

...the struct that is generated would look something like...

```rust
pub struct Heading {
  pub title: &'title str
}
```

This is important to understand from a safety point of view when structuring
your libraries.

#### Full example

```rust
use render::html::HTML5Doctype;
use render::{component, rsx, html, Render};

#[component]
fn Page<'a, Children: Render>(title: &'a str, children: Children) {
   rsx! {
     <>
       <HTML5Doctype />
       <html>
         <head><title>{title}</title></head>
         <body>
           {children}
         </body>
       </html>
     </>
   }
}

pub fn some_page(user_name: &str) -> String {
    html! {
      <Page title={"Home"}>
        {format!("Welcome, {}", user_name)}
      </Page>
    }
}

```
