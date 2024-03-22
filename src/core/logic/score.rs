#[derive(Clone)]
pub enum Action {
    Single,
    Double,
    Triple,
    Tetris, // line clear
    // https://tetris.wiki/T-Spin
    TSpinMiniNoLines,
    TSpinMiniSingle,
    TSpinMiniDouble,
    TSpinNoLines,
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
    BackToBackDifficultLineClears,
    Combo,
    SoftDrop,
    HardDrop,
}

impl Action {
    pub fn as_str(&self) -> &str {
        match *self {
            Action::Single => "Single",
            Action::Double => "Double",
            Action::Triple => "Triple",
            Action::Tetris => "Tetris",
            Action::TSpinMiniNoLines => "T-Spin Mini No Lines",
            Action::TSpinNoLines => "T-Spin No Lines",
            Action::TSpinMiniSingle => "T-Spin Mini Single",
            Action::TSpinSingle => "T-Spin Single",
            Action::TSpinMiniDouble => "T-Spin Mini Double",
            Action::TSpinDouble => "T-Spin Double",
            Action::TSpinTriple => "T-Spin Triple",
            Action::BackToBackDifficultLineClears => "Back-to-Back Difficult Line Clears",
            Action::Combo => "Combo",
            Action::SoftDrop => "Soft Drop",
            Action::HardDrop => "Hard Drop",
        }
    }

    pub fn add_score(&self, level: &f64, action_score: &f64) -> f64 {
        match *self {
            Action::Single => {
                100. * level
                // 100 × level
            }
            Action::Double => {
                300. * level
                // 300 × level
            }
            Action::Triple => {
                500. * level
                // 500 × level
            }
            Action::Tetris => {
                800. * level
                // 800 × level; difficult
            }
            Action::TSpinMiniNoLines => {
                100. * level
                // 100 × level
            }
            Action::TSpinNoLines => {
                400. * level
                // 400 × level
            }
            Action::TSpinMiniSingle => {
                200. * level
                // 200 × level; difficult
            }
            Action::TSpinSingle => {
                800. * level
                // 800 × level; difficult
            }
            Action::TSpinMiniDouble => {
                400. * level
                // 400 × level; difficult
            }
            Action::TSpinDouble => {
                1200. * level
                // 1200 × level; difficult
            }
            Action::TSpinTriple => {
                1600. * level
                // 1600 × level; difficult
            }
            Action::BackToBackDifficultLineClears => {
                // Action score × 1.5 (excluding soft drop and hard drop)
                action_score * 1.5
            }
            Action::Combo => {
                // 50 × combo count × level
                26.
            }
            Action::SoftDrop => {
                // 1 per cell
                1.
            }
            Action::HardDrop => {
                // 2 per cell
                2.
            }
        }
    }

    // pub fn get_garbage(&self, additional: u64) -> u64 {
    //     match *self {
    //         Action::Single => 0,
    //         Action::Double => 1,
    //         Action::Triple => 2,
    //         Action::Tetris => 4,
    //         Action::TSpinMiniNoLines => 0,
    //         Action::TSpinNoLines => 0,
    //         Action::TSpinMiniSingle => 2,
    //         Action::TSpinSingle => 2,
    //         Action::TSpinMiniDouble => 4,
    //         Action::TSpinDouble => 4,
    //         Action::TSpinTriple => 6,
    //         Action::BackToBackDifficultLineClears => {
    //             // Action score × 1.5 (excluding soft drop and hard drop)
    //             action_score * 1.5
    //         }
    //         Action::Combo => {
    //             // 50 × combo count × level
    //             26.
    //         }
    //         Action::SoftDrop => {
    //             // 1 per cell
    //             1.
    //         }
    //         Action::HardDrop => {
    //             // 2 per cell
    //             2.
    //         }
    //     }
    // }
}
