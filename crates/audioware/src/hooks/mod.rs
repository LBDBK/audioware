use red4ext_rs::SdkEnv;

mod audio;
mod audio_system;
mod sound_component;
mod time_dilatable;
mod time_system;

#[cfg(debug_assertions)]
mod entity;
mod ink_menu_scenario;
#[cfg(debug_assertions)]
mod save_handling_controller;

#[cfg(feature = "research")]
mod events;
#[cfg(feature = "research")]
mod ink_logic_controller;
#[cfg(feature = "research")]
mod localization_manager;
#[cfg(feature = "research")]
mod onscreen_vo;
#[cfg(feature = "research")]
mod script_audio_player;
#[cfg(feature = "research")]
mod vo;

pub fn attach(env: &SdkEnv) {
    sound_component::attach_hook(env);
    audio::attach_hook(env);
    audio_system::attach_hooks(env);
    time_dilatable::attach_hooks(env);
    time_system::attach_hooks(env);
    ink_menu_scenario::attach_hooks(env);

    #[cfg(feature = "research")]
    {
        onscreen_vo::attach_hook(env);
        localization_manager::attach_hook(env);
        ink_logic_controller::attach_hook(env);
        script_audio_player::attach_hooks(env);
    }

    #[cfg(debug_assertions)]
    save_handling_controller::attach_hook(env);
    #[cfg(debug_assertions)]
    entity::attach_hook(env);

    #[cfg(feature = "research")]
    {
        // events::audio::attach_hook(env); // 🌊
        events::vehicle_audio::attach_hook(env);
        // events::dialog_line_end::attach_hook(env);
        // events::dialog_line::attach_hook(env);
        events::weapon::attach_hook(env);
        events::trigger::attach_hooks(env);
        events::ink::attach_hook(env);
        events::ent::attach_hook(env);
    }
}

#[rustfmt::skip]
#[doc(hidden)]
mod offsets {
    pub const AUDIO_PLAY_DIALOG_LINE: u32                       = 0x28F53A76;   // 0x1405FC310 (2.3)
    pub const AUDIOSYSTEM_PLAY: u32                             = 0xCDB11D0E;   // 0x140974F58 (2.12a)
    pub const AUDIOSYSTEM_STOP: u32                             = 0xD2781D1E;   // 0x1424503F8 (2.12a)
    pub const AUDIOSYSTEM_SWITCH: u32                           = 0x15081DEA;   // 0x140291688 (2.12a)
    #[cfg(debug_assertions)]
    pub const ENTITY_DISPOSE: u32                               = 0x3221A80;    // 0x14232C744 (2.13)
    pub const TIMEDILATABLE_SETINDIVIDUALTIMEDILATION: u32      = 0x80102488;   // 0x1423AF554 (2.13)
    pub const TIMEDILATABLE_UNSETINDIVIDUALTIMEDILATION: u32    = 0xDA20256B;   // 0x14147B424 (2.13)
    pub const TIMESYSTEM_SETTIMEDILATION: u32                   = 0xA1DC1F92;   // 0x140A46EE4 (2.13)
    pub const TIMESYSTEM_UNSETTIMEDILATION: u32                 = 0xF0652075;   // 0x1409BAD34 (2.13)
    // gameuiSaveHandlingController
    // note: LoadSaveInGame and LoadModdedSave share same underlying address
    #[cfg(debug_assertions)]
    pub const SAVEHANDLINGCONTROLLER_LOAD_SAVE_IN_GAME: u32     = 0x9AB824D9;   // 0x14083FB6C (2.13)
    pub const INKMENUSCENARIO_SWITCH_TO_SCENARIO: u32           = 0xE9B92059;   // 0x1409CF068 (2.3)
    pub const INKMENUSCENARIO_QUEUE_EVENT: u32                  = 0x56A9218A;   // 0x14130F6B8 (2.3)
    pub const SOUNDCOMPONENT_ONSTOPDIALOGLINE: u32              = 0xD4F11D73;   // 0x1405FCB28 (2.3)
    
    #[cfg(feature = "research")]
    mod natives {
        pub const INKLOGICCONTROLLER_QUEUE_EVENT: u32               = 0xC87F2007;   // 0x1408663B0 (2.3)
        pub const VO_STORAGE_GET_VO_FILE: u32                       = 0x899C28D0;   // 0x140A93F84 (2.31)
        pub const LOCALIZATIONMANAGER_RESOLVEFILENAME: u32          = 0x8D2C2B6E;   // 0x142045B38 (2.3)
        pub const ONSCREENVOPLAYERCONTROLLER_SHOWSUBTITLE: u32      = 0xFE3C1D52;   // 0x1404F42A0 (2.3)
        pub const SCRIPTAUDIOPLAYER_PLAY_SINGLE: u32                = 0x90251060;   // 0x1406944E0 (2.3)
        pub const SCRIPTAUDIOPLAYER_PLAY_THREE: u32                 = 0x8B616DE;    // 0x1421A25DC (2.3)
        pub const SCRIPTAUDIOPLAYER_PLAY_UNIQUE_WITH_SEEK: u32      = 0xD02C1648;   // 0x1421A2644 (2.3)
        pub const SCRIPTAUDIOPLAYER_STOP: u32                       = 0x92EC1070;   // 0x1421A272C (2.3)
        pub const SCRIPTAUDIOPLAYER_SET_SWITCH: u32                 = 0x50D1298;    // 0x1421A26F4 (2.3)
        pub const SCRIPTAUDIOPLAYER_SET_PARAMETER: u32              = 0x447413E4;   // 0x1421A26BC (2.3)
    }
    #[cfg(feature = "research")]
    pub use natives::*;

    #[cfg(feature = "research")]
    mod events {
        pub const EVENT_DIALOGLINE: u32                             = 0x10E71E89;   // 0x1409C12A8 (2.12a)
        pub const EVENT_DIALOGLINEEND: u32                          = 0x6F24331;    // 0x141188BF4 (2.12a)
        pub const VEHICLE_AUDIO_EVENT: u32                          = 0x69EF1461;   // 0x1418D4C44 (2.13)
        pub const AUDIO_EVENT: u32                                  = 0x10C412FD;   // 0x14065816C (2.13)
        pub const WEAPON_PRE_FIRE_EVENT: u32                        = 0x7BC51906;   // 0x140652AB4 (2.13)
        // note: gameaudioeventsStopWeaponFire and gameweaponeventsStopFiringEvent share same underlying address
        pub const WEAPON_STOP_FIRING_EVENT: u32                     = 0xA83C1996;   // 0x140652AF8 (2.13)
        pub const AREA_ENTERED_EVENT: u32                           = 0x252517CB;   // 0x142863744 (2.21)
        pub const AREA_EXITED_EVENT: u32                            = 0xF3E11703;   // 0x142863818 (2.21)
        pub const INK_VO_REQUEST_EVT: u32                           = 0xBDB51D56;   // 0x1405FCBC4 (2.3)
        pub const SOUND_PLAY_VO: u32                                = 0x7ED1B0B;   // 0x1405327B8 (2.3)
    }
    #[cfg(feature = "research")]
    pub use events::*;
}

#[macro_export]
macro_rules! attach_native_func {
    ($name:literal, $offset:path, $hook: ident, $me:ident, $to:ident $(, $v:vis)?) => {
        ::red4ext_rs::hooks! {
            static $hook: fn(
                i: *mut ::red4ext_rs::types::IScriptable,
                f: *mut ::red4ext_rs::types::StackFrame,
                a3: ::red4ext_rs::VoidPtr,
                a4: ::red4ext_rs::VoidPtr) -> ();
        }

        #[allow(clippy::missing_transmute_annotations)]
        $($v)? fn $me(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve($offset);
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook($hook, addr, $to) };
            $crate::utils::intercept!("attached native func hook for {}", $name);
        }
    };
    ($name:literal, $offset:path) => {
        attach_native_func!($name, $offset, HOOK, attach_hook, detour, pub);
    };
}

#[macro_export]
macro_rules! attach_native_event {
    ($offset:path, $class:path, $to:ident $(, $v:vis)?) => {
        ::red4ext_rs::hooks! {
            static HOOK: fn(
                a1: *mut ::red4ext_rs::types::IScriptable,
                a2: *mut $class) -> ();
        }

        #[allow(clippy::missing_transmute_annotations)]
        $($v)? fn attach_hook(env: &::red4ext_rs::SdkEnv) {
            let addr = ::red4ext_rs::addr_hashes::resolve($offset);
            let addr = unsafe { ::std::mem::transmute(addr) };
            unsafe { env.attach_hook(HOOK, addr, $to) };
            $crate::utils::intercept!("attached native event hook for {}", <$class as ::red4ext_rs::ScriptClass>::NAME);
        }
    };
    ($offset:path, $class:path) => {
        attach_native_event!($offset, $class, detour, pub);
    };
}
