use crossterm::terminal;

mod keyhandler;

fn main() {

    terminal::enable_raw_mode();
    let receiver = keyhandler::run();
    loop {        
        match keyhandler::handle_input(&receiver) {
            keyhandler::KeyType::Quit => {
                terminal::disable_raw_mode();
                break;
            }
            
            keyhandler::KeyType::Pause => {
                ()
            }

            keyhandler::KeyType::None => {
                ()
            }
        }
        
    }
}
