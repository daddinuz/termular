# Termular

Termular is a toolkit for the development of [terminal user interfaces](https://en.wikipedia.org/w/index.php?title=Text-based_user_interface&oldid=1067654771) that aims to increase the readability and expressiveness of the code. Both goals are achieved by exposing a lazy [fluent interface](https://en.wikipedia.org/w/index.php?title=Fluent_interface&oldid=1092445824), the latter brings another benefit: greater control over buffers flushing.

### Examples

```Rust
use termular::nio::ReadNonblock;
use termular::printer::{Color, FontWeight, Style};
use termular::screen::Buffer;
use termular::{Mode, Term};

use std::{io, time::Duration};

fn main() -> io::Result<()> {
    let (stdout, stderr) = (io::stdout(), io::stderr());

    let mut term = Term::open(stdout.lock(), stderr.lock())?;
    let center = term.size()? / 2;

    term.set_mode(Mode::Raw)?;
    term.screen()
        .set_buffer(Buffer::Alternate)
        .clear()
        .cursor()
        .hide()
        .set_position(center - [5, 4])
        .printer()
        .using(Style::from(FontWeight::Bold).with_foreground(Color::Green))
        .print("Hello world") // automatically restores default style after printing
        .cursor()
        .set_position(center - [2, 2])
        .printer()
        .print("press")
        .cursor()
        .set_position(center - [5, 1])
        .printer()
        .print("- <SPACE> -")
        .cursor()
        .set_position(center - [3, 0])
        .printer()
        .print("to exit")
        .flush()?; // perform all the actions

    term.stdin_mut()
        .read_timeout_until(b' ', &mut Vec::new(), Duration::from_secs(30))
        .map(|_| ())
}
```

Check out more examples in this [folder](https://github.com/daddinuz/termular/tree/main/examples).

### Alternatives

There are a lot of amazing crates around that try to perform the same task (or a very similar one) as Termular.  
Here is a list, in no particular order, of those from which I drew inspiration:
- [CrossTerm](https://github.com/crossterm-rs/crossterm)
- [Cursive](https://github.com/gyscos/cursive)
- [Termion](https://github.com/redox-os/termion)

### About stability

Termular is in a early development stage so expect it to **break compatibility at any time**.  
Consider this project **unstable** and **not backward compatible**.

### About portability

Currently, only UNIX systems are supported, and very few terminals have been tested.

### About safety

Internally, some unsafe blocks are used, mainly to interface with the C language.

### License

MIT