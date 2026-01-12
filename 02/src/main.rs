use std::fs;
use std::io::{self, Read};

#[derive(Clone, Copy, Default)]
struct SelectedCounts {
    PrintLines: bool,
    PrintWords: bool,
    PrintChars: bool,
    PrintBytes: bool,
    PrintMaxLineLength: bool,
}

#[derive(Clone, Copy, Default)]
struct WcCounts {
    Lines: u64,
    Words: u64,
    Chars: u64,
    Bytes: u64,
    MaxLineLength: u64,
}

struct ParsedArguments {
    Selected: SelectedCounts,
    Files0From: Option<String>,
    InputNames: Vec<String>,
    ShowHelp: bool,
    ShowVersion: bool,
}

fn main() {
    let Arguments: Vec<String> = std::env::args().skip(1).collect();
    let Parsed: ParsedArguments = ParseArguments(Arguments);

    if Parsed.ShowHelp {
        PrintHelp();
        return;
    }

    if Parsed.ShowVersion {
        PrintVersion();
        return;
    }

    let mut Selected: SelectedCounts = Parsed.Selected;
    if !Selected.PrintLines && !Selected.PrintWords && !Selected.PrintChars && !Selected.PrintBytes && !Selected.PrintMaxLineLength {
        Selected.PrintLines = true;
        Selected.PrintWords = true;
        Selected.PrintBytes = true;
    }

    let InputNames: Vec<String> = match Parsed.Files0From {
        Some(Source) => ReadNamesFromFiles0From(&Source).unwrap_or_else(|Message| ExitWithError(Message)),
        None => Parsed.InputNames,
    };

    RunWc(Selected, InputNames);
}

fn RunWc(Selected: SelectedCounts, InputNames: Vec<String>) {
    let mut Results: Vec<(Option<String>, WcCounts)> = Vec::new();
    let mut StdinCache: Option<Vec<u8>> = None;

    if InputNames.is_empty() {
        let InputBytes: Vec<u8> = ReadAllStdinBytes().unwrap_or_else(|Message| ExitWithError(Message));
        Results.push((None, ComputeCounts(&InputBytes)));
        PrintResults(Selected, &Results);
        return;
    }

    for Name in InputNames {
        if Name == "-" {
            let InputBytes: Vec<u8> = GetOrReadStdin(&mut StdinCache).unwrap_or_else(|Message| ExitWithError(Message));
            Results.push((Some("-".to_string()), ComputeCounts(&InputBytes)));
            continue;
        }

        let InputBytes: Vec<u8> = fs::read(&Name)
            .map_err(|Error| format!("wc: {}: {}", Name, Error))
            .unwrap_or_else(|Message| ExitWithError(Message));

        Results.push((Some(Name), ComputeCounts(&InputBytes)));
    }

    PrintResults(Selected, &Results);
}

fn GetOrReadStdin(StdinCache: &mut Option<Vec<u8>>) -> Result<Vec<u8>, String> {
    if let Some(Cached) = StdinCache.as_ref() {
        return Ok(Cached.clone());
    }

    let Bytes: Vec<u8> = ReadAllStdinBytes()?;
    *StdinCache = Some(Bytes.clone());
    Ok(Bytes)
}

fn ParseArguments(Arguments: Vec<String>) -> ParsedArguments {
    let mut Selected: SelectedCounts = SelectedCounts::default();
    let mut Files0From: Option<String> = None;
    let mut InputNames: Vec<String> = Vec::new();
    let mut ShowHelp: bool = false;
    let mut ShowVersion: bool = false;

    let mut Index: usize = 0;
    while Index < Arguments.len() {
        let Arg: &str = &Arguments[Index];

        if Arg == "--help" {
            ShowHelp = true;
            Index += 1;
            continue;
        }

        if Arg == "--version" {
            ShowVersion = true;
            Index += 1;
            continue;
        }

        if let Some(Value) = Arg.strip_prefix("--files0-from=") {
            Files0From = Some(Value.to_string());
            Index += 1;
            continue;
        }

        if Arg == "--files0-from" {
            if Index + 1 >= Arguments.len() {
                ExitWithError("wc: option '--files0-from' requires an argument".to_string());
            }
            Files0From = Some(Arguments[Index + 1].clone());
            Index += 2;
            continue;
        }

        if Arg.starts_with("--") {
            ApplyLongFlag(&mut Selected, Arg);
            Index += 1;
            continue;
        }

        if Arg.starts_with('-') && Arg != "-" {
            for FlagChar in Arg[1..].chars() {
                ApplyShortFlag(&mut Selected, FlagChar, &mut ShowHelp, &mut ShowVersion);
            }
            Index += 1;
            continue;
        }

        InputNames.push(Arg.to_string());
        Index += 1;
    }

    if Files0From.is_some() && !InputNames.is_empty() {
        ExitWithError("wc: cannot combine --files0-from with FILE arguments".to_string());
    }

    ParsedArguments { Selected, Files0From, InputNames, ShowHelp, ShowVersion }
}

fn ApplyLongFlag(Selected: &mut SelectedCounts, Arg: &str) {
    match Arg {
        "--bytes" => Selected.PrintBytes = true,
        "--chars" => Selected.PrintChars = true,
        "--lines" => Selected.PrintLines = true,
        "--words" => Selected.PrintWords = true,
        "--max-line-length" => Selected.PrintMaxLineLength = true,
        _ => ExitWithError(format!("wc: unrecognized option '{}'", Arg)),
    }
}

fn ApplyShortFlag(Selected: &mut SelectedCounts, FlagChar: char, ShowHelp: &mut bool, ShowVersion: &mut bool) {
    match FlagChar {
        'c' => Selected.PrintBytes = true,
        'm' => Selected.PrintChars = true,
        'l' => Selected.PrintLines = true,
        'w' => Selected.PrintWords = true,
        'L' => Selected.PrintMaxLineLength = true,
        'h' => *ShowHelp = true,
        'V' => *ShowVersion = true,
        _ => ExitWithError(format!("wc: invalid option -- '{}'", FlagChar)),
    }
}

fn ReadNamesFromFiles0From(Source: &str) -> Result<Vec<String>, String> {
    let Bytes: Vec<u8> = if Source == "-" {
        ReadAllStdinBytes()?
    } else {
        fs::read(Source).map_err(|Error| format!("wc: {}: {}", Source, Error))?
    };

    if Bytes.is_empty() {
        return Ok(Vec::new());
    }

    let mut Names: Vec<String> = Vec::new();
    let Parts: Vec<&[u8]> = Bytes.split(|Byte| *Byte == 0u8).collect();

    if Parts.len() > 1 {
        for Part in Parts {
            if !Part.is_empty() {
                let Name: String = String::from_utf8_lossy(Part).trim().to_string();
                if !Name.is_empty() {
                    Names.push(Name);
                }
            }
        }
        return Ok(Names);
    }

    let Single: String = String::from_utf8_lossy(&Bytes).trim().to_string();
    if !Single.is_empty() {
        Names.push(Single);
    }

    Ok(Names)
}

fn ReadAllStdinBytes() -> Result<Vec<u8>, String> {
    let mut Buffer: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut Buffer).map_err(|Error| format!("wc: stdin: {}", Error))?;
    Ok(Buffer)
}

fn ComputeCounts(InputBytes: &[u8]) -> WcCounts {
    let Bytes: u64 = InputBytes.len() as u64;
    let Lines: u64 = InputBytes.iter().filter(|Byte| **Byte == b'\n').count() as u64;

    let Text = String::from_utf8_lossy(InputBytes);
    let Words: u64 = Text.split_whitespace().count() as u64;
    let Chars: u64 = Text.chars().count() as u64;

    let mut MaxLineLength: u64 = 0;
    for Line in Text.split('\n') {
        let LineLength: u64 = Line.chars().count() as u64;
        if LineLength > MaxLineLength {
            MaxLineLength = LineLength;
        }
    }

    WcCounts { Lines, Words, Chars, Bytes, MaxLineLength }
}

fn PrintResults(Selected: SelectedCounts, Results: &[(Option<String>, WcCounts)]) {
    let HasNames: bool = Results.iter().any(|(Name, _)| Name.is_some());
    let NeedsTotal: bool = Results.len() > 1;
    let Total: Option<WcCounts> = if NeedsTotal { Some(ComputeTotal(Results)) } else { None };

    let Widths: (usize, usize, usize, usize, usize) = ComputeWidths(Selected, Results, Total.as_ref());

    for (Name, Counts) in Results {
        PrintOneLine(Selected, *Counts, Name.as_deref(), HasNames, Widths);
    }

    if let Some(TotalCounts) = Total {
        PrintOneLine(Selected, TotalCounts, Some("total"), true, Widths);
    }
}

fn ComputeTotal(Results: &[(Option<String>, WcCounts)]) -> WcCounts {
    let mut Total: WcCounts = WcCounts::default();

    for (_, Counts) in Results {
        Total.Lines += Counts.Lines;
        Total.Words += Counts.Words;
        Total.Chars += Counts.Chars;
        Total.Bytes += Counts.Bytes;
        Total.MaxLineLength = Total.MaxLineLength.max(Counts.MaxLineLength);
    }

    Total
}

fn ComputeWidths(Selected: SelectedCounts, Results: &[(Option<String>, WcCounts)], Total: Option<&WcCounts>) -> (usize, usize, usize, usize, usize) {
    let mut MaxLines: u64 = 0;
    let mut MaxWords: u64 = 0;
    let mut MaxChars: u64 = 0;
    let mut MaxBytes: u64 = 0;
    let mut MaxMaxLine: u64 = 0;

    for (_, Counts) in Results {
        UpdateMaxes(&mut MaxLines, &mut MaxWords, &mut MaxChars, &mut MaxBytes, &mut MaxMaxLine, *Counts);
    }

    if let Some(TotalCounts) = Total {
        UpdateMaxes(&mut MaxLines, &mut MaxWords, &mut MaxChars, &mut MaxBytes, &mut MaxMaxLine, *TotalCounts);
    }

    let LinesWidth: usize = if Selected.PrintLines { DigitCount(MaxLines) + 1 } else { 0 };
    let WordsWidth: usize = if Selected.PrintWords { DigitCount(MaxWords) + 1 } else { 0 };
    let CharsWidth: usize = if Selected.PrintChars { DigitCount(MaxChars) + 1 } else { 0 };
    let BytesWidth: usize = if Selected.PrintBytes { DigitCount(MaxBytes) + 1 } else { 0 };
    let MaxLineWidth: usize = if Selected.PrintMaxLineLength { DigitCount(MaxMaxLine) + 1 } else { 0 };

    (LinesWidth, WordsWidth, CharsWidth, BytesWidth, MaxLineWidth)
}

fn UpdateMaxes(MaxLines: &mut u64, MaxWords: &mut u64, MaxChars: &mut u64, MaxBytes: &mut u64, MaxMaxLine: &mut u64, Counts: WcCounts) {
    *MaxLines = (*MaxLines).max(Counts.Lines);
    *MaxWords = (*MaxWords).max(Counts.Words);
    *MaxChars = (*MaxChars).max(Counts.Chars);
    *MaxBytes = (*MaxBytes).max(Counts.Bytes);
    *MaxMaxLine = (*MaxMaxLine).max(Counts.MaxLineLength);
}

fn DigitCount(Value: u64) -> usize {
    if Value == 0 {
        return 1;
    }

    let mut Count: usize = 0;
    let mut Current: u64 = Value;
    while Current > 0 {
        Current /= 10;
        Count += 1;
    }

    Count
}

fn PrintOneLine(Selected: SelectedCounts, Counts: WcCounts, Name: Option<&str>, HasNames: bool, Widths: (usize, usize, usize, usize, usize)) {
    let (LinesWidth, WordsWidth, CharsWidth, BytesWidth, MaxLineWidth) = Widths;
    let mut Output: String = String::new();

    if Selected.PrintLines { Output.push_str(&format!("{:>Width$}", Counts.Lines, Width = LinesWidth)); }
    if Selected.PrintWords { Output.push_str(&format!("{:>Width$}", Counts.Words, Width = WordsWidth)); }
    if Selected.PrintChars { Output.push_str(&format!("{:>Width$}", Counts.Chars, Width = CharsWidth)); }
    if Selected.PrintBytes { Output.push_str(&format!("{:>Width$}", Counts.Bytes, Width = BytesWidth)); }
    if Selected.PrintMaxLineLength { Output.push_str(&format!("{:>Width$}", Counts.MaxLineLength, Width = MaxLineWidth)); }

    if HasNames {
        if let Some(Value) = Name {
            Output.push(' ');
            Output.push_str(Value);
        }
    }

    println!("{}", Output);
}

fn PrintVersion() {
    println!("wc (my_wc) 0.1.0");
}

fn PrintHelp() {
    println!("Usage: wc [OPTION]... [FILE]...");
    println!("  or:  wc [OPTION]... --files0-from=F");
    println!("Print newline, word, and byte counts for each FILE, and a total line if");
    println!("more than one FILE is specified.  A word is a non-zero-length sequence of");
    println!("characters delimited by white space.");
    println!();
    println!("With no FILE, or when FILE is -, read standard input.");
    println!();
    println!("The options below may be used to select which counts are printed, always in");
    println!("the following order: newline, word, character, byte, maximum line length.");
    println!("  -c, --bytes            print the byte counts");
    println!("  -m, --chars            print the character counts");
    println!("  -l, --lines            print the newline counts");
    println!("      --files0-from=F    read input from the files specified by NUL-terminated names in file F;");
    println!("                           If F is - then read names from standard input");
    println!("  -L, --max-line-length  print the maximum display width");
    println!("  -w, --words            print the word counts");
    println!("      --help     display this help and exit");
    println!("      --version  output version information and exit");
}

fn ExitWithError(Message: String) -> ! {
    eprintln!("{}", Message);
    std::process::exit(1);
}
