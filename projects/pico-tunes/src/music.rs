use defmt::trace;
use rtic_monotonics::{systick::Systick, Monotonic};

use self::note::MusicalNote;

pub mod note;
pub mod songs;

pub async fn play_notes(set_top: &mut dyn FnMut(u16), bpm: u16, track: &'static [MusicalNote]) {
    let pwm_freq = 125_000_000.0 / 256.0;

    for note in track {
        let frequency = note.frequency();
        let note_sustain_secs = ((note.sustain * 4) / bpm as u64) * 60;

        // b m s
        //   b m

        let div = pwm_freq / frequency;

        let top: u16 = div as u16;
        set_top(top);

        trace!(
            "{}{=u8} ({=f32}Hz) for {=u64}/{=u64}s",
            note.letter,
            note.octave,
            frequency,
            note_sustain_secs.numer(),
            note_sustain_secs.denom(),
        );

        Systick::delay(<Systick as Monotonic>::Duration::micros(
            (note_sustain_secs * 1_000_000).to_integer() as u32,
        ))
        .await;

        set_top(0);

        let rest_sec = ((note.rest * 4) / bpm as u64) * 60;

        let rest_micros = (rest_sec * 1_000_000).to_integer();

        if rest_micros > 0 {
            trace!(
                "rest for {=u64}/{=u64}s",
                rest_sec.numer(),
                rest_sec.denom(),
            );

            Systick::delay(<Systick as Monotonic>::Duration::micros(rest_micros as u32)).await;
        }
    }
}
