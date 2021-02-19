use crossterm::terminal;

mod keyhandler;

fn main() {

    terminal::enable_raw_mode();
    let receiver = keyhandler::run();
    loop { 
        match keyhandler::handle_input(&receiver) {
            keyhandler::KeyType::Quit => {
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

    terminal::disable_raw_mode();

}
