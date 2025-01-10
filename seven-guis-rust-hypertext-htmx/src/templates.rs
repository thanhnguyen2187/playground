use hypertext::{rsx, rsx_static, Renderable, html_elements, GlobalAttributes, Rendered};

pub fn index(title: String, body_fn: fn(&mut String)) -> fn(&mut String) {
    let body = body_fn.render();
    let template_fn = rsx! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <meta http-equiv="X-UA-Compatible" content="ie=edge">
                <title>{title}</title>
                <link rel="stylesheet" href="https://matcha.mizu.sh/matcha.css">
            </head>
            <body>
                {body}
            </body>
        </html>
    };

    template_fn
}