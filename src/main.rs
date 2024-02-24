use yew::{platform::spawn_local, prelude::*};
use base64::{engine::general_purpose, Engine};
use std::ops::Deref;
use gloo_file::File;
use web_sys::{FileList, HtmlInputElement};

#[function_component(App)]
fn app() -> Html {
    let cat_list = use_state(|| Vec::<CatDetail>::new());

    let on_change = {
        let cat_list = cat_list.clone();
        move |e: Event| {
            let cat_list = cat_list.clone();
            spawn_local(async move {
                let input: HtmlInputElement = e.target_unchecked_into();
                let files = upload_file(input.files());

                let mut interior_cat_list = cat_list.deref().clone();

                for file in files {
                    let new_detail = CatDetail {
                        name: file.name(),
                        image: gloo_file::futures::read_as_bytes(&file).await.unwrap(),
                    };

                    interior_cat_list.push(new_detail);
                }
                
                cat_list.set(interior_cat_list);
            })
        }     
    };


    html! {
        <div>
            <h1>{"Catdex"}</h1>

            <input type="file" accept="image/*" onchange={on_change} />

            <section>
                { for cat_list.iter().map(cat) }
            </section>
        </div>
    }
}

#[derive(Clone)]
struct CatDetail {
    name: String,
    image: Vec<u8>,
}

fn cat(cat: &CatDetail) -> Html {
    html! {
        <article class="cat">
            <h3>{ format!("{}", cat.name) }</h3>
            <img src={format!("data:image;base64,{}", general_purpose::STANDARD.encode(&cat.image))} />
        </article>
    }
}

fn upload_file(files: Option<FileList>) -> Vec<File> {
    files
        .map(|files| {
            js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from)
                .collect()
        })
        .unwrap_or_default()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
