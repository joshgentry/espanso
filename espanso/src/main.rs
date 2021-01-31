use espanso_detect::event::{InputEvent, Status};
use espanso_ui::menu::*;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};

fn main() {
  println!("Hello, world!z");
  CombinedLogger::init(vec![
    TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed),
    // WriteLogger::new(
    //   LevelFilter::Info,
    //   Config::default(),
    //   File::create("my_rust_binary.log").unwrap(),
    // ),
  ])
  .unwrap();

  let icon_paths = vec![
    (
      espanso_ui::icons::TrayIcon::Normal,
      r"C:\Users\Freddy\AppData\Local\espanso\espanso.ico".to_string(),
    ),
    (
      espanso_ui::icons::TrayIcon::Disabled,
      r"C:\Users\Freddy\AppData\Local\espanso\espansored.ico".to_string(),
    ),
  ];

 
  // let (remote, mut eventloop) = espanso_ui::win32::create(espanso_ui::win32::Win32UIOptions {
  //   show_icon: true,
  //   icon_paths: &icon_paths,
  //   notification_icon_path: r"C:\Users\Freddy\Insync\Development\Espanso\Images\icongreensmall.png"
  //     .to_string(),
  // });

  let handle = std::thread::spawn(move || {
    //let mut source = espanso_detect::win32::Win32Source::new();
    let mut source = espanso_detect::x11::X11Source::new();
    source.initialize();
    source.eventloop(Box::new(move |event: InputEvent| {
      println!("ev {:?}", event);
      match event {
        InputEvent::Mouse(_) => {}
        InputEvent::Keyboard(evt) => {
          if evt.key == espanso_detect::event::Key::Shift && evt.status == Status::Pressed {
            //remote.update_tray_icon(espanso_ui::icons::TrayIcon::Disabled);
            //remote.show_notification("Espanso is running!");
          }
        }
      }
    }));
  });

  // eventloop.initialize();
  // eventloop.run(Box::new(move |event| {
  //   println!("ui {:?}", event);
  //   let menu = Menu::from(vec![
  //     MenuItem::Simple(SimpleMenuItem::new("open", "Open")),
  //     MenuItem::Separator,
  //     MenuItem::Sub(SubMenuItem::new(
  //       "Sub",
  //       vec![
  //         MenuItem::Simple(SimpleMenuItem::new("sub1", "Sub 1")),
  //         MenuItem::Simple(SimpleMenuItem::new("sub2", "Sub 2")),
  //       ],
  //     )),
  //   ])
  //   .unwrap();
  //   remote.show_context_menu(&menu);
  // }))
  handle.join();
}