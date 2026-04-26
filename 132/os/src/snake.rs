#![allow(non_snake_case)]

use alloc::collections::VecDeque;
use core::sync::atomic::Ordering;
use lazy_static::lazy_static;
use spin::Mutex;

const Empty: u8 = b' ';
const SnakeBlock: u8 = b'o';
const FoodBlock: u8 = b'A';
const TopLeft: u8 = 0xDA;
const TopRight: u8 = 0xBF;
const BottomLeft: u8 = 0xC0;
const BottomRight: u8 = 0xD9;
const Horizontal: u8 = 0xC4;
const Vertical: u8 = 0xB3;
const MoveTicks: u64 = 4;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    Row: usize,
    Col: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeState {
    Snake: VecDeque<Position>,
    Food: Position,
    Direction: Direction,
    LastMoveTick: u64,
    Seed: usize,
    IsGameOver: bool,
}

impl SnakeState {
    fn New() -> Self {
        let mut Snake = VecDeque::new();
        Snake.push_back(Position { Row: 12, Col: 40 });

        Self {
            Snake,
            Food: Position { Row: 7, Col: 20 },
            Direction: Direction::Right,
            LastMoveTick: 0,
            Seed: 371,
            IsGameOver: false,
        }
    }
}

lazy_static! {
    static ref GAME: Mutex<SnakeState> = Mutex::new(SnakeState::New());
}

pub fn Init() {
    crate::vga::DisableCursor();
    crate::vga::ClearScreen();
    DrawBorder();

    {
        let mut Game = GAME.lock();
        *Game = SnakeState::New();

        for Position in Game.Snake.iter() {
            DrawPosition(*Position, SnakeBlock);
        }

        DrawPosition(Game.Food, FoodBlock);
    }

    crate::vga::WriteAt(0, 3, " Snake ");
    crate::vga::WriteAt(24, 3, " WASD to move, Q to quit ");
}

pub fn HandleKey(Character: char) {
    let CurrentTick = crate::TIMER_TICKS.load(Ordering::Relaxed);

    let mut Game = GAME.lock();
    Game.Seed = Game.Seed
        .wrapping_mul(1103515245)
        .wrapping_add(CurrentTick as usize)
        .wrapping_add(Character as usize);

    match Character {
        'w' | 'W' => {
            if Game.Direction != Direction::Down {
                Game.Direction = Direction::Up;
            }
        }
        'a' | 'A' => {
            if Game.Direction != Direction::Right {
                Game.Direction = Direction::Left;
            }
        }
        's' | 'S' => {
            if Game.Direction != Direction::Up {
                Game.Direction = Direction::Down;
            }
        }
        'd' | 'D' => {
            if Game.Direction != Direction::Left {
                Game.Direction = Direction::Right;
            }
        }
        'q' | 'Q' => {
            crate::QemuQuit(crate::QemuPass);
        }
        _ => {}
    }
}

pub fn Update() {
    let CurrentTick = crate::TIMER_TICKS.load(Ordering::Relaxed);

    let mut ShouldQuitFailure = false;
    let mut ShouldQuitVictory = false;

    {
        let mut Game = GAME.lock();

        if Game.IsGameOver {
            return;
        }

        if CurrentTick.saturating_sub(Game.LastMoveTick) < MoveTicks {
            return;
        }

        Game.LastMoveTick = CurrentTick;

        let Head = match Game.Snake.back() {
            Some(Head) => *Head,
            None => return,
        };

        let NewHead = NextPosition(Head, Game.Direction);

        if IsBoundary(NewHead) {
            Game.IsGameOver = true;
            ShouldQuitFailure = true;
        } else {
            let AteFood = NewHead == Game.Food;
            let mut RemovedTail = None;

            if !AteFood {
                RemovedTail = Game.Snake.pop_front();
            }

            if Game.Snake.iter().any(|SnakePosition| *SnakePosition == NewHead) {
                Game.IsGameOver = true;
                ShouldQuitFailure = true;
            } else {
                Game.Snake.push_back(NewHead);
                DrawPosition(NewHead, SnakeBlock);

                if let Some(Tail) = RemovedTail {
                    DrawPosition(Tail, Empty);
                }

                if AteFood {
                    if PlaceFood(&mut Game) {
                        DrawPosition(Game.Food, FoodBlock);
                    } else {
                        Game.IsGameOver = true;
                        ShouldQuitVictory = true;
                    }
                }
            }
        }
    }

    if ShouldQuitVictory {
        crate::vga::WriteAt(12, 34, "YOU WIN");
        crate::HltLoop();
    }

    if ShouldQuitFailure {
        crate::vga::WriteAt(12, 34, "GAME OVER");
        crate::HltLoop();
    }
}

fn DrawBorder() {
    crate::vga::WriteByteAt(0, 0, TopLeft);
    crate::vga::WriteByteAt(0, crate::vga::COLS - 1, TopRight);
    crate::vga::WriteByteAt(crate::vga::ROWS - 1, 0, BottomLeft);
    crate::vga::WriteByteAt(crate::vga::ROWS - 1, crate::vga::COLS - 1, BottomRight);

    for ColIndex in 1..crate::vga::COLS - 1 {
        crate::vga::WriteByteAt(0, ColIndex, Horizontal);
        crate::vga::WriteByteAt(crate::vga::ROWS - 1, ColIndex, Horizontal);
    }

    for RowIndex in 1..crate::vga::ROWS - 1 {
        crate::vga::WriteByteAt(RowIndex, 0, Vertical);
        crate::vga::WriteByteAt(RowIndex, crate::vga::COLS - 1, Vertical);
    }
}

fn DrawPosition(Position: Position, Byte: u8) {
    crate::vga::WriteByteAt(Position.Row, Position.Col, Byte);
}

fn NextPosition(Position: Position, Direction: Direction) -> Position {
    match Direction {
        Direction::Up => Position {
            Row: Position.Row.saturating_sub(1),
            Col: Position.Col,
        },
        Direction::Down => Position {
            Row: Position.Row + 1,
            Col: Position.Col,
        },
        Direction::Left => Position {
            Row: Position.Row,
            Col: Position.Col.saturating_sub(1),
        },
        Direction::Right => Position {
            Row: Position.Row,
            Col: Position.Col + 1,
        },
    }
}

fn IsBoundary(Position: Position) -> bool {
    Position.Row == 0
        || Position.Row >= crate::vga::ROWS - 1
        || Position.Col == 0
        || Position.Col >= crate::vga::COLS - 1
}

fn PlaceFood(Game: &mut SnakeState) -> bool {
    let mut AvailableCount = 0;

    for RowIndex in 1..crate::vga::ROWS - 1 {
        for ColIndex in 1..crate::vga::COLS - 1 {
            let Position = Position {
                Row: RowIndex,
                Col: ColIndex,
            };

            if !Game.Snake.iter().any(|SnakePosition| *SnakePosition == Position) {
                AvailableCount += 1;
            }
        }
    }

    if AvailableCount == 0 {
        return false;
    }

    Game.Seed = Game.Seed
        .wrapping_mul(1664525)
        .wrapping_add(1013904223);

    let TargetIndex = Game.Seed % AvailableCount;
    let mut CurrentIndex = 0;

    for RowIndex in 1..crate::vga::ROWS - 1 {
        for ColIndex in 1..crate::vga::COLS - 1 {
            let Position = Position {
                Row: RowIndex,
                Col: ColIndex,
            };

            if Game.Snake.iter().any(|SnakePosition| *SnakePosition == Position) {
                continue;
            }

            if CurrentIndex == TargetIndex {
                Game.Food = Position;
                return true;
            }

            CurrentIndex += 1;
        }
    }

    false
}