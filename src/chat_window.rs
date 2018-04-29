extern crate pancurses;

use pancurses::{Window, Input, init_pair};


struct Content {
    text: Option<String>,
    url: Option<String>
}

struct Message {
    m_type: String,
    to: String,
    from: String,
    timestamp: String,
    content: Content
}

/*
 * Processes incoming messages and moves up old messages accordingly.
 */
fn process_incoming_msg(window: &Window) -> Option<String> {
    Some("test".to_string())
}

type MsgLines = Vec<Vec<String>>;
struct DispMsg {
    lines: MsgLines,
    usr_snt: bool
}

fn gen_test_msg(s: &str) -> Message {
	Message {
		m_type: "text".to_string(),
        to: "me".to_string(),
        from: "you".to_string(),
        timestamp: "000".to_string(),
        content: Content {
            text: Some(s.to_string()),
            url: None
        }
	}
}

fn load_old_msgs(window: &Window) -> Vec<DispMsg> {
    let mut old_msgs: Vec<Message> = Vec::new();
    old_msgs.push(Message {
        m_type: "text".to_string(),
        to: "me".to_string(),
        from: "you".to_string(),
        timestamp: "000".to_string(),
        content: Content {
            text: Some("this is an old message".to_string()),
            url: None
        }
    });
    old_msgs.push(Message {
        m_type: "text".to_string(),
        to: "me".to_string(),
        from: "you".to_string(),
        timestamp: "000".to_string(),
        content: Content {
            text: Some("this is a really really long old message that will hopefully cause the thing to wrap a couple times"
            .to_string()),
            url: None
        }
    });
    old_msgs.push(gen_test_msg("ha"));
    old_msgs.push(gen_test_msg("hello"));
    old_msgs.push(gen_test_msg("pls stop ignoring my texts"));
    return old_msgs.iter().map(|s| DispMsg {
        lines: get_msg_str(&window, s),
        usr_snt: false
    }).collect();
}

fn max_msg_width(window: &Window) -> i32 {
    window.get_max_x() / 3
}

/*
 * Returns the text stored in a message object. The resulting vec
 * contains a Vec<String> to represent each line, and the Vec<String>
 * is split into words.
 */
fn wrap_str(wrap_width: i32, txt: &String) -> MsgLines {
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

fn get_msg_str(window: &Window, msg: &Message) -> MsgLines {
    let wrap_width = max_msg_width(&window);
    if let Some(ref txt) = msg.content.text {
        return wrap_str(wrap_width, &txt);
    } else {
        return Vec::new();
    }
}

fn send_msg(window: &Window, txt: &String) -> DispMsg {
    DispMsg {
        lines: get_msg_str(&window, &Message {
            m_type: "text".to_string(),
            to: "me".to_string(),
            from: "you".to_string(),
            timestamp: "000".to_string(),
            content: Content {
                text: Some(txt.to_string()),
                url: None
            }
        }),
        usr_snt: true
    }
}

type color = i16;
const FG_DEFAULT: color = pancurses::COLOR_WHITE;
const BG_RECEIVE_DEFAULT: color = pancurses::COLOR_MAGENTA;
const BG_SEND_DEFAULT: color = pancurses::COLOR_CYAN;
const BG_SCREEN_DEFAULT: color = pancurses::COLOR_BLACK;

const PAIR_OLD_RECEIVED: color = 1;
const PAIR_DEFAULT: color = 2;
const PAIR_OLD_SENT: color = 3;
const PAIR_TYPING: color = 4;
const PAIR_BORDER: color = 5;

fn set_color_pairs() {
    init_pair(PAIR_OLD_RECEIVED, FG_DEFAULT, BG_RECEIVE_DEFAULT);
    init_pair(PAIR_DEFAULT, pancurses::COLOR_GREEN, BG_SCREEN_DEFAULT);
    init_pair(PAIR_OLD_SENT, pancurses::COLOR_BLUE, BG_SEND_DEFAULT);
    init_pair(PAIR_TYPING, FG_DEFAULT, BG_SCREEN_DEFAULT);
    init_pair(PAIR_BORDER, pancurses::COLOR_RED, BG_SCREEN_DEFAULT);
}

const INPUT_BOX_HEIGHT: i32 = 6;
const INPUT_BOX_LEFT: i32 = 2;

fn border(window: &Window) {
    window.color_set(PAIR_BORDER);
    window.border('|', '|', ' ', '_', '/', '\\', '\\', '/');

    window.mv(window.get_max_y() - INPUT_BOX_HEIGHT - 1, 0);
    window.hline('-', window.get_max_x());
}

/*
 * Draws a message and then returns the coordinates of the cursor at the end
 */
fn draw_msg(window: &Window, msg: &DispMsg, mut curr_y: i32, mut curr_x: i32) -> (i32, i32) {
    let lines = &msg.lines;
    let usr_snt = msg.usr_snt;
    let x0 = curr_x;
    // TODO distinguish between r/l justify
    if usr_snt {
        window.color_set(PAIR_OLD_SENT);
    } else {
        window.color_set(PAIR_OLD_RECEIVED);
    }
    window.mv(curr_y, x0);
    for ln in lines.iter().rev() {
        for s in ln {
            if curr_y < 0 {
                return (curr_y, curr_x);
            }
            window.printw(s);
            window.addch(' ');
            curr_x += 1;
            // can't add usize to i32
            // UNSAFE CAST
            curr_x += s.chars().count() as i32;
            window.mv(curr_y, curr_x);
        }
        curr_y -= 1;
        curr_x = x0;
        window.mv(curr_y, x0);
    }
    curr_y -= 1;
    return (curr_y, curr_x)
}

fn clr_in_box(window: &Window) {
    window.color_set(PAIR_DEFAULT);
    window.mv(window.get_max_y() - INPUT_BOX_HEIGHT + 1, INPUT_BOX_LEFT);
    window.hline(' ', window.get_max_x() - 3);
}

pub fn init(window: &Window) {
    window.clear();
    let height = window.get_max_y();
    let width = window.get_max_x();
    // set color pairs
    set_color_pairs();
    // box for user input
    window.color_set(PAIR_BORDER);
    // TODO load chat history
    let mut old_msgs = load_old_msgs(&window);
    // push up chat msgs accordingly
    let update_old_msgs_view = |msgs: &Vec<DispMsg>| {
	    border(&window);
        let mut curr_y = height - INPUT_BOX_HEIGHT - 2;
        // clear old stuff
        window.color_set(PAIR_DEFAULT);
        for y in 1..curr_y {
        	window.mv(y, INPUT_BOX_LEFT);
            window.hline(' ', width - 3);
        }
        // draw every line of this message, moving upwards
        let mut curr_x = INPUT_BOX_LEFT;
        // write old messages
        for msg in msgs.iter().rev() {
            let new_coords = draw_msg(window, msg, curr_y, curr_x);
            curr_y = new_coords.0;
            curr_x = new_coords.1;
        }
    };
    update_old_msgs_view(&old_msgs);
    let mut active_msg = String::new();
    let input_box_right: i32 = width - 1;
    let mut x = INPUT_BOX_LEFT;
    loop {
        // small time delay

        // TODO check for incoming messages
        // poll user input
        match window.getch() {
            Some(Input::KeyEnter) => {
                old_msgs.push(send_msg(&window, &active_msg));
                update_old_msgs_view(&old_msgs);
                // possible memory leak in reassignment?
                active_msg = String::new();
                x = INPUT_BOX_LEFT;
            },
            Some(Input::KeyBackspace) => {
            	if let Some(_) = active_msg.pop() {
            		x -= 1;
            	}
            },
            Some(Input::Character(c)) => {
                active_msg.push(c);
                x += 1;
            },
            Some(_) => { continue; },
            None => { continue; }
        }
        // print user input under box
        // (rerenders whole thing on every run)
        let disp = wrap_str(5/*input_box_right - INPUT_BOX_LEFT - 4*/, &active_msg);
        clr_in_box(window);
        // UNSAFE CAST
        let mut y = height - INPUT_BOX_HEIGHT;
        window.color_set(PAIR_TYPING);
        'drawln: for ln in disp.iter().rev() {
            x = INPUT_BOX_LEFT - 1;
            window.mv(y, x);
            for s in ln {
                if y < 0 {
                    break 'drawln
                }
                window.addch(' ');
                window.printw(s);
                x += 1;
                // can't add usize to i32
                // UNSAFE CAST
                x += s.chars().count() as i32;
                window.mv(y, x);
            }
            y -= 1;
        }
    }
}

