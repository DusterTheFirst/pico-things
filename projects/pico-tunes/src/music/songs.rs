use crate::{music::note::Song, notes};

// pub static MEGALOVANIA: &[MidiNote] = notes! [
//     D   4 for 50 yield for 50,
//     D   4 for 100,
//     D   5 for 100 yield for 100,
//     A   4 for 200 yield for 100,
//     Gsh 4 for 100 yield for 100,
//     G   4 for 100 yield for 100,
//     F   4 for 200,
//     D   4 for 100,
//     F   4 for 100,
//     G   4 for 100
// ];

// pub static JERK_IT_OUT: &[MidiNote] = notes! [
//     Gsh 4 for 500,
//     B   4 for 250,
//     Gsh 4 for 250,
//     B   4 for 200 yield for 50,
//     B   4 for 200 yield for 50,
//     Gsh 4 for 250,
//     B   4 for 250,
//     Fsh 4 for 500,
//     Ash 4 for 250,
//     Fsh 4 for 250,
//     E   4 for 500 yield for 1000
// ];

pub static THE_GOOD_LIFE: Song = Song {
    title: "The Good Life",
    bpm: 136 * 4,
    tracks: [
        notes![
            B   4 for 1/2,
            G   5 for 1/2,
            G   5 for 1/2,
            G   5 for 1/2,
            Fsh 5 for 1/2,
            Fsh 5 for 1/2,
            E   5 for 1/2,
            Fsh 5 for 1,
            E   5 for 1,
            B   4 for 1,
            A   4 for 1/2,
            A   4 for 1,

            Fsh 5 for 1/2,
            Fsh 5 for 1/2,
            Fsh 5 for 1/2,
            Fsh 5 for 1/2,
            E   5 for 1/2,
            E   5 for 1/2,
            D   5 for 1/2,
            E   5 for 1,
            D   5 for 1,
            B   4 for 1,
            A   4 for 1/2,
            G   4 for 1,

            G   5 for 1/2,
            G   5 for 1/2,
            G   5 for 1/2,
            G   5 for 1/2,
            Fsh 5 for 1/2,
            Fsh 5 for 1/2,
            E   5 for 1/2,
            Fsh 5 for 1,
            E   5 for 1,
            E   5 for 1/2,
            G   5 for 1,
            Fsh 5 for 1   yield for 1,
            E   5 for 1/2,
            E   5 for 1/2,
            E   5 for 1/2,
            Fsh 5 for 1,
            G   5 for 1,
            Fsh 5 for 1,
            E   5 for 1/2,
            D   5 for 1,
            G   4 for 1   yield for 1,

            G   5 for 1/2,
            G   5 for 1/2,
            G   5 for 1/2,
            G   5 for 1,
            G   5 for 2   yield for 1/4,
            G   5 for 1/2,
            G   5 for 1/2,
            G   5 for 1,
            Fsh 5 for 2   yield for 1/4,
            Fsh 5 for 1/2,
            Fsh 5 for 1/2,
            Fsh 5 for 1/2,
            Fsh 5 for 1/2,
            G   5 for 1,
            Fsh 5 for 1/2,
            Fsh 5 for 1/2,
        ],
        notes! {
            G 4 for 1   yield for 1/2,
            G 4 for 1   yield for 1/2,
            E 4 for 1,

            G 4 for 1   yield for 1/2,
            G 4 for 1   yield for 1/2,
            E 4 for 1,

            G 4 for 1   yield for 1/2,
            G 4 for 1   yield for 1/2,
            E 4 for 1,

            G 4 for 1   yield for 1/2,
            G 4 for 1   yield for 1/2,
            E 4 for 1,

            G 4 for 1   yield for 1/2,
            G 4 for 1   yield for 1/2,
            E 4 for 1,


            Fsh 4 for 1/2 yield for 1,
            Fsh 4 for 1/2 yield for 1,
            Fsh 4 for 1,
            F   4 for 1/2 yield for 1,
            F   4 for 1/2 yield for 1,
            F   4 for 1,
            Gsh 4 for 1/2 yield for 1,
            Gsh 4 for 1/2 yield for 1,
            Gsh 4 for 1,
            G   4 for 1/2 yield for 1,
            G   4 for 1/2 yield for 1,
            G   4 for 1,

            Fsh 4 for 1/2 yield for 1,
            Fsh 4 for 1/2 yield for 1,
            Fsh 4 for 1,
            F   4 for 1/2 yield for 1,
            F   4 for 1/2 yield for 1,
            F   4 for 1,
            Gsh 4 for 1/2 yield for 1,
            Gsh 4 for 1/2 yield for 1,
            Gsh 4 for 1,
            G   4 for 1/2 yield for 1,
            G   4 for 1/2 yield for 1,
            G   4 for 1,

        },
    ],
};

// pub static THE_GOOD_LIFE_MELODY: &[MidiNote] = notes! [
//     Dsh 5 for 250,
//     Fsh 5 for 250,
//     Fsh 5 for 250,
//     Fsh 5 for 250,
//     F   5 for 250,
//     F   5 for 250,
//     Dsh 5 for 250,
//     F   5 for 500,
//     Dsh 5 for 500,
//     Ash 4 for 500,
//     Gsh 4 for 250,
//     Gsh 4 for 500,
//     F   5 for 250,
//     F   5 for 250,
//     F   5 for 250,
//     F   5 for 250,
//     Dsh 5 for 250,
//     Dsh 5 for 250,
//     Csh 5 for 250,
//     Dsh 5 for 500,
//     Csh 5 for 500,
//     Ash 4 for 500,
//     Gsh 4 for 250,
//     Fsh 4 for 500
// ];

pub static NUMB_LITTLE_BUG: Song = Song {
    title: "Numb Little Bug",
    bpm: 85,
    tracks: [
        notes![
            B 4 for 1/8
            yield for 7/8,

            Ash 4 for 1/8
            yield for 7/8,

            Gsh 4 for 1/8
            yield for 7/8,

            G 4 for 1/8
            yield for 3/8,

            G 5 for 1/48,
            B 5 for 1/48,
            Dsh 6 for 1/48
            yield for 3/16,
            B 3 for 1/16,
            Csh 4 for 3/16,

            Dsh 4 for 3/16,
            B 3 for 1/16,
            Fsh 4 for 1/16,
            Gsh 4 for 1/8,
            Dsh 4 for 5/16
            yield for 1/8,
            B 3 for 1/16,
            Csh 4 for 1/16,

            Dsh 4 for 3/16,
            B 3 for 2/16,
            Csh 4 for 1/8,
            Gsh 3 for 5/16
            yield for 1/8,
            B 3 for 1/16,
            Csh 4 for 1/16,

            Dsh 4 for 1/8,
            Csh 4 for 1/8,
            B 3 for 1/8,
            Csh 4 for 1/8,
            Fsh 4 for 1/8,
            Gsh 4 for 1/16,
            Dsh 4 for 1/16
            yield for 3/16,
            B 3 for 1/16,

            Csh 4 for 1/32, // acciaccatura
            Dsh 4 for 5/32,
            Csh 4 for 1/16,
            B 3 for 3/16,
            Csh 4 for 1/16,
            Csh 4 for 1/4
            yield for 1/8,
            B 3 for 1/16,
            Csh 4 for 1/16,

            Dsh 4 for 3/16,
            B 3 for 1/16,
            Fsh 4 for 1/16,
            Gsh 4 for 1/8,
            Dsh 4 for 5/16
            yield for 1/8,
            B 3 for 1/16,
            Csh 4 for 1/16,

            Dsh 4 for 1/8,
            Csh 4 for 1/16,
            B 3 for 1/8,
            Csh 4 for 1/8,
            Gsh 3 for 5/16
            yield for 1/8,
            B 3 for 1/16,
            Csh 4 for 1/16,
        ],
        notes![
            B 3 for 1/8,
            B 3 for 1/8
            yield for 1/8,
            Fsh 3 for 1/16,
            Dsh 4 for 1/16
            yield for 1/16,
            Dsh 4 for 1/16
            yield for 1/8,
            Fsh 3 for 1/16,
            Gsh 3 for 1/16,
            B 3 for 1/16,
            Gsh 3 for 1/16,

            B 3 for 1/8,
            B 3 for 1/8
            yield for 1/8,
            Fsh 3 for 1/16,
            Dsh 4 for 1/16
            yield for 1/16,
            Dsh 4 for 1/16
            yield for 1/8,
            Fsh 3 for 1/16,
            Gsh 3 for 1/16,
            B 3 for 1/16,
            Gsh 3 for 1/16,

            E 3 for 1/8,
            E 3 for 1/8
            yield for 1/8,
            Fsh 3 for 1/16,
            Dsh 4 for 1/16
            yield for 1/16,
            Dsh 4 for 1/16
            yield for 1/8,
            Fsh 3 for 1/16,
            Gsh 3 for 1/16,
            B 3 for 1/16,
            Gsh 3 for 1/16,

            E 3 for 1/8,
            E 3 for 1/8,
            E 3 for 1/4,
            Fsh 3 for 1/2,

            B 2 for 1/8,
            B 2 for 1/8
            yield for 3/4,

            B 2 for 1/8,
            B 2 for 1/8
            yield for 3/4,

            E 3 for 1/8,
            E 3 for 1/8
            yield for 3/4,

            E 3 for 1/8,
            E 3 for 1/8
            yield for 1/4,
            G 2 for 1/4,
            Fsh 2 for 1/4,

            B 2 for 1/8,
            B 2 for 1/8
            yield for 3/4,

            B 2 for 1/8,
            B 2 for 1/8
            yield for 3/4,
        ],
    ],
};
