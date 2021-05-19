use dj8::note::{Note, Pitch};
use dj8::score::Stem;

#[test]
fn test_c_major() {
    let c_major = Stem::Join(vec![
        Stem::Note(Note {
            pitch: Pitch::C3,
            duration: 4.,
        }),
        Stem::Note(Note {
            pitch: Pitch::E3,
            duration: 4.,
        }),
        Stem::Note(Note {
            pitch: Pitch::G3,
            duration: 4.,
        }),
    ]);

    let join_from_abc = Stem::join_from_abc("C,4E,4G,4");
    assert_eq!(c_major, join_from_abc);
}

#[test]
fn test_molody() {
    let melody = Stem::Cat(vec![
        Stem::Note(Note {
            pitch: Pitch::A4,
            duration: 1.,
        }),
        Stem::Note(Note {
            pitch: Pitch::G4,
            duration: 1.,
        }),
        Stem::Note(Note {
            pitch: Pitch::F4,
            duration: 1.,
        }),
        Stem::Note(Note {
            pitch: Pitch::G4,
            duration: 1.,
        }),
    ]);

    let cat_from_abc = Stem::cat_from_abc("AGFG");
    assert_eq!(melody, cat_from_abc);
}

#[test]
fn test_mary() {
    let c_major = Stem::Join(vec![
        Stem::Note(Note {
            pitch: Pitch::C3,
            duration: 4.,
        }),
        Stem::Note(Note {
            pitch: Pitch::E3,
            duration: 4.,
        }),
        Stem::Note(Note {
            pitch: Pitch::G3,
            duration: 4.,
        }),
    ]);

    let f_major = Stem::Join(vec![
        Stem::Note(Note {
            pitch: Pitch::F3,
            duration: 4.,
        }),
        Stem::Note(Note {
            pitch: Pitch::A3,
            duration: 4.,
        }),
        Stem::Note(Note {
            pitch: Pitch::C4,
            duration: 4.,
        }),
    ]);

    let music = Stem::Cat(vec![
        Stem::Join(vec![
            Stem::Cat(vec![
                Stem::Note(Note {
                    pitch: Pitch::A4,
                    duration: 1.,
                }),
                Stem::Note(Note {
                    pitch: Pitch::G4,
                    duration: 1.,
                }),
                Stem::Note(Note {
                    pitch: Pitch::F4,
                    duration: 1.,
                }),
                Stem::Note(Note {
                    pitch: Pitch::G4,
                    duration: 1.,
                }),
            ]),
            f_major.clone(),
        ]),
        Stem::Join(vec![
            Stem::Cat(vec![
                Stem::Note(Note {
                    pitch: Pitch::A4,
                    duration: 1.,
                }),
                Stem::Note(Note {
                    pitch: Pitch::A4,
                    duration: 1.,
                }),
                Stem::Note(Note {
                    pitch: Pitch::A4,
                    duration: 2.,
                }),
            ]),
            f_major.clone(),
        ]),
        Stem::Join(vec![
            Stem::Cat(vec![
                Stem::Note(Note {
                    pitch: Pitch::G4,
                    duration: 1.,
                }),
                Stem::Note(Note {
                    pitch: Pitch::G4,
                    duration: 1.,
                }),
                Stem::Note(Note {
                    pitch: Pitch::G4,
                    duration: 2.,
                }),
            ]),
            c_major.clone(),
        ]),
    ]);

    let stem_from_abc = Stem::Cat(vec![
        Stem::Join(vec![
            Stem::cat_from_abc("AGFG"),
            Stem::join_from_abc("F,4A,4C4"),
        ]),
        Stem::Join(vec![
            Stem::cat_from_abc("AAA2"),
            Stem::join_from_abc("F,4A,4C4"),
        ]),
        Stem::Join(vec![
            Stem::cat_from_abc("GGG2"),
            Stem::join_from_abc("C,4E,4G,4"),
        ]),
    ]);
    assert_eq!(music, stem_from_abc);
}
