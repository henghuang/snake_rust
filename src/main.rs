
extern crate rand;
extern crate termion;
use rand::prelude::*;
use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;

use termion::raw::IntoRawMode;
use termion::{async_stdin, color, cursor, terminal_size};

#[derive(Debug)]
struct Element {
    x: i16,
    y: i16,
}
#[derive(Debug)]
struct Snake {
    body: Vec<Element>,
    direction: (i16, i16), //x,y
    food: (u16, u16),      //x,y
    score: u64,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: vec![Element { x: 10, y: 10 }],
            direction: (1, 0),
            food: (15, 15),
            score: 0,
        }
    }
    fn draw(&mut self, key: &Option<Result<u8, std::io::Error>>, stdout: &mut Write) -> bool {
        //draw snake
        for each in self.body.iter() {
            write!(
                stdout,
                "{}{}{}",
                cursor::Goto(each.x as u16, each.y as u16),
                color::Fg(color::Red),
                "*"
            )
            .unwrap();
        }
        //draw food
        write!(
            stdout,
            "{}{}{}",
            cursor::Goto(self.food.0 as u16, self.food.1 as u16),
            color::Fg(color::Rgb(255, 156, 0)),
            "*"
        )
        .unwrap();
        //draw title
        write!(
            stdout,
            "{}{}Score: {}, press 'q' to quit ",
            cursor::Goto(1, 1),
            color::Fg(color::Green),
            self.score,
        )
        .unwrap();

        return Self::update(self, key);
    }

    fn update(&mut self, key: &Option<Result<u8, std::io::Error>>) -> bool {
        match key {
            Some(Ok(b'a')) => self.direction = (-1, 0),
            Some(Ok(b'd')) => self.direction = (1, 0),
            Some(Ok(b'w')) => self.direction = (0, -1),
            Some(Ok(b's')) => self.direction = (0, 1),
            Some(Ok(_)) => {}
            Some(Err(_)) => {}
            None => {}
        }
        //eat food?
        let head = self.body.first().unwrap();
        let mut rng = rand::thread_rng();
        let (screen_x, screen_y) = terminal_size().unwrap();
        if head.x == (self.food.0 as i16) && head.y == (self.food.1 as i16) {
            self.food.0 = rng.gen_range(0, screen_x);
            self.food.1 = rng.gen_range(0, screen_y);
            self.body.push(Element { x: 1, y: 1 }); //increase the body if eat
            self.score += 1;
        }

        //check game end
        if self.body[0].x < 0
            || self.body[0].y < 0
            || self.body[0].x > screen_x as i16
            || self.body[0].y > screen_y as i16
        {
            return false;
        }

        //update body
        for i in (1..self.body.len()).rev() {
            self.body[i].x = self.body[i - 1].x;
            self.body[i].y = self.body[i - 1].y;
        }
        self.body[0].x += self.direction.0;
        self.body[0].y += self.direction.1;

        return true;
    }
}

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut snake = Snake::new();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();


    loop {
        write!(stdout, "{}", termion::clear::All).unwrap();
        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }
        let game_ok = snake.draw(&b, &mut stdout);
        if !game_ok {
            break;
        }
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(200));
    }
    write!(
        stdout,
        "{}{}Score: {}, game over ",
        cursor::Goto(1, 1),
        color::Fg(color::Green),
        snake.score
    )
    .unwrap();
}
