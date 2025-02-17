use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    str::FromStr,
};

use floem::keyboard::{Key, KeyCode, NativeKey};

#[derive(Clone, Debug, Eq)]
pub(crate) enum KeyInput {
    Keyboard(floem::keyboard::Key, floem::keyboard::KeyCode),
    Pointer(floem::pointer::PointerButton),
}

impl KeyInput {
    fn keyboard_from_str(s: &str) -> Option<(Key, KeyCode)> {
        // Checks if it is a character key
        fn is_key_string(s: &str) -> bool {
            s.chars().all(|c| !c.is_control())
                && s.chars().skip(1).all(|c| !c.is_ascii())
        }

        fn char_to_keycode(char: &str) -> KeyCode {
            match char {
                "a" => KeyCode::KeyA,
                "b" => KeyCode::KeyB,
                "c" => KeyCode::KeyC,
                "d" => KeyCode::KeyD,
                "e" => KeyCode::KeyE,
                "f" => KeyCode::KeyF,
                "g" => KeyCode::KeyG,
                "h" => KeyCode::KeyH,
                "i" => KeyCode::KeyI,
                "j" => KeyCode::KeyJ,
                "k" => KeyCode::KeyK,
                "l" => KeyCode::KeyL,
                "m" => KeyCode::KeyM,
                "n" => KeyCode::KeyN,
                "o" => KeyCode::KeyO,
                "p" => KeyCode::KeyP,
                "q" => KeyCode::KeyQ,
                "r" => KeyCode::KeyR,
                "s" => KeyCode::KeyS,
                "t" => KeyCode::KeyT,
                "u" => KeyCode::KeyU,
                "v" => KeyCode::KeyV,
                "w" => KeyCode::KeyW,
                "x" => KeyCode::KeyX,
                "y" => KeyCode::KeyY,
                "z" => KeyCode::KeyZ,
                "=" => KeyCode::Equal,
                "-" => KeyCode::Minus,
                "0" => KeyCode::Digit0,
                "1" => KeyCode::Digit1,
                "2" => KeyCode::Digit2,
                "3" => KeyCode::Digit3,
                "4" => KeyCode::Digit4,
                "5" => KeyCode::Digit5,
                "6" => KeyCode::Digit6,
                "7" => KeyCode::Digit7,
                "8" => KeyCode::Digit8,
                "9" => KeyCode::Digit9,
                "`" => KeyCode::Backquote,
                "/" => KeyCode::Slash,
                "\\" => KeyCode::Backslash,
                "," => KeyCode::Comma,
                "." => KeyCode::Period,
                "*" => KeyCode::NumpadMultiply,
                "+" => KeyCode::NumpadAdd,
                ";" => KeyCode::Semicolon,
                "'" => KeyCode::Quote,
                "[" => KeyCode::BracketLeft,
                "]" => KeyCode::BracketRight,
                "<" => KeyCode::IntlBackslash,
                _ => KeyCode::Fn,
            }
        }

        // Import into scope to reduce noise
        use floem::keyboard::Key::*;
        Some(match s {
            s if is_key_string(s) => {
                let char = Character(s.into());
                (char.clone(), char_to_keycode(char.to_text().unwrap()))
            }
            "unidentified" => (Unidentified(NativeKey::Unidentified), KeyCode::Fn),
            "alt" => (Alt, KeyCode::AltLeft),
            "altgraph" => (AltGraph, KeyCode::AltRight),
            "capslock" => (CapsLock, KeyCode::CapsLock),
            "control" => (Control, KeyCode::ControlLeft),
            "fn" => (Fn, KeyCode::Fn),
            "fnlock" => (FnLock, KeyCode::FnLock),
            "meta" => (Meta, KeyCode::Meta),
            "numlock" => (NumLock, KeyCode::NumLock),
            "scrolllock" => (ScrollLock, KeyCode::ScrollLock),
            "shift" => (Shift, KeyCode::ShiftLeft),
            "symbol" => (Symbol, KeyCode::Fn),
            "symbollock" => (SymbolLock, KeyCode::Fn),
            "hyper" => (Hyper, KeyCode::Hyper),
            "super" => (Super, KeyCode::Meta),
            "enter" => (Enter, KeyCode::Enter),
            "tab" => (Tab, KeyCode::Tab),
            "arrowdown" => (ArrowDown, KeyCode::ArrowDown),
            "arrowleft" => (ArrowLeft, KeyCode::ArrowLeft),
            "arrowright" => (ArrowRight, KeyCode::ArrowRight),
            "arrowup" => (ArrowUp, KeyCode::ArrowUp),
            "end" => (End, KeyCode::End),
            "home" => (Home, KeyCode::Home),
            "pagedown" => (PageDown, KeyCode::PageDown),
            "pageup" => (PageUp, KeyCode::PageUp),
            "backspace" => (Backspace, KeyCode::Backspace),
            "clear" => (Clear, KeyCode::Fn),
            "copy" => (Copy, KeyCode::Copy),
            "crsel" => (CrSel, KeyCode::Fn),
            "cut" => (Cut, KeyCode::Cut),
            "delete" => (Delete, KeyCode::Delete),
            "eraseeof" => (EraseEof, KeyCode::Fn),
            "exsel" => (ExSel, KeyCode::Fn),
            "insert" => (Insert, KeyCode::Insert),
            "paste" => (Paste, KeyCode::Paste),
            "redo" => (Redo, KeyCode::Fn),
            "undo" => (Undo, KeyCode::Undo),
            "accept" => (Accept, KeyCode::Fn),
            "again" => (Again, KeyCode::Again),
            "attn" => (Attn, KeyCode::Fn),
            "cancel" => (Cancel, KeyCode::Fn),
            "contextmenu" => (ContextMenu, KeyCode::ContextMenu),
            "escape" => (Escape, KeyCode::Escape),
            "execute" => (Execute, KeyCode::Fn),
            "find" => (Find, KeyCode::Find),
            "help" => (Help, KeyCode::Help),
            "pause" => (Pause, KeyCode::Pause),
            "play" => (Play, KeyCode::MediaPlayPause),
            "props" => (Props, KeyCode::Props),
            "select" => (Select, KeyCode::Select),
            "zoomin" => (ZoomIn, KeyCode::Fn),
            "zoomout" => (ZoomOut, KeyCode::Fn),
            "brightnessdown" => (BrightnessDown, KeyCode::Fn),
            "brightnessup" => (BrightnessUp, KeyCode::Fn),
            "eject" => (Eject, KeyCode::Eject),
            "logoff" => (LogOff, KeyCode::Fn),
            "power" => (Power, KeyCode::Power),
            "poweroff" => (PowerOff, KeyCode::Fn),
            "printscreen" => (PrintScreen, KeyCode::PrintScreen),
            "hibernate" => (Hibernate, KeyCode::Fn),
            "standby" => (Standby, KeyCode::Fn),
            "wakeup" => (WakeUp, KeyCode::WakeUp),
            "allcandidates" => (AllCandidates, KeyCode::Fn),
            "alphanumeric" => (Alphanumeric, KeyCode::Fn),
            "codeinput" => (CodeInput, KeyCode::Fn),
            "compose" => (Compose, KeyCode::Fn),
            "convert" => (Convert, KeyCode::Convert),
            "dead" => (Dead(None), KeyCode::Fn),
            "finalmode" => (FinalMode, KeyCode::Fn),
            "groupfirst" => (GroupFirst, KeyCode::Fn),
            "grouplast" => (GroupLast, KeyCode::Fn),
            "groupnext" => (GroupNext, KeyCode::Fn),
            "groupprevious" => (GroupPrevious, KeyCode::Fn),
            "modechange" => (ModeChange, KeyCode::Fn),
            "nextcandidate" => (NextCandidate, KeyCode::Fn),
            "nonconvert" => (NonConvert, KeyCode::NonConvert),
            "previouscandidate" => (PreviousCandidate, KeyCode::Fn),
            "process" => (Process, KeyCode::Fn),
            "singlecandidate" => (SingleCandidate, KeyCode::Fn),
            "hangulmode" => (HangulMode, KeyCode::Fn),
            "hanjamode" => (HanjaMode, KeyCode::Fn),
            "junjamode" => (JunjaMode, KeyCode::Fn),
            "eisu" => (Eisu, KeyCode::Fn),
            "hankaku" => (Hankaku, KeyCode::Fn),
            "hiragana" => (Hiragana, KeyCode::Hiragana),
            "hiraganakatakana" => (HiraganaKatakana, KeyCode::Fn),
            "kanamode" => (KanaMode, KeyCode::KanaMode),
            "kanjimode" => (KanjiMode, KeyCode::Fn),
            "katakana" => (Katakana, KeyCode::Katakana),
            "romaji" => (Romaji, KeyCode::Fn),
            "zenkaku" => (Zenkaku, KeyCode::Fn),
            "zenkakuhankaku" => (ZenkakuHankaku, KeyCode::Fn),
            "f1" => (F1, KeyCode::F1),
            "f2" => (F2, KeyCode::F2),
            "f3" => (F3, KeyCode::F3),
            "f4" => (F4, KeyCode::F4),
            "f5" => (F5, KeyCode::F5),
            "f6" => (F6, KeyCode::F6),
            "f7" => (F7, KeyCode::F7),
            "f8" => (F8, KeyCode::F8),
            "f9" => (F9, KeyCode::F9),
            "f10" => (F10, KeyCode::F10),
            "f11" => (F11, KeyCode::F11),
            "f12" => (F12, KeyCode::F12),
            "soft1" => (Soft1, KeyCode::Fn),
            "soft2" => (Soft2, KeyCode::Fn),
            "soft3" => (Soft3, KeyCode::Fn),
            "soft4" => (Soft4, KeyCode::Fn),
            "channeldown" => (ChannelDown, KeyCode::Fn),
            "channelup" => (ChannelUp, KeyCode::Fn),
            "close" => (Close, KeyCode::Fn),
            "mailforward" => (MailForward, KeyCode::Fn),
            "mailreply" => (MailReply, KeyCode::Fn),
            "mailsend" => (MailSend, KeyCode::Fn),
            "mediaclose" => (MediaClose, KeyCode::Fn),
            "mediafastforward" => (MediaFastForward, KeyCode::Fn),
            "mediapause" => (MediaPause, KeyCode::Fn),
            "mediaplay" => (MediaPlay, KeyCode::Fn),
            "mediaplaypause" => (MediaPlayPause, KeyCode::MediaPlayPause),
            "mediarecord" => (MediaRecord, KeyCode::Fn),
            "mediarewind" => (MediaRewind, KeyCode::Fn),
            "mediastop" => (MediaStop, KeyCode::MediaStop),
            "mediatracknext" => (MediaTrackNext, KeyCode::MediaTrackNext),
            "mediatrackprevious" => {
                (MediaTrackPrevious, KeyCode::MediaTrackPrevious)
            }
            "new" => (New, KeyCode::Fn),
            "open" => (Open, KeyCode::Open),
            "print" => (Print, KeyCode::Fn),
            "save" => (Save, KeyCode::Fn),
            "spellcheck" => (SpellCheck, KeyCode::Fn),
            "key11" => (Key11, KeyCode::Fn),
            "key12" => (Key12, KeyCode::Fn),
            "audiobalanceleft" => (AudioBalanceLeft, KeyCode::Fn),
            "audiobalanceright" => (AudioBalanceRight, KeyCode::Fn),
            "audiobassboostdown" => (AudioBassBoostDown, KeyCode::Fn),
            "audiobassboosttoggle" => (AudioBassBoostToggle, KeyCode::Fn),
            "audiobassboostup" => (AudioBassBoostUp, KeyCode::Fn),
            "audiofaderfront" => (AudioFaderFront, KeyCode::Fn),
            "audiofaderrear" => (AudioFaderRear, KeyCode::Fn),
            "audiosurroundmodenext" => (AudioSurroundModeNext, KeyCode::Fn),
            "audiotrebledown" => (AudioTrebleDown, KeyCode::Fn),
            "audiotrebleup" => (AudioTrebleUp, KeyCode::Fn),
            "audiovolumedown" => (AudioVolumeDown, KeyCode::AudioVolumeDown),
            "audiovolumeup" => (AudioVolumeUp, KeyCode::AudioVolumeUp),
            "audiovolumemute" => (AudioVolumeMute, KeyCode::AudioVolumeMute),
            "microphonetoggle" => (MicrophoneToggle, KeyCode::Fn),
            "microphonevolumedown" => (MicrophoneVolumeDown, KeyCode::Fn),
            "microphonevolumeup" => (MicrophoneVolumeUp, KeyCode::Fn),
            "microphonevolumemute" => (MicrophoneVolumeMute, KeyCode::Fn),
            "speechcorrectionlist" => (SpeechCorrectionList, KeyCode::Fn),
            "speechinputtoggle" => (SpeechInputToggle, KeyCode::Fn),
            "launchapplication1" => (LaunchApplication1, KeyCode::Fn),
            "launchapplication2" => (LaunchApplication2, KeyCode::Fn),
            "launchcalendar" => (LaunchCalendar, KeyCode::Fn),
            "launchcontacts" => (LaunchContacts, KeyCode::Fn),
            "launchmail" => (LaunchMail, KeyCode::LaunchMail),
            "launchmediaplayer" => (LaunchMediaPlayer, KeyCode::Fn),
            "launchmusicplayer" => (LaunchMusicPlayer, KeyCode::Fn),
            "launchphone" => (LaunchPhone, KeyCode::Fn),
            "launchscreensaver" => (LaunchScreenSaver, KeyCode::Fn),
            "launchspreadsheet" => (LaunchSpreadsheet, KeyCode::Fn),
            "launchwebbrowser" => (LaunchWebBrowser, KeyCode::Fn),
            "launchwebcam" => (LaunchWebCam, KeyCode::Fn),
            "launchwordprocessor" => (LaunchWordProcessor, KeyCode::Fn),
            "browserback" => (BrowserBack, KeyCode::BrowserBack),
            "browserfavorites" => (BrowserFavorites, KeyCode::BrowserFavorites),
            "browserforward" => (BrowserForward, KeyCode::BrowserForward),
            "browserhome" => (BrowserHome, KeyCode::BrowserHome),
            "browserrefresh" => (BrowserRefresh, KeyCode::BrowserRefresh),
            "browsersearch" => (BrowserSearch, KeyCode::BrowserSearch),
            "browserstop" => (BrowserStop, KeyCode::BrowserStop),
            "appswitch" => (AppSwitch, KeyCode::Fn),
            "call" => (Call, KeyCode::Fn),
            "camera" => (Camera, KeyCode::Fn),
            "camerafocus" => (CameraFocus, KeyCode::Fn),
            "endcall" => (EndCall, KeyCode::Fn),
            "goback" => (GoBack, KeyCode::Fn),
            "gohome" => (GoHome, KeyCode::Fn),
            "headsethook" => (HeadsetHook, KeyCode::Fn),
            "lastnumberredial" => (LastNumberRedial, KeyCode::Fn),
            "notification" => (Notification, KeyCode::Fn),
            "mannermode" => (MannerMode, KeyCode::Fn),
            "voicedial" => (VoiceDial, KeyCode::Fn),
            "tv" => (TV, KeyCode::Fn),
            "tv3dmode" => (TV3DMode, KeyCode::Fn),
            "tvantennacable" => (TVAntennaCable, KeyCode::Fn),
            "tvaudiodescription" => (TVAudioDescription, KeyCode::Fn),
            "tvaudiodescriptionmixdown" => (TVAudioDescriptionMixDown, KeyCode::Fn),
            "tvaudiodescriptionmixup" => (TVAudioDescriptionMixUp, KeyCode::Fn),
            "tvcontentsmenu" => (TVContentsMenu, KeyCode::Fn),
            "tvdataservice" => (TVDataService, KeyCode::Fn),
            "tvinput" => (TVInput, KeyCode::Fn),
            "tvinputcomponent1" => (TVInputComponent1, KeyCode::Fn),
            "tvinputcomponent2" => (TVInputComponent2, KeyCode::Fn),
            "tvinputcomposite1" => (TVInputComposite1, KeyCode::Fn),
            "tvinputcomposite2" => (TVInputComposite2, KeyCode::Fn),
            "tvinputhdmi1" => (TVInputHDMI1, KeyCode::Fn),
            "tvinputhdmi2" => (TVInputHDMI2, KeyCode::Fn),
            "tvinputhdmi3" => (TVInputHDMI3, KeyCode::Fn),
            "tvinputhdmi4" => (TVInputHDMI4, KeyCode::Fn),
            "tvinputvga1" => (TVInputVGA1, KeyCode::Fn),
            "tvmediacontext" => (TVMediaContext, KeyCode::Fn),
            "tvnetwork" => (TVNetwork, KeyCode::Fn),
            "tvnumberentry" => (TVNumberEntry, KeyCode::Fn),
            "tvpower" => (TVPower, KeyCode::Fn),
            "tvradioservice" => (TVRadioService, KeyCode::Fn),
            "tvsatellite" => (TVSatellite, KeyCode::Fn),
            "tvsatellitebs" => (TVSatelliteBS, KeyCode::Fn),
            "tvsatellitecs" => (TVSatelliteCS, KeyCode::Fn),
            "tvsatellitetoggle" => (TVSatelliteToggle, KeyCode::Fn),
            "tvterrestrialanalog" => (TVTerrestrialAnalog, KeyCode::Fn),
            "tvterrestrialdigital" => (TVTerrestrialDigital, KeyCode::Fn),
            "tvtimer" => (TVTimer, KeyCode::Fn),
            "avrinput" => (AVRInput, KeyCode::Fn),
            "avrpower" => (AVRPower, KeyCode::Fn),
            "colorf0red" => (ColorF0Red, KeyCode::Fn),
            "colorf1green" => (ColorF1Green, KeyCode::Fn),
            "colorf2yellow" => (ColorF2Yellow, KeyCode::Fn),
            "colorf3blue" => (ColorF3Blue, KeyCode::Fn),
            "colorf4grey" => (ColorF4Grey, KeyCode::Fn),
            "colorf5brown" => (ColorF5Brown, KeyCode::Fn),
            "closedcaptiontoggle" => (ClosedCaptionToggle, KeyCode::Fn),
            "dimmer" => (Dimmer, KeyCode::Fn),
            "displayswap" => (DisplaySwap, KeyCode::Fn),
            "dvr" => (DVR, KeyCode::Fn),
            "exit" => (Exit, KeyCode::Fn),
            "favoriteclear0" => (FavoriteClear0, KeyCode::Fn),
            "favoriteclear1" => (FavoriteClear1, KeyCode::Fn),
            "favoriteclear2" => (FavoriteClear2, KeyCode::Fn),
            "favoriteclear3" => (FavoriteClear3, KeyCode::Fn),
            "favoriterecall0" => (FavoriteRecall0, KeyCode::Fn),
            "favoriterecall1" => (FavoriteRecall1, KeyCode::Fn),
            "favoriterecall2" => (FavoriteRecall2, KeyCode::Fn),
            "favoriterecall3" => (FavoriteRecall3, KeyCode::Fn),
            "favoritestore0" => (FavoriteStore0, KeyCode::Fn),
            "favoritestore1" => (FavoriteStore1, KeyCode::Fn),
            "favoritestore2" => (FavoriteStore2, KeyCode::Fn),
            "favoritestore3" => (FavoriteStore3, KeyCode::Fn),
            "guide" => (Guide, KeyCode::Fn),
            "guidenextday" => (GuideNextDay, KeyCode::Fn),
            "guidepreviousday" => (GuidePreviousDay, KeyCode::Fn),
            "info" => (Info, KeyCode::Fn),
            "instantreplay" => (InstantReplay, KeyCode::Fn),
            "link" => (Link, KeyCode::Fn),
            "listprogram" => (ListProgram, KeyCode::Fn),
            "livecontent" => (LiveContent, KeyCode::Fn),
            "lock" => (Lock, KeyCode::Fn),
            "mediaapps" => (MediaApps, KeyCode::Fn),
            "mediaaudiotrack" => (MediaAudioTrack, KeyCode::Fn),
            "medialast" => (MediaLast, KeyCode::Fn),
            "mediaskipbackward" => (MediaSkipBackward, KeyCode::Fn),
            "mediaskipforward" => (MediaSkipForward, KeyCode::Fn),
            "mediastepbackward" => (MediaStepBackward, KeyCode::Fn),
            "mediastepforward" => (MediaStepForward, KeyCode::Fn),
            "mediatopmenu" => (MediaTopMenu, KeyCode::Fn),
            "navigatein" => (NavigateIn, KeyCode::Fn),
            "navigatenext" => (NavigateNext, KeyCode::Fn),
            "navigateout" => (NavigateOut, KeyCode::Fn),
            "navigateprevious" => (NavigatePrevious, KeyCode::Fn),
            "nextfavoritechannel" => (NextFavoriteChannel, KeyCode::Fn),
            "nextuserprofile" => (NextUserProfile, KeyCode::Fn),
            "ondemand" => (OnDemand, KeyCode::Fn),
            "pairing" => (Pairing, KeyCode::Fn),
            "pinpdown" => (PinPDown, KeyCode::Fn),
            "pinpmove" => (PinPMove, KeyCode::Fn),
            "pinptoggle" => (PinPToggle, KeyCode::Fn),
            "pinpup" => (PinPUp, KeyCode::Fn),
            "playspeeddown" => (PlaySpeedDown, KeyCode::Fn),
            "playspeedreset" => (PlaySpeedReset, KeyCode::Fn),
            "playspeedup" => (PlaySpeedUp, KeyCode::Fn),
            "randomtoggle" => (RandomToggle, KeyCode::Fn),
            "rclowbattery" => (RcLowBattery, KeyCode::Fn),
            "recordspeednext" => (RecordSpeedNext, KeyCode::Fn),
            "rfbypass" => (RfBypass, KeyCode::Fn),
            "scanchannelstoggle" => (ScanChannelsToggle, KeyCode::Fn),
            "screenmodenext" => (ScreenModeNext, KeyCode::Fn),
            "settings" => (Settings, KeyCode::Fn),
            "splitscreentoggle" => (SplitScreenToggle, KeyCode::Fn),
            "stbinput" => (STBInput, KeyCode::Fn),
            "stbpower" => (STBPower, KeyCode::Fn),
            "subtitle" => (Subtitle, KeyCode::Fn),
            "teletext" => (Teletext, KeyCode::Fn),
            "videomodenext" => (VideoModeNext, KeyCode::Fn),
            "wink" => (Wink, KeyCode::Fn),
            "zoomtoggle" => (ZoomToggle, KeyCode::Fn),

            // Custom key name mappings
            "esc" => (Escape, KeyCode::Escape),
            "space" => (Character(" ".into()), KeyCode::Space),
            "bs" => (Backspace, KeyCode::Backspace),
            "up" => (ArrowUp, KeyCode::ArrowUp),
            "down" => (ArrowDown, KeyCode::ArrowDown),
            "right" => (ArrowRight, KeyCode::ArrowRight),
            "left" => (ArrowLeft, KeyCode::ArrowLeft),
            "del" => (Delete, KeyCode::Delete),

            _ => return None,
        })
    }

    fn mouse_from_str(s: &str) -> Option<floem::pointer::PointerButton> {
        use floem::pointer::PointerButton as B;

        Some(match s {
            "mousemiddle" => B::Auxiliary,
            "mouseforward" => B::X2,
            "mousebackward" => B::X1,
            _ => return None,
        })
    }
}

impl Display for KeyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use floem::pointer::PointerButton as B;

        match self {
            Self::Keyboard(_key, key_code) => match key_code {
                KeyCode::Unidentified(_) => f.write_str("Unidentified"),
                KeyCode::Backquote => f.write_str("`"),
                KeyCode::Backslash => f.write_str("\\"),
                KeyCode::BracketLeft => f.write_str("["),
                KeyCode::BracketRight => f.write_str("]"),
                KeyCode::Comma => f.write_str(","),
                KeyCode::Digit0 => f.write_str("0"),
                KeyCode::Digit1 => f.write_str("1"),
                KeyCode::Digit2 => f.write_str("2"),
                KeyCode::Digit3 => f.write_str("3"),
                KeyCode::Digit4 => f.write_str("4"),
                KeyCode::Digit5 => f.write_str("5"),
                KeyCode::Digit6 => f.write_str("6"),
                KeyCode::Digit7 => f.write_str("7"),
                KeyCode::Digit8 => f.write_str("8"),
                KeyCode::Digit9 => f.write_str("9"),
                KeyCode::Equal => f.write_str("="),
                KeyCode::IntlBackslash => f.write_str("<"),
                KeyCode::IntlRo => f.write_str("IntlRo"),
                KeyCode::IntlYen => f.write_str("IntlYen"),
                KeyCode::KeyA => f.write_str("A"),
                KeyCode::KeyB => f.write_str("B"),
                KeyCode::KeyC => f.write_str("C"),
                KeyCode::KeyD => f.write_str("D"),
                KeyCode::KeyE => f.write_str("E"),
                KeyCode::KeyF => f.write_str("F"),
                KeyCode::KeyG => f.write_str("G"),
                KeyCode::KeyH => f.write_str("H"),
                KeyCode::KeyI => f.write_str("I"),
                KeyCode::KeyJ => f.write_str("J"),
                KeyCode::KeyK => f.write_str("K"),
                KeyCode::KeyL => f.write_str("L"),
                KeyCode::KeyM => f.write_str("M"),
                KeyCode::KeyN => f.write_str("N"),
                KeyCode::KeyO => f.write_str("O"),
                KeyCode::KeyP => f.write_str("P"),
                KeyCode::KeyQ => f.write_str("Q"),
                KeyCode::KeyR => f.write_str("R"),
                KeyCode::KeyS => f.write_str("S"),
                KeyCode::KeyT => f.write_str("T"),
                KeyCode::KeyU => f.write_str("U"),
                KeyCode::KeyV => f.write_str("V"),
                KeyCode::KeyW => f.write_str("W"),
                KeyCode::KeyX => f.write_str("X"),
                KeyCode::KeyY => f.write_str("Y"),
                KeyCode::KeyZ => f.write_str("Z"),
                KeyCode::Minus => f.write_str("-"),
                KeyCode::Period => f.write_str("."),
                KeyCode::Quote => f.write_str("'"),
                KeyCode::Semicolon => f.write_str(";"),
                KeyCode::Slash => f.write_str("/"),
                KeyCode::AltLeft => f.write_str("Alt"),
                KeyCode::AltRight => f.write_str("Alt"),
                KeyCode::Backspace => f.write_str("backspace"),
                KeyCode::CapsLock => f.write_str("CapsLock"),
                KeyCode::ContextMenu => f.write_str("ContextMenu"),
                KeyCode::ControlLeft => f.write_str("Ctrl"),
                KeyCode::ControlRight => f.write_str("Ctrl"),
                KeyCode::Enter => f.write_str("Enter"),
                KeyCode::SuperLeft => match std::env::consts::OS {
                    "macos" => f.write_str("Cmd"),
                    "windows" => f.write_str("Win"),
                    _ => f.write_str("Meta"),
                },
                KeyCode::SuperRight => match std::env::consts::OS {
                    "macos" => f.write_str("Cmd"),
                    "windows" => f.write_str("Win"),
                    _ => f.write_str("Meta"),
                },
                KeyCode::ShiftLeft => f.write_str("Shift"),
                KeyCode::ShiftRight => f.write_str("Shift"),
                KeyCode::Space => f.write_str("Space"),
                KeyCode::Tab => f.write_str("Tab"),
                KeyCode::Convert => f.write_str("Convert"),
                KeyCode::KanaMode => f.write_str("KanaMode"),
                KeyCode::Lang1 => f.write_str("Lang1"),
                KeyCode::Lang2 => f.write_str("Lang2"),
                KeyCode::Lang3 => f.write_str("Lang3"),
                KeyCode::Lang4 => f.write_str("Lang4"),
                KeyCode::Lang5 => f.write_str("Lang5"),
                KeyCode::NonConvert => f.write_str("NonConvert"),
                KeyCode::Delete => f.write_str("Delete"),
                KeyCode::End => f.write_str("End"),
                KeyCode::Help => f.write_str("Help"),
                KeyCode::Home => f.write_str("Home"),
                KeyCode::Insert => f.write_str("Insert"),
                KeyCode::PageDown => f.write_str("PageDown"),
                KeyCode::PageUp => f.write_str("PageUp"),
                KeyCode::ArrowDown => f.write_str("Down"),
                KeyCode::ArrowLeft => f.write_str("Left"),
                KeyCode::ArrowRight => f.write_str("Right"),
                KeyCode::ArrowUp => f.write_str("Up"),
                KeyCode::NumLock => f.write_str("NumLock"),
                KeyCode::Numpad0 => f.write_str("Numpad0"),
                KeyCode::Numpad1 => f.write_str("Numpad1"),
                KeyCode::Numpad2 => f.write_str("Numpad2"),
                KeyCode::Numpad3 => f.write_str("Numpad3"),
                KeyCode::Numpad4 => f.write_str("Numpad4"),
                KeyCode::Numpad5 => f.write_str("Numpad5"),
                KeyCode::Numpad6 => f.write_str("Numpad6"),
                KeyCode::Numpad7 => f.write_str("Numpad7"),
                KeyCode::Numpad8 => f.write_str("Numpad8"),
                KeyCode::Numpad9 => f.write_str("Numpad9"),
                KeyCode::NumpadAdd => f.write_str("NumpadAdd"),
                KeyCode::NumpadBackspace => f.write_str("NumpadBackspace"),
                KeyCode::NumpadClear => f.write_str("NumpadClear"),
                KeyCode::NumpadClearEntry => f.write_str("NumpadClearEntry"),
                KeyCode::NumpadComma => f.write_str("NumpadComma"),
                KeyCode::NumpadDecimal => f.write_str("NumpadDecimal"),
                KeyCode::NumpadDivide => f.write_str("NumpadDivide"),
                KeyCode::NumpadEnter => f.write_str("NumpadEnter"),
                KeyCode::NumpadEqual => f.write_str("NumpadEqual"),
                KeyCode::NumpadHash => f.write_str("NumpadHash"),
                KeyCode::NumpadMemoryAdd => f.write_str("NumpadMemoryAdd"),
                KeyCode::NumpadMemoryClear => f.write_str("NumpadMemoryClear"),
                KeyCode::NumpadMemoryRecall => f.write_str("NumpadMemoryRecall"),
                KeyCode::NumpadMemoryStore => f.write_str("NumpadMemoryStore"),
                KeyCode::NumpadMemorySubtract => f.write_str("NumpadMemorySubtract"),
                KeyCode::NumpadMultiply => f.write_str("NumpadMultiply"),
                KeyCode::NumpadParenLeft => f.write_str("NumpadParenLeft"),
                KeyCode::NumpadParenRight => f.write_str("NumpadParenRight"),
                KeyCode::NumpadStar => f.write_str("NumpadStar"),
                KeyCode::NumpadSubtract => f.write_str("NumpadSubtract"),
                KeyCode::Escape => f.write_str("Escape"),
                KeyCode::Fn => f.write_str("Fn"),
                KeyCode::FnLock => f.write_str("FnLock"),
                KeyCode::PrintScreen => f.write_str("PrintScreen"),
                KeyCode::ScrollLock => f.write_str("ScrollLock"),
                KeyCode::Pause => f.write_str("Pause"),
                KeyCode::BrowserBack => f.write_str("BrowserBack"),
                KeyCode::BrowserFavorites => f.write_str("BrowserFavorites"),
                KeyCode::BrowserForward => f.write_str("BrowserForward"),
                KeyCode::BrowserHome => f.write_str("BrowserHome"),
                KeyCode::BrowserRefresh => f.write_str("BrowserRefresh"),
                KeyCode::BrowserSearch => f.write_str("BrowserSearch"),
                KeyCode::BrowserStop => f.write_str("BrowserStop"),
                KeyCode::Eject => f.write_str("Eject"),
                KeyCode::LaunchApp1 => f.write_str("LaunchApp1"),
                KeyCode::LaunchApp2 => f.write_str("LaunchApp2"),
                KeyCode::LaunchMail => f.write_str("LaunchMail"),
                KeyCode::MediaPlayPause => f.write_str("MediaPlayPause"),
                KeyCode::MediaSelect => f.write_str("MediaSelect"),
                KeyCode::MediaStop => f.write_str("MediaStop"),
                KeyCode::MediaTrackNext => f.write_str("MediaTrackNext"),
                KeyCode::MediaTrackPrevious => f.write_str("MediaTrackPrevious"),
                KeyCode::Power => f.write_str("Power"),
                KeyCode::Sleep => f.write_str("Sleep"),
                KeyCode::AudioVolumeDown => f.write_str("AudioVolumeDown"),
                KeyCode::AudioVolumeMute => f.write_str("AudioVolumeMute"),
                KeyCode::AudioVolumeUp => f.write_str("AudioVolumeUp"),
                KeyCode::WakeUp => f.write_str("WakeUp"),
                KeyCode::Meta => match std::env::consts::OS {
                    "macos" => f.write_str("Cmd"),
                    "windows" => f.write_str("Win"),
                    _ => f.write_str("Meta"),
                },
                KeyCode::Hyper => f.write_str("Hyper"),
                KeyCode::Turbo => f.write_str("Turbo"),
                KeyCode::Abort => f.write_str("Abort"),
                KeyCode::Resume => f.write_str("Resume"),
                KeyCode::Suspend => f.write_str("Suspend"),
                KeyCode::Again => f.write_str("Again"),
                KeyCode::Copy => f.write_str("Copy"),
                KeyCode::Cut => f.write_str("Cut"),
                KeyCode::Find => f.write_str("Find"),
                KeyCode::Open => f.write_str("Open"),
                KeyCode::Paste => f.write_str("Paste"),
                KeyCode::Props => f.write_str("Props"),
                KeyCode::Select => f.write_str("Select"),
                KeyCode::Undo => f.write_str("Undo"),
                KeyCode::Hiragana => f.write_str("Hiragana"),
                KeyCode::Katakana => f.write_str("Katakana"),
                KeyCode::F1 => f.write_str("F1"),
                KeyCode::F2 => f.write_str("F2"),
                KeyCode::F3 => f.write_str("F3"),
                KeyCode::F4 => f.write_str("F4"),
                KeyCode::F5 => f.write_str("F5"),
                KeyCode::F6 => f.write_str("F6"),
                KeyCode::F7 => f.write_str("F7"),
                KeyCode::F8 => f.write_str("F8"),
                KeyCode::F9 => f.write_str("F9"),
                KeyCode::F10 => f.write_str("F10"),
                KeyCode::F11 => f.write_str("F11"),
                KeyCode::F12 => f.write_str("F12"),
                KeyCode::F13 => f.write_str("F13"),
                KeyCode::F14 => f.write_str("F14"),
                KeyCode::F15 => f.write_str("F15"),
                KeyCode::F16 => f.write_str("F16"),
                KeyCode::F17 => f.write_str("F17"),
                KeyCode::F18 => f.write_str("F18"),
                KeyCode::F19 => f.write_str("F19"),
                KeyCode::F20 => f.write_str("F20"),
                KeyCode::F21 => f.write_str("F21"),
                KeyCode::F22 => f.write_str("F22"),
                KeyCode::F23 => f.write_str("F23"),
                KeyCode::F24 => f.write_str("F24"),
                KeyCode::F25 => f.write_str("F25"),
                KeyCode::F26 => f.write_str("F26"),
                KeyCode::F27 => f.write_str("F27"),
                KeyCode::F28 => f.write_str("F28"),
                KeyCode::F29 => f.write_str("F29"),
                KeyCode::F30 => f.write_str("F30"),
                KeyCode::F31 => f.write_str("F31"),
                KeyCode::F32 => f.write_str("F32"),
                KeyCode::F33 => f.write_str("F33"),
                KeyCode::F34 => f.write_str("F34"),
                KeyCode::F35 => f.write_str("F35"),
                _ => f.write_str("Unidentified"),
            },
            Self::Pointer(B::Auxiliary) => f.write_str("MouseMiddle"),
            Self::Pointer(B::X2) => f.write_str("MouseForward"),
            Self::Pointer(B::X1) => f.write_str("MouseBackward"),
            Self::Pointer(_) => f.write_str("MouseUnimplemented"),
        }
    }
}

impl FromStr for KeyInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        KeyInput::keyboard_from_str(&s)
            .map(|key| KeyInput::Keyboard(key.0, key.1))
            .or_else(|| KeyInput::mouse_from_str(&s).map(KeyInput::Pointer))
            .ok_or(())
    }
}

impl Hash for KeyInput {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Keyboard(_key, key_code) => key_code.hash(state),
            // TODO: Implement `Hash` for `druid::MouseButton`
            Self::Pointer(btn) => (*btn as u8).hash(state),
        }
    }
}

impl PartialEq for KeyInput {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                KeyInput::Keyboard(_key_a, key_code_a),
                KeyInput::Keyboard(_key_b, key_code_b),
            ) => key_code_a.eq(key_code_b),
            (KeyInput::Pointer(a), KeyInput::Pointer(b)) => a.eq(b),
            _ => false,
        }
    }
}
