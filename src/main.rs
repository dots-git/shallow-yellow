// The main file. This is what will be executed when opening the program.

#[macro_use]
extern  crate conrod;

mod interface;

fn main()
{
    interface::open_window();
}