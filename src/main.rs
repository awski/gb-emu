mod gbe;
use gbe::Gbe;

//TODO:
/*
    cartrige loader
    cpu
    address bus
    ppu
    timer
*/

fn main() {
    Gbe::setup().load_cartrige("./test.rom").run();

    // load cartrige
    // init video
    // init cpu
    // timer start
    // game loop
}
