# print-style
Easily write text decorations with Rust's println!

# example
```Rust
use print_style::{Stylize as _, AnsiStylize as _};

println!("{}", "Hello, World".red().bold());

for i in 0..5 {
  print!("{}", format!("{i} / 5").override_line());  // better: print!("{} / 2", i.override_line());
  std::io::stdout().flush().unwrap();
}
println!("{}", "Complete!".bold().blink().bright_white());

let foo = Foo::new();  // Foo implements "Debug"
println!("{:?}", foo.bg_yellow());

let bar = Bar::new();  // Bar implements "Display"
println!("{}", bar.bg_truecolor(128, 50, 255));
```
