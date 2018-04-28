extern crate pancurses;

use pancurses::{Window, Input};

const INPUT_BOX_HEIGHT: i32 = 6;

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

type HeightedMsg = (String, i32);

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

fn load_old_msgs(window: &Window) -> Vec<HeightedMsg> {
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
    return old_msgs.iter().map(|s| get_msg_str_nofmt(&window, s)).collect();
}

fn max_msg_width(window: &Window) -> i32 {
    window.get_max_x() / 3
}

/*
 * Returns the text stored in a message object, as well as the message's height.
 */
fn get_msg_str_nofmt(window: &Window, msg: &Message) -> HeightedMsg {
    get_msg_str(&window, msg, "".to_string(), "".to_string())
}

fn wrap_str(wrap_width: i32, txt: &String, fg: String, bg: String) -> HeightedMsg {
    let mut line_len = 0;
    let split = txt.split(' ');
    let mut ret = fg;
    let mut line_ct = 1;
    // ignore other delimiters for now
    for s in split {
        let s_len = s.chars().count();
        // if it's too long, put it on its own line
        if s_len >= wrap_width as usize {
        	ret.push('\n');
            ret.push_str(s);
            line_len = 0;
        } else if line_len + s_len >= wrap_width as usize {
            // time for a new line
            ret.push('\n');
            ret.push_str(s);
            line_len = s_len;
            line_ct += 1;
        } else {
            // continue adding on this line
            ret.push_str(s);
            ret.push(' ');
            line_len += s_len;
        }
    }
    ret.push_str(&bg);
    ret.push('\n');
    return (ret, line_ct);
}

fn get_msg_str(window: &Window, msg: &Message, fg: String, bg: String) -> HeightedMsg {
    let wrap_width = max_msg_width(&window);
    if let Some(ref txt) = msg.content.text {
        return wrap_str(wrap_width, &txt, fg, bg);
    } else {
        return ("".to_string(), 0);
    }
}

fn send_msg(window: &Window, txt: &String) -> HeightedMsg {
    get_msg_str_nofmt(&window, &Message {
        m_type: "text".to_string(),
        to: "me".to_string(),
        from: "you".to_string(),
        timestamp: "000".to_string(),
        content: Content {
            text: Some(txt.to_string()),
            url: None
        }
    })
}

fn border(window: &Window) {
    window.border('|', '|', '_', '_', '/', '\\', '\\', '/');
}

pub fn init(window: &Window) {
    window.clear();
    let height = window.get_max_y();
    let width = window.get_max_x();
    // box for user input
    window.mv(height - INPUT_BOX_HEIGHT - 1, 0);
    window.hline('-', width);
    // TODO load chat history
    let mut old_msgs = load_old_msgs(&window);
    // push up chat msgs accordingly
    const INPUT_BOX_LEFT: i32 = 2;
    let update_old_msgs_view = |msgs: &Vec<HeightedMsg>| {
	    border(&window);
        let mut curr_y = height - INPUT_BOX_HEIGHT;
        // clear old stuff
        for y in 1..curr_y {
        	window.mv(y, INPUT_BOX_LEFT);
            window.hline(' ', width - 2);
        }
        for &(ref s, ht) in msgs.iter().rev() {
            curr_y = curr_y - ht - 1;
	        window.mv(curr_y, INPUT_BOX_LEFT);
            if curr_y < 0 {
                break;
            }
            window.addstr(&s);
        }
    };
    update_old_msgs_view(&old_msgs);
    let mut active_msg = String::new();
    let input_box_right: i32 = width - 1;
    let mut x = INPUT_BOX_LEFT;
    loop {
        // check for incoming messages
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
            Some(_) => (),
            None => ()
        }
        // print user input under box
        // (rerenders whole thing on every run)
        let (disp, y) = wrap_str(input_box_right - INPUT_BOX_LEFT, &active_msg, "".to_string(), "".to_string());
        window.mv(height - INPUT_BOX_HEIGHT, INPUT_BOX_LEFT);
        window.addstr(&disp);
        window.mv(height - INPUT_BOX_HEIGHT + y - 1, x);
    }
}

