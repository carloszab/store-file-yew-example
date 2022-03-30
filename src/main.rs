use gloo_file::callbacks::FileReader;
use gloo_file::File;
use log::*;
use reqwest::header::{HeaderMap};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlInputElement};
use yew::{html, html::TargetCast, Component, Context, Html};



type Chunks = bool;

#[derive(Debug)]
pub enum Msg {
    Loaded(String, String),
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>, Chunks),
    ToggleReadBytes,
}

pub struct App {
    readers: HashMap<String, FileReader>,
    files: Vec<String>,
    read_bytes: bool,
}

async fn store_file(file: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "image/png".parse().unwrap());
    headers.insert("x-ms-blob-type", "BlockBlob".parse().unwrap());
    // headers.insert("Content-Length", file.metadata().unwrap().len().to_string().parse().unwrap());

    // let body = if let Ok(mut f) = std::fs::File::open("assets/avatar.png") {
    //     let mut buffer = vec![0; f.metadata().unwrap().len() as usize];
    //     if let Ok(_) = f.read(&mut buffer) {
    //         Some(buffer)
    //     } else {None}
    // } else {None};

    let body = Some(file);

    if body.is_some() {
        let client = reqwest::Client::new();
        let res = client
            .put("url")
            .headers(headers)
            .body(body.unwrap())
            .send()
            .await?;
    
        println!("response: {:?}", res);
    } else {
        println!("error al procesar archivo")
    }
    Ok(())
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: vec![],
            read_bytes: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("msg: {:?}", msg);
        println!("msg: {:?}", msg);
        match msg {
            Msg::Loaded(file_name, data) => {
                let info = format!("file_name: {}, data: {:?}", file_name, data);
                self.files.push(info);
                // info!("loaded file_name: {}, data: {:?}", file_name, data);
                self.readers.remove(&file_name);
                true
            }
            Msg::LoadedBytes(file_name, data) => {
                let info = format!("file_name: {}, data: {:?}", file_name, data);
                self.files.push(info);
                // info!("loaded bytes file_name: {}, data: {:?}", file_name, data);
                self.readers.remove(&file_name);

                spawn_local(async move {
                    let response = store_file(data).await;
                    // let response = reset_credential_request(vars).await;
                    match response {
                        Ok(msg) => info!("storeFile ok: {:?}", msg),
                        Err(_e) => {},
                    }
                });

                true
            }
            Msg::Files(files, bytes) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();

                        if bytes {
                            gloo_file::callbacks::read_as_bytes(&file, move |res| {
                                link.send_message(Msg::LoadedBytes(
                                    file_name,
                                    res.expect("failed to read file"),
                                ))
                            })
                        } else {
                            gloo_file::callbacks::read_as_text(&file, move |res| {
                                link.send_message(Msg::Loaded(
                                    file_name,
                                    res.unwrap_or_else(|e| e.to_string()),
                                ))
                            })
                        }
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
            Msg::ToggleReadBytes => {
                self.read_bytes = !self.read_bytes;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let flag = self.read_bytes;
        html! {
            <div>
                <div>
                    <p>{ "Choose a file to upload to see the uploaded bytes" }</p>
                    <input type="file" multiple=true onchange={ctx.link().callback(move |e: Event| {
                            let mut result = Vec::new();
                            let input: HtmlInputElement = e.target_unchecked_into();

                            if let Some(files) = input.files() {
                                let files = js_sys::try_iter(&files)
                                    .unwrap()
                                    .unwrap()
                                    .map(|v| web_sys::File::from(v.unwrap()))
                                    .map(File::from);
                                result.extend(files);
                            }
                            Msg::Files(result, flag)
                        })}
                    />
                </div>
                <div>
                    <label>{ "Read bytes" }</label>
                    <input type="checkbox" checked={flag} onclick={ctx.link().callback(|_| Msg::ToggleReadBytes)} />
                </div>
                <ul>
                    { for self.files.iter().map(|f| Self::view_file(f)) }
                </ul>
            </div>
        }
    }
}

impl App {
    fn view_file(data: &str) -> Html {
        html! {
            <li>{ data }</li>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<App>();
}