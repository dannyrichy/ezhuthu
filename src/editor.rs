use crate::Document;
use crate::Terminal;
use crate::Row;
use std::env;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position{
    pub x:usize,
    pub y:usize,
}
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
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

        let args: Vec<String> = env::args().collect();
        let document = if args.len() >1{
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };


        Self{
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
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
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position{x,y} = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y< offset.y{
            offset.y = y;
        } else if y>= offset.y.saturating_add(height){
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x< offset.x{
            offset.x = x;
        } else if x >=offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }


    }

    fn move_cursor(&mut self, key:Key){
        let terminal_h = self.terminal.size().height as usize;
        let Position{mut x, mut y} = self.cursor_position;

        let h = self.document.len();
        let mut w = if let Some(row) = self.document.row(y){
            row.len()
        } else {
            0
        };

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
            Key::PageUp => {
                y = if y > terminal_h{ y-terminal_h} else {0}
            },
            Key::PageDown => {                
                y = if y.saturating_add(terminal_h)<h{
                    y + terminal_h as usize
                } else {h}
            }, 
            _ => (),
        }
        w = if let Some(row) = self.document.row(y){
            row.len()
        } else {0};
        if x > w{
            x=w;
        }
        self.cursor_position = Position {x,y}
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{
        
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye Moon man!\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&Position { 
                x: self.cursor_position.x.saturating_sub(self.offset.x), 
                y: self.cursor_position.y.saturating_sub(self.offset.y) 
            });
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

    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = width + self.offset.y;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self){

        let height = self.terminal.size().height;
        
        for term_row  in 0..height -1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(term_row as usize + self.offset.y){
                self.draw_row(row);
            } else if self.document.is_empty() && term_row == height /3 {
                self.draw_msg();
            } else {
                println!("~\r");
            }
            

        }
    }

    }    

fn die(e:&std::io::Error){
    Terminal::clear_screen();
    panic!("{}", e);
}


