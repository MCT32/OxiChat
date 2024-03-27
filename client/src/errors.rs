use std::process::exit;

pub fn terminal_error() {
    eprintln!("
    ERROR: TERMINAL ERROR\nThe fiery serpent strikes at dawn,\n
    its venom consumes the prideful heart; for in arrogance,\n
    the wicked shall find their demise
    ");
    exit(1)
}

pub fn network_error() {
    eprintln!("
    ERROR: NETWORKING ERROR\nAs the floodwaters rise, so does the hand of retribution;\n
    let not the unrepentant deceive themselves, for the deluge of justice knows no mercy
    ");
    exit(1)
}

pub fn argument_error() {
    eprintln!("
    ERROR: ARGUMENTS INVALID\nAs the walls of Jericho crumble, so too does the pride of the defiant;\n
    the trumpet blast heralds the downfall of those who resist divine command
    ");
    exit(1)
}
    
