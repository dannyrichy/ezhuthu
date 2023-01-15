use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position{
    pub x:usize,
    pub y:usize,
}
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor{
    pub fn run(&mut self){

        loop {
                if let Err(error) = self.refresh_screen() {
                    die(&error);
                }
                if self.should_quit {
                    break;
                }
                if let Err(error) = self.process_key_press() {
                    die(&error);
                }
        }
    }

    pub fn default() -> Self{
        Self{
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    fn process_key_press(&mut self)-> Result<(), std::io::Error>{
        let key = Terminal::read_key()?;
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up 
            | Key::Down 
            | Key::Left 
            | Key::Right 
            | Key::PageUp 
            | Key::PageDown 
            | Key::Home 
            | Key::End => self.move_cursor(key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key:Key){
        let Position{mut x, mut y} = self.cursor_position;

        let size = self.terminal.size();
        let h = size.height.saturating_sub(1) as usize;
        let w = size.width.saturating_sub(1) as usize;

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < h {
                    y = y.saturating_add(1);
                }
            },
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                    if x < w{
                        x = x.saturating_add(1)
                    }
                },
            Key::Home => x = 0,
            Key::End => x = w,
            Key::PageUp => y = 0,
            Key::PageDown => y = h, 
            _ => (),
        }

        self.cursor_position = Position {x,y}
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{
        
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position{x:0,y:0});
        
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye Moon man!\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        } 
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_msg(&self){
        let mut msg:String = format!("Ezhuthu editor -- version {}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = msg.len();

        let padding = width.saturating_sub(len)/2;
        let space = " ".repeat(padding.saturating_sub(1));
        
        msg = format!("~{}{}", space, msg);
        msg.truncate(width);
        println!("{}\r", msg);
    }

    fn draw_rows(&self){

        let height = self.terminal.size().height;
        
        for row  in 0..height -1 {
            Terminal::clear_current_line();
            if row == height/3{
                self.draw_msg();
            }
            println!("~\r");

        }
    }

    }    

fn die(e:&std::io::Error){
    Terminal::clear_screen();
    panic!("{}", e);
}


