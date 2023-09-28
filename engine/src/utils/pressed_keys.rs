use std::{sync::atomic::{AtomicBool, Ordering::Relaxed}, ops::Index};
use winit::event::VirtualKeyCode;

macro_rules! pressed_keys {
    ($($key: ident),*) => {
        #[derive(Default)]
        #[allow(non_snake_case)]
        pub struct PressedKeys {
            pub $( $key: AtomicBool ),*
        }
        impl PressedKeys {
            pub fn set(&self, key: VirtualKeyCode, value: bool) {
                match key {
                    $(VirtualKeyCode::$key => self.$key.store(value, Relaxed)),*
                }
            }
        }
        impl Index<VirtualKeyCode> for PressedKeys {
            type Output = bool;
            fn index(&self, index: VirtualKeyCode) -> &Self::Output {
                if match index {
                    $(VirtualKeyCode::$key => self.$key.load(Relaxed)),*
                } { &true } else { &false }
            }
        }
        impl Index<&'static str> for PressedKeys {
            type Output = bool;
            fn index(&self, index: &'static str) -> &Self::Output {
                if match index {
                    $(stringify!($key) => self.$key.load(Relaxed),)*
                    _ => panic!("Key '{index}' not found")
                } { &true } else { &false }
            }
        }
    };
}

pressed_keys!(
    Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Escape,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, Snapshot, Scroll, Pause, Insert, Home,
    Delete, End, PageDown, PageUp, Left, Up, Right, Down, Back, Return, Space, Compose, Caret, Numlock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5,
    Numpad6, Numpad7, Numpad8, Numpad9, NumpadAdd, NumpadDivide, NumpadDecimal, NumpadComma, NumpadEnter, NumpadEquals, NumpadMultiply, NumpadSubtract, AbntC1,
    AbntC2, Apostrophe, Apps, Asterisk, At, Ax, Backslash, Calculator, Capital, Colon, Comma, Convert, Equals, Grave, Kana, Kanji, LAlt, LBracket, LControl,
    LShift, LWin, Mail, MediaSelect, MediaStop, Minus, Mute, MyComputer, NavigateForward, NavigateBackward, NextTrack, NoConvert, OEM102, Period, PlayPause,
    Plus, Power, PrevTrack, RAlt, RBracket, RControl, RShift, RWin, Semicolon, Slash, Sleep, Stop, Sysrq, Tab, Underline, Unlabeled, VolumeDown, VolumeUp, Wake,
    WebBack, WebFavorites, WebForward, WebHome, WebRefresh, WebSearch, WebStop, Yen, Copy, Paste, Cut
);