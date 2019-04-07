//! tracker handles initializing the game world and other state tracking
//! mechanisms.

use std::collections::VecDeque;
use event::Event;
use specs::{
    World,
    Dispatcher
};

pub struct Tracker<'a, 'b> {
    world: &'a World,
    dispatcher: Dispatcher<'a,'b>,
}

impl <'a, 'b> Tracker<'a,'b> {
    #[allow(dead_code)]
    pub fn new(world: &'a mut World, dispatcher: Dispatcher<'a, 'b>) -> Tracker<'a, 'b> {
        let ev: Option<Event> = None;
        world.add_resource(ev);

        Tracker{
            world: world,
            dispatcher: dispatcher,

        }
    }

    #[allow(dead_code)]
    pub fn tick(&self, events: VecDeque<Event>) {
    }
}