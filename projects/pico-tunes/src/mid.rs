use midi_msg::{MidiMsg, ReceiverContext};

mod songs;

pub fn convert_midi(midi: &[u8]) {
    let mut context = ReceiverContext::new();

    let (midi, message_length) = MidiMsg::from_midi_with_context(midi, &mut context).unwrap();

    match midi {
        MidiMsg::ChannelVoice { channel, msg } => todo!(),
        MidiMsg::RunningChannelVoice { channel, msg } => todo!(),
        MidiMsg::ChannelMode { channel, msg } => todo!(),
        MidiMsg::RunningChannelMode { channel, msg } => todo!(),
        MidiMsg::SystemCommon { msg } => todo!(),
        MidiMsg::SystemRealTime { msg } => todo!(),
    }
}
