use tide::http::mime;
use tide::utils::After;
use tide::{log, Request, Response};
use tide_jsx::html::HTML5Doctype;
use tide_jsx::{component, rsx, view};

#[component]
fn Heading<'title>(title: &'title str) {
    rsx! { <h1 class={"title"}>{title}</h1> }
}

async fn index(_req: Request<()>) -> tide::Result {
    view! {
      <>
       <HTML5Doctype />
       <html>
         <head><title>{"Tide JSX"}</title></head>
         <body>
             <div>
              <Heading title={"Hello world"} />
            </div>
        </body>
       </html>
     </>
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
    app.with(After(|mut res: Response| async {
        if let Some(err) = res.error() {
            let msg = format!("<h1>Error: {:?}</h1>", err);
            res.set_status(err.status());
            res.set_content_type(mime::HTML);
            res.set_body(msg);
        }
        Ok(res)
    }));
    app.at("/").get(index);
    app.listen("127.0.0.1:5000").await?;
    Ok(())
}
