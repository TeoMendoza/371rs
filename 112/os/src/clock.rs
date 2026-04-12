use core::sync::atomic::Ordering;

use spin::Mutex;

pub const TicksPerSecond: u64 = 18;
const PROMPT: &str = "Enter Initial Time (HHMMSS): ";

struct ClockState {
    InputDigits: [u8; 6],
    InputLength: usize,
    IsInitialized: bool,
    Hours: u8,
    Minutes: u8,
    Seconds: u8,
    LastSecondTick: u64,
}

impl ClockState {
    const fn New() -> Self {
        Self {
            InputDigits: [0; 6],
            InputLength: 0,
            IsInitialized: false,
            Hours: 0,
            Minutes: 0,
            Seconds: 0,
            LastSecondTick: 0,
        }
    }
}

static CLOCK: Mutex<ClockState> = Mutex::new(ClockState::New());

pub fn PrintPrompt() {
    crate::vga::ClearScreen();
    crate::vga::WriteAt(0, 0, PROMPT);
    DrawInputInline();
    UpdateInputCursor();
}

pub fn IsInitialized() -> bool {
    CLOCK.lock().IsInitialized
}

pub fn HandleKey(Character: char) {
    if !Character.is_ascii_digit() {
        return;
    }

    let mut ShouldFinalize = false;

    {
        let mut Clock = CLOCK.lock();

        if Clock.IsInitialized || Clock.InputLength >= 6 {
            return;
        }

        let InputIndex = Clock.InputLength;
        Clock.InputDigits[InputIndex] = Character as u8 - b'0';
        Clock.InputLength += 1;

        if Clock.InputLength == 6 {
            ShouldFinalize = true;
        }
    }

    DrawInputInline();
    UpdateInputCursor();

    if ShouldFinalize {
        FinalizeInput();
    }
}

fn DrawInputInline() {
    let Characters = {
        let Clock = CLOCK.lock();

        let mut Characters = [b'H', b'H', b':', b'M', b'M', b':', b'S', b'S'];

        if Clock.InputLength > 0 {
            Characters[0] = b'0' + Clock.InputDigits[0];
        }
        if Clock.InputLength > 1 {
            Characters[1] = b'0' + Clock.InputDigits[1];
        }
        if Clock.InputLength > 2 {
            Characters[3] = b'0' + Clock.InputDigits[2];
        }
        if Clock.InputLength > 3 {
            Characters[4] = b'0' + Clock.InputDigits[3];
        }
        if Clock.InputLength > 4 {
            Characters[6] = b'0' + Clock.InputDigits[4];
        }
        if Clock.InputLength > 5 {
            Characters[7] = b'0' + Clock.InputDigits[5];
        }

        Characters
    };

    let DisplayText = core::str::from_utf8(&Characters).unwrap();
    let PromptCol = PROMPT.len();

    crate::vga::WriteAt(0, PromptCol, "        ");
    crate::vga::WriteAt(0, PromptCol, DisplayText);
}

fn UpdateInputCursor() {
    let InputLength = {
        let Clock = CLOCK.lock();
        Clock.InputLength
    };

    let PromptCol = PROMPT.len();

    let CursorOffset = match InputLength {
        0 => 0,
        1 => 1,
        2 => 3,
        3 => 4,
        4 => 6,
        5 => 7,
        _ => 7,
    };

    crate::vga::SetCursorPosition(0, PromptCol + CursorOffset);
}

fn FinalizeInput() {
    let CurrentTick = crate::TIMER_TICKS.load(Ordering::Relaxed);

    let mut ParsedTime = None;

    {
        let mut Clock = CLOCK.lock();

        let Hours = Clock.InputDigits[0] * 10 + Clock.InputDigits[1];
        let Minutes = Clock.InputDigits[2] * 10 + Clock.InputDigits[3];
        let Seconds = Clock.InputDigits[4] * 10 + Clock.InputDigits[5];

        if Hours < 24 && Minutes < 60 && Seconds < 60 {
            Clock.Hours = Hours;
            Clock.Minutes = Minutes;
            Clock.Seconds = Seconds;
            Clock.IsInitialized = true;
            Clock.LastSecondTick = CurrentTick;
            ParsedTime = Some((Clock.Hours, Clock.Minutes, Clock.Seconds));
        } else {
            Clock.InputDigits = [0; 6];
            Clock.InputLength = 0;
        }
    }

    match ParsedTime {
        Some((Hours, Minutes, Seconds)) => {
            crate::vga::ClearScreen();
            DrawTime(Hours, Minutes, Seconds);
            crate::vga::SetWriterPosition(1, 0);
        }
        None => {
            crate::vga::ClearScreen();
            crate::vga::WriteAt(0, 0, "Invalid Time. Use HHMMSS: ");
            DrawInvalidInputInline();
            UpdateInvalidCursor();
        }
    }
}

fn DrawInvalidInputInline() {
    let Prompt = "Invalid Time. Use HHMMSS: ";
    crate::vga::WriteAt(0, Prompt.len(), "HH:MM:SS");
}

fn UpdateInvalidCursor() {
    let InputLength = {
        let Clock = CLOCK.lock();
        Clock.InputLength
    };

    let PromptCol = "Invalid Time. Use HHMMSS: ".len();

    let CursorOffset = match InputLength {
        0 => 0,
        1 => 1,
        2 => 3,
        3 => 4,
        4 => 6,
        5 => 7,
        _ => 7,
    };

    crate::vga::SetCursorPosition(0, PromptCol + CursorOffset);
}

pub fn Update() {
    let CurrentTick = crate::TIMER_TICKS.load(Ordering::Relaxed);
    let mut TimeToDraw = None;

    {
        let mut Clock = CLOCK.lock();

        if !Clock.IsInitialized {
            return;
        }

        while CurrentTick.saturating_sub(Clock.LastSecondTick) >= TicksPerSecond {
            Clock.LastSecondTick += TicksPerSecond;
            AdvanceOneSecond(&mut Clock);
            TimeToDraw = Some((Clock.Hours, Clock.Minutes, Clock.Seconds));
        }
    }

    if let Some((Hours, Minutes, Seconds)) = TimeToDraw {
        DrawTime(Hours, Minutes, Seconds);
    }
}

fn AdvanceOneSecond(Clock: &mut ClockState) {
    Clock.Seconds += 1;

    if Clock.Seconds >= 60 {
        Clock.Seconds = 0;
        Clock.Minutes += 1;
    }

    if Clock.Minutes >= 60 {
        Clock.Minutes = 0;
        Clock.Hours += 1;
    }

    if Clock.Hours >= 24 {
        Clock.Hours = 0;
    }
}

fn DrawTime(Hours: u8, Minutes: u8, Seconds: u8) {
    let TimeBytes = FormatTime(Hours, Minutes, Seconds);
    let TimeText = core::str::from_utf8(&TimeBytes).unwrap();

    crate::vga::WriteAt(0, 0, "        ");
    crate::vga::WriteAt(0, 0, TimeText);
}

fn FormatTime(Hours: u8, Minutes: u8, Seconds: u8) -> [u8; 8] {
    [
        b'0' + (Hours / 10),
        b'0' + (Hours % 10),
        b':',
        b'0' + (Minutes / 10),
        b'0' + (Minutes % 10),
        b':',
        b'0' + (Seconds / 10),
        b'0' + (Seconds % 10),
    ]
}