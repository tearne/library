// #![windows_subsystem = "windows"]
/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/



use native_windows_gui as nwg;
use native_windows_derive as nwd;

use tunnel::Tunnel;

use self::nwd::NwgUi;
use self::nwg::NativeUi;

mod tunnel;

#[derive(Default, NwgUi)]
pub struct GUI {
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [GUI::terminate] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "remote server", focus: true)]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    destination_edit: nwg::TextInput,

    #[nwg_control(focus: true, password: Some('*'))]
    #[nwg_layout_item(layout: grid, row: 1, col: 0)]
    password: nwg::TextInput,

    #[nwg_control(text: "Go")]
    #[nwg_layout_item(layout: grid, col: 0, row: 2, row_span: 2)]
    #[nwg_events( OnButtonClick: [GUI::start] )]
    start_button: nwg::Button,

    tunnel: Tunnel,
}

impl GUI {
    fn start(&self) {
        nwg::modal_info_message(&self.window, "...", &format!("Start connection to {}", self.destination_edit.text()));
        self.tunnel.start();
    }
    
    fn terminate(&self) {
        nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.destination_edit.text()));
        nwg::stop_thread_dispatch();
    }

}



fn main() {
    println!("Hello, world!");

    let p = std::env::current_dir().unwrap();
    println!("--==--> {:?}", p);

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    // let _app = GUI::build_ui(Default::default()).expect("Failed to build UI");
    let _app = GUI::build_ui(GUI{ tunnel: Tunnel::default(), ..Default::default()}).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}