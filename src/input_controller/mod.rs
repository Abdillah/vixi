mod insert_mode;
mod normal_mode;
mod rpc;

use self::insert_mode::InsertMode;
use self::normal_mode::NormalMode;
use crate::devices::keyboard::Keyboard;
use crate::devices::terminal::Terminal;

use failure::Error;
use xi_rpc::Peer;

enum Mode {
    Normal,
    Insert,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Response {
    Continue,
    Stop,
    SwitchToInsertMode,
    SwitchToNormalMode,
}

impl Mode {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Mode::Normal => String::from("NORMAL"),
            Mode::Insert => String::from("INSERT"),
        }
    }
}

pub struct Controller {
    terminal: Terminal,
    keyboard: Keyboard,
    view_id: String,
    normal_mode: NormalMode,
    insert_mode: InsertMode,
    mode: Mode,
}

impl Controller {
    pub fn new(terminal: Terminal, keyboard: Keyboard) -> Self {
        Self {
            terminal,
            keyboard,
            view_id: String::new(),
            normal_mode: NormalMode::default(),
            insert_mode: InsertMode::default(),
            mode: Mode::Normal,
        }
    }

    pub fn open_file(&mut self, core: &dyn Peer, file_path: &str) -> Result<(), Error> {
        let mut xi_config_dir =
            dirs::config_dir().ok_or_else(|| format_err!("config dir not found"))?;
        xi_config_dir.push("xi");

        core.send_rpc_notification(
            "client_started",
            &json!({ "config_dir": xi_config_dir.to_str().unwrap(), }),
        );

        let view_id = core
            .send_rpc_request("new_view", &json!({ "file_path": file_path }))
            .expect("failed to create the new view");

        self.view_id = view_id.as_str().unwrap().to_string();

        let (size_y, _) = self.terminal.get_size();
        core.send_rpc_notification(
            "edit",
            &json!({
                "method": "scroll",
                "view_id": self.view_id,
                "params": [0 ,size_y + 1] // + 1 bc range not inclusive
            }),
        );

        //core.send_rpc_notification(
        //"plugin",
        //&json!({
        //"command": "plugin_rpc",
        //"view_id": self.view_id,
        //"receiver": "vixi",
        //"rpc": {
        //"method": "add_status_item",
        //"rpc_type": "notification",
        //"params": {
        //"alignment": "left",
        //"key": "change-mode",
        //"value": self.mode.to_string(),
        //}
        //}
        //}),
        //);

        Ok(())
    }

    pub fn start_keyboard_event_loop(&mut self, core: &dyn Peer) -> Result<(), Error> {
        loop {
            let key = self.keyboard.get_next_keystroke();

            let res = match self.mode {
                Mode::Normal => self.normal_mode.handle_keystroke(key, &self.view_id, core),
                Mode::Insert => self.insert_mode.handle_keystroke(key, &self.view_id, core),
            };

            match res {
                Response::Continue => {}
                Response::Stop => break,
                Response::SwitchToInsertMode => self.mode = Mode::Insert,
                Response::SwitchToNormalMode => self.mode = Mode::Normal,
            }
        }

        Ok(())
    }
}