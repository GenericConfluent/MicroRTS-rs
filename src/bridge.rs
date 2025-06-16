// See https://github.com/rlidwka/bevy_mod_physx/blob/3baa1ea0701aaea8faedf3281e488b6f369cfe7b/src/events.rs#L3
// It introduces event channels, on one side of which is mpsc::Sender<T>, and on another
// side is bevy's EventReader<T>, and it automatically bridges between the two.
// This feature is needed to get information from physx callbacks to bevy world.

use bevy::prelude::*;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

#[derive(Resource, Deref, DerefMut)]
struct ChannelReceiver<T>(Mutex<Receiver<T>>);

pub trait BridgeExtension {
    // Allows you to create bevy events using mpsc Sender
    fn add_event_channel<T: Event>(&mut self, receiver: Receiver<T>) -> &mut Self;
}

impl BridgeExtension for App {
    fn add_event_channel<T: Event>(&mut self, receiver: Receiver<T>) -> &mut Self {
        assert!(
            !self.world().contains_resource::<ChannelReceiver<T>>(),
            "this event channel is already initialized",
        );

        self.add_event::<T>();
        self.add_systems(
            First,
            channel_to_event::<T>.after(bevy::ecs::event::event_update_system),
        );
        self.insert_resource(ChannelReceiver(Mutex::new(receiver)));
        self
    }
}

fn channel_to_event<T: 'static + Event + Send + Sync>(
    receiver: Res<ChannelReceiver<T>>,
    mut writer: EventWriter<T>,
) {
    // this should be the only system working with the receiver,
    // thus we always expect to get this lock
    let events = receiver.lock().expect("unable to acquire mutex lock");

    writer.send_batch(events.try_iter());
}
