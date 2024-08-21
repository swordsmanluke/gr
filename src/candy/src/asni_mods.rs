const CLEAR: &str = "\x1b[0m";
// Text colors

const BLACK: &str = "\x1b[30m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";
const DEFAULT: &str = "\x1b[39m";

// Background colors

const BG_BLACK: &str = "\x1b[40m";
const BG_RED: &str = "\x1b[41m";
const BG_GREEN: &str = "\x1b[42m";
const BG_YELLOW: &str = "\x1b[43m";
const BG_BLUE: &str = "\x1b[44m";
const BG_MAGENTA: &str = "\x1b[45m";
const BG_CYAN: &str = "\x1b[46m";
const BG_WHITE: &str = "\x1b[47m";
const BG_DEFAULT: &str = "\x1b[49m";

// Text styles
const BLINK: &str = "\x1b[5m";
const BOLD: &str = "\x1b[1m";
const UNDERLINE: &str = "\x1b[4m";
const ITALIC: &str = "\x1b[3m";

pub trait AnsiWrapper {
    fn wrap(&self, ansi_esc_seq: &str) -> String;
    // styles
    fn blink(&self) -> String;
    fn bold(&self) -> String;
    fn underline(&self) -> String;

    fn italic(&self) -> String;

    // colors
    fn black(&self) -> String;
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
    fn white(&self) -> String;
    // background colors
    fn on_black(&self) -> String;
    fn on_red(&self) -> String;
    fn on_green(&self) -> String;
    fn on_yellow(&self) -> String;
    fn on_blue(&self) -> String;
    fn on_magenta(&self) -> String;
    fn on_cyan(&self) -> String;
    fn on_white(&self) -> String;
}


impl AnsiWrapper for String {
    fn wrap(&self, ansi_esc_seq: &str) -> String {
        format!("{}{}{}", ansi_esc_seq, self, CLEAR)
    }

    fn blink(&self) -> String {
        self.wrap(BLINK)
    }


    fn bold(&self) -> String {
        self.wrap(BOLD)
    }

    fn underline(&self) -> String {
        self.wrap(UNDERLINE)
    }

    fn italic(&self) -> String {
        self.wrap(ITALIC)
    }

    fn black(&self) -> String {
        self.wrap(BLACK)
    }

    fn red(&self) -> String {
        self.wrap(RED)
    }

    fn green(&self) -> String {
        self.wrap(GREEN)
    }

    fn yellow(&self) -> String {
        self.wrap(YELLOW)
    }

    fn blue(&self) -> String {
        self.wrap(BLUE)
    }

    fn magenta(&self) -> String {
        self.wrap(MAGENTA)
    }

    fn cyan(&self) -> String {
        self.wrap(CYAN)
    }

    fn white(&self) -> String {
        self.wrap(WHITE)
    }

    fn on_black(&self) -> String {
        self.wrap(BG_BLACK)
    }

    fn on_red(&self) -> String {
        self.wrap(BG_RED)
    }

    fn on_green(&self) -> String {
        self.wrap(BG_GREEN)
    }

    fn on_yellow(&self) -> String {
        self.wrap(BG_YELLOW)
    }

    fn on_blue(&self) -> String {
        self.wrap(BG_BLUE)
    }

    fn on_magenta(&self) -> String {
        self.wrap(BG_MAGENTA)
    }

    fn on_cyan(&self) -> String {
        self.wrap(BG_CYAN)
    }

    fn on_white(&self) -> String {
        self.wrap(BG_WHITE)
    }
}


impl AnsiWrapper for &str {
    fn wrap(&self, ansi_esc_seq: &str) -> String {
        format!("{}{}{}", ansi_esc_seq, self, CLEAR)
    }

    fn blink(&self) -> String {
        self.wrap(BLINK)
    }


    fn bold(&self) -> String {
        self.wrap(BOLD)
    }

    fn underline(&self) -> String {
        self.wrap(UNDERLINE)
    }

    fn italic(&self) -> String {
        self.wrap(ITALIC)
    }

    fn black(&self) -> String {
        self.wrap(BLACK)
    }

    fn red(&self) -> String {
        self.wrap(RED)
    }

    fn green(&self) -> String {
        self.wrap(GREEN)
    }

    fn yellow(&self) -> String {
        self.wrap(YELLOW)
    }

    fn blue(&self) -> String {
        self.wrap(BLUE)
    }

    fn magenta(&self) -> String {
        self.wrap(MAGENTA)
    }

    fn cyan(&self) -> String {
        self.wrap(CYAN)
    }

    fn white(&self) -> String {
        self.wrap(WHITE)
    }

    fn on_black(&self) -> String {
        self.wrap(BG_BLACK)
    }

    fn on_red(&self) -> String {
        self.wrap(BG_RED)
    }

    fn on_green(&self) -> String {
        self.wrap(BG_GREEN)
    }

    fn on_yellow(&self) -> String {
        self.wrap(BG_YELLOW)
    }

    fn on_blue(&self) -> String {
        self.wrap(BG_BLUE)
    }

    fn on_magenta(&self) -> String {
        self.wrap(BG_MAGENTA)
    }

    fn on_cyan(&self) -> String {
        self.wrap(BG_CYAN)
    }

    fn on_white(&self) -> String {
        self.wrap(BG_WHITE)
    }
}
