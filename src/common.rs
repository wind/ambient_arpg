//use ambient_api::prelude::*;
//use ambient_api::entity::{AnimationAction, AnimationController};
use std::fmt;

macro_rules! enum_ext {
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub u8);

        impl $name {
            $(pub const $variant:Self = $name($val);)*
        }

        impl $name {
            pub fn name(self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                    ,_ => unreachable!(),
                }
            }

            pub fn from(val : u8) -> Self {
                $name(val)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.name())
            }
        }
    };
}

enum_ext! {
    enum Animation {
        Idle01 = 0,
        Idle02 = 1,
        Idle03 = 2,
        BattleRunForward = 3,
        BattleWalkForward = 4,
        BattleWalkBack = 5,
        BattleWalkLeft = 6,
        BattleWalkRight = 7,
        Attack01 = 8,
        Attack02Start = 9,
        Attack02Maintain = 10,
        Attack03Start = 11,
        Attack03Maintain = 12,
        Attack04 = 13,
        GetHit = 14,
        DefendStart = 15,
        Die = 16,
        DieRecovery = 17,
        Count = 18,
    }
}

enum_ext! {
    enum CharacterState {
        Idle = 0,
        Moving = 1,
        Attacking = 2,
        GetHit = 3,
        Dying = 4,
        Count = 5,
    }
}

impl CharacterState {
    pub fn movable(self) -> bool {
        return self == CharacterState::Idle || self == CharacterState::Moving;
    }
}

macro_rules! enum_index {
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        #[derive(Clone, Copy, PartialEq)]
        pub struct $name;

        impl $name {
            $(pub const $variant:usize = $val;)*
        }
    };
}

enum_index! {
    enum Stats {
        Hp = 0,
        Mp = 1,
        MaxHp = 2,
        MaxMp = 3,
        Attack = 4,
        Defense = 5,
        Speed = 6,

        Count = 16,
    }
}

