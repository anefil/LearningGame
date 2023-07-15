pub mod colors {
    pub mod button {
        use bevy::prelude::Color;

        pub const DEFAULT_BG: Color = Color::rgb(0.6, 0.6, 0.6);
        pub const DEFAULT_BG_ACTIVE: Color = Color::rgb(0.6, 0.5, 0.5);
        pub const DEFAULT_BG_HOVER: Color = Color::rgb(0.8, 0.7, 0.7);
        pub const DEFAULT_BORDER: Color = Color::rgb(0.8, 0.2, 0.2);
    }
}