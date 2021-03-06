extern crate chrono;
extern crate pancurses;

use chat_window::chrono::prelude::{DateTime, Local};
use pancurses::{init_pair, noecho, Input, Window};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

mod client;

const FG_DEFAULT: color = -1;
const BG_RECEIVE_DEFAULT: color = pancurses::COLOR_MAGENTA;
const BG_SEND_DEFAULT: color = pancurses::COLOR_CYAN;
const BG_SCREEN_DEFAULT: color = -1;

const PAIR_OLD_RECEIVED: color = 1;
const PAIR_DEFAULT: color = 2;
const PAIR_OLD_SENT: color = 3;
const PAIR_TYPING: color = 4;
const PAIR_BORDER: color = 5;

fn set_color_pairs() {
    init_pair(
        PAIR_OLD_RECEIVED,
        pancurses::COLOR_WHITE,
        BG_RECEIVE_DEFAULT,
    );
    init_pair(PAIR_DEFAULT, pancurses::COLOR_GREEN, BG_SCREEN_DEFAULT);
    init_pair(PAIR_OLD_SENT, pancurses::COLOR_BLACK, BG_SEND_DEFAULT);
    init_pair(PAIR_TYPING, FG_DEFAULT, BG_SCREEN_DEFAULT);
    init_pair(PAIR_BORDER, pancurses::COLOR_RED, BG_SCREEN_DEFAULT);
}

const INPUT_BOX_HEIGHT: i32 = 6;
const INPUT_BOX_LEFT: i32 = 2;

#[derive(Serialize, Deserialize)]
enum MsgType {
    Text,
    Image,
}

#[derive(Serialize, Deserialize)]
struct MsgMetaData {
    to: String,
    from: String,
    timestamp: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
enum Message {
    Raw {
        meta_data: MsgMetaData,
        text: String,
        msg_type: MsgType,
    },
    Text {
        meta_data: MsgMetaData,
        msg_lines: MsgLines,
    },
    Image {
        meta_data: MsgMetaData,
        url: String,
    },
}

pub struct ChatWindow<'w> {
    pending_msgs: VecDeque<Message>,
    window: &'w Window,
}

type color = i16;
type MsgLines = Vec<Vec<String>>;

impl<'t> ChatWindow<'t> {
    pub fn new<'a>(window: &'a Window) -> ChatWindow<'a> {
        ChatWindow {
            pending_msgs: VecDeque::new(),
            window,
        }
    }

    fn load_old_msgs(&self) -> Vec<Message> {
        let old_msgs: Vec<Message> = Vec::new();
        old_msgs
    }

    fn max_msg_width(&self) -> i32 {
        self.window.get_max_x() / 3
    }

    /*
     * Returns the text stored in a message object. The resulting vec
     * contains a Vec<String> to represent each line, and the Vec<String>
     * is split into words.
     */
    fn wrap_str(&self, wrap_width: i32, txt: &String) -> MsgLines {
        let split = txt.split(' ');
        let mut lines: Vec<Vec<String>> = Vec::new();
        let mut curr_ln: Vec<String> = Vec::new();
        let mut line_chrs = 0;
        // ignore other delimiters for now
        for s in split {
            let s_len = s.chars().count();
            // if it's too long, put it on its own line
            if s_len >= wrap_width as usize {
                // push last string in
                if curr_ln.len() > 0 {
                    lines.push(curr_ln);
                }
                // push this string in, one segment at a time
                // TODO split string up if it's too long
                lines.push(vec![s.to_string()]);
                curr_ln = Vec::new();
                line_chrs = 0;
            } else if line_chrs + s_len >= wrap_width as usize {
                // time for a new line
                lines.push(curr_ln);
                curr_ln = Vec::new();
                curr_ln.push(s.to_string());
                line_chrs = s_len;
            } else {
                // continue adding on this line
                curr_ln.push(s.to_string());
                line_chrs += s_len;
            }
        }
        if curr_ln.len() > 0 {
            lines.push(curr_ln);
        }
        return lines;
    }

    fn send_msg(&self, socket: &mut client::Socket, txt: &String) -> Message {
        let msg = Message::Raw {
            meta_data: MsgMetaData {
                to: "you".to_string(),
                from: "me".to_string(),
                timestamp: Local::now(),
            },
            msg_type: MsgType::Text,
            text: txt.to_string(),
        };

        socket.send_message(bincode::serialize(&msg).unwrap());

        Message::Text {
            meta_data: MsgMetaData {
                to: "you".to_string(),
                from: "me".to_string(),
                timestamp: Local::now(),
            },
            msg_lines: self.wrap_str(self.max_msg_width(), txt),
        }
    }

    fn border(&self) {
        self.window.color_set(PAIR_BORDER);
        // self.window.border('|', '|', ' ', '_', '/', '\\', '\\', '/');

        self.window.mv(
            self.window.get_max_y() - INPUT_BOX_HEIGHT - 1,
            INPUT_BOX_LEFT - 1,
        );
        self.window.hline('-', self.window.get_max_x() - 2);
    }

    /*
     * Draws a message and then returns the coordinates of the cursor at the end
     */
    fn draw_msg(&self, msg: &Message, y0: i32, x0: i32) -> (i32, i32) {
        match msg {
            Message::Text { .. } => self.draw_text_msg(msg, y0, x0),
            Message::Image { .. } => self.draw_img_msg(msg, y0, x0),
            _ => (y0, x0),
        }
    }

    fn draw_img_msg(&self, msg: &Message, y0: i32, x0: i32) -> (i32, i32) {
        // unimplemented
        (y0, x0)
    }

    fn draw_text_msg(&self, msg: &Message, y0: i32, x0: i32) -> (i32, i32) {
        if let Message::Text {
            meta_data,
            msg_lines,
        } = msg
        {
            let mut curr_x: i32;
            let mut curr_y: i32 = y0;
            // TODO distinguish between r/l justify
            let left_x = self.window.get_max_x() - 2;
            if meta_data.from == "me".to_string() {
                self.window.color_set(PAIR_OLD_SENT);
                curr_x = left_x;
            } else {
                self.window.color_set(PAIR_OLD_RECEIVED);
                curr_x = x0;
            }
            self.window.mv(curr_y, curr_x);
            for ln in msg_lines.iter().rev() {
                // go backwards for right justify
                if meta_data.from == "me".to_string() {
                    for s in ln.iter().rev() {
                        if curr_y < 0 {
                            return (curr_y, curr_x);
                        }
                        // UNSAFE CAST
                        let s_len = s.chars().count() as i32;
                        curr_x -= 1 + s_len;
                        self.window.mv(curr_y, curr_x);
                        self.window.addch(' ');
                        self.window.mv(curr_y, curr_x + 1);
                        self.window.printw(s);
                        self.window.mv(curr_y, curr_x);
                    }
                } else {
                    for s in ln {
                        if curr_y < 0 {
                            return (curr_y, curr_x);
                        }
                        // UNSAFE CAST
                        let s_len = s.chars().count() as i32;
                        self.window.printw(s);
                        self.window.addch(' ');
                        curr_x += 1 + s_len;
                        self.window.mv(curr_y, curr_x);
                    }
                }
                curr_y -= 1;
                if meta_data.from == "me".to_string() {
                    curr_x = left_x;
                } else {
                    curr_x = x0;
                }
                self.window.mv(curr_y, x0);
            }
            curr_y -= 1;
            return (curr_y, curr_x);
        }
        return (y0, x0);
    }

    /*
     * Clears a box of these dimensions, including the firstbounds
     * and excluding the last ones.
     */
    fn clr_box(&self, y0: i32, yf: i32, x0: i32, xf: i32) {
        self.window.color_set(PAIR_DEFAULT);
        let w = xf - x0;
        for y in y0..yf {
            self.window.mv(y, x0);
            self.window.hline(' ', w);
        }
    }

    fn clr_input_box(&self) {
        let ht = self.window.get_max_y();
        self.clr_box(
            ht - INPUT_BOX_HEIGHT,
            ht,
            INPUT_BOX_LEFT,
            self.window.get_max_x() - 1,
        );
    }

    fn draw_old_msgs(&self, old_msgs: &Vec<Message>) {
        let (height, width) = self.window.get_max_yx();
        // push up chat msgs accordingly
        self.clr_box(1, height - INPUT_BOX_HEIGHT, INPUT_BOX_LEFT, width);
        self.border();
        let mut curr_y = height - INPUT_BOX_HEIGHT - 2;
        // draw every line of this message, moving upwards
        // write old messages
        for msg in old_msgs.iter().rev() {
            let new_coords = self.draw_msg(msg, curr_y, INPUT_BOX_LEFT);
            curr_y = new_coords.0;
        }
    }

    pub fn run(&mut self, rx: Receiver<Vec<u8>>, tx: Sender<Vec<u8>>) {
        // create loading screen
        self.window.clear();
        // set color pairs
        set_color_pairs();
        noecho();
        let (mut height, mut width) = self.window.get_max_yx();

        // connect socket
        let mut socket = client::Socket::new(tx);

        // draw screen
        self.window.clear();
        // box for user input
        self.window.color_set(PAIR_BORDER);
        // TODO load chat history
        let mut old_msgs = self.load_old_msgs();
        self.draw_old_msgs(&old_msgs);

        let mut active_msg = String::new();
        let input_box_right: i32 = width - 1;
        let mut x = INPUT_BOX_LEFT;
        self.window.mv(height - INPUT_BOX_HEIGHT, INPUT_BOX_LEFT);

        loop {
            for msg in rx.try_iter() {
                self.pending_msgs
                    .push_back(bincode::deserialize(&msg).unwrap())
            }

            // recalc self.window dimensions
            height = self.window.get_max_y();
            width = self.window.get_max_x();

            // check for incoming messages
            while let Some(Message::Raw {
                meta_data,
                msg_type,
                text,
            }) = self.pending_msgs.pop_front()
            {
                old_msgs.push(Message::Text {
                    meta_data: meta_data,
                    msg_lines: self.wrap_str(self.max_msg_width(), &text),
                });
            }

            self.draw_old_msgs(&old_msgs);

            // poll user input
            match self.window.getch() {
                Some(Input::KeyDC) => {
                    if let Some(_) = active_msg.pop() {
                        x -= 1;
                    }
                }
                Some(Input::KeyEnter)
                | Some(Input::Character('\n'))
                | Some(Input::Character('\r')) => {
                    if active_msg.chars().count() > 0 {
                        old_msgs.push(self.send_msg(&mut socket, &active_msg));
                        self.clr_box(1, height - INPUT_BOX_HEIGHT, INPUT_BOX_LEFT, width);
                        self.border();
                        self.draw_old_msgs(&old_msgs);
                        // possible memory leak in reassignment?
                        active_msg = String::new();
                        x = INPUT_BOX_LEFT;
                    }
                }
                Some(Input::KeyBackspace)
                | Some(Input::Character('\x08'))
                | Some(Input::Character('\x7f')) => {
                    if let Some(_) = active_msg.pop() {
                        x -= 1;
                    }
                }
                Some(Input::Character(c)) => {
                    if c.is_control() {
                        match c {
                            _ => (),
                        }
                    } else {
                        active_msg.push(c);
                        x += 1;
                    }
                }
                _ => {
                    continue;
                }
            }

            // print user input under box
            let disp = self.wrap_str(input_box_right - INPUT_BOX_LEFT - 4, &active_msg);
            self.clr_input_box();
            // UNSAFE CAST
            let mut y = height - INPUT_BOX_HEIGHT;
            self.window.color_set(PAIR_TYPING);
            'drawln: for ln in disp.iter().rev() {
                x = INPUT_BOX_LEFT - 1;
                self.window.mv(y, x);
                for s in ln {
                    if y < 0 {
                        break 'drawln;
                    }
                    self.window.addch(' ');
                    self.window.printw(s);
                    x += 1;
                    // can't add usize to i32
                    // UNSAFE CAST
                    x += s.chars().count() as i32;
                    self.window.mv(y, x);
                }
                y -= 1;
            }
        } // end event loop
    }
}
