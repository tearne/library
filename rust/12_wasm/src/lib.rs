mod app;
pub mod plot;
pub mod data;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use geojson::GeoJson;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

async fn get_geojson() -> GeoJson {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::SameOrigin);

    let url = "data/outline.json";

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.expect("b");

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().expect("d");

    let json = JsFuture::from(resp.json().unwrap()).await.expect("a");

    json.into_serde::<GeoJson>().unwrap()
}


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start_canvas(canvas_id: String) {
    let app = app::TemplateApp::default();
    eframe::start_web(&canvas_id, Box::new(app)).unwrap();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p").unwrap();
    val.set_inner_html("Hello using web-sys");
    body.append_child(&val).unwrap();

    let gj = get_geojson().await;
    log!("You got json: {}", gj);

    use geojson::*;

    let fc = match gj {
        GeoJson::FeatureCollection(ref fc) => fc,
        _ => unreachable!()
    };

    let gv = &fc
        .features[0]
        .geometry
        .as_ref()
        .unwrap()
        .value;

    let outline = match gv {
        Value::Polygon(p) => &p[0],
        _ => unreachable!()
    };

    log!("outline {:?}", outline);





    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Step{
        D(Decision),
        F(Forwarder),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Decision {
        pub name: String,
        pub text: String,
        pub opts: Vec<Opt>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Opt{
        pub text: String, 
        pub goto: String,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Forwarder {
        pub name: String,
        pub text: String,
        pub goto: String,
    }

    let t = Step::D(Decision{
        name: String::from("A name"),
        text: String::from("Text, a,b,c"),
        opts: vec![
            Opt{ 
                text: String::from("do A"),
                goto: String::from("StepA")
            },
            Opt{ 
                text: String::from("do B"),
                goto: String::from("StepB")
            }
        ]
    });

    let j = serde_json::to_string(&t).unwrap();
    log!("---- {}", j);
}

