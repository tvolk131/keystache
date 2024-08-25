use iced::widget::{row, text_input, Column, Text};
use nostr_sdk::secp256k1::{rand::thread_rng, Keypair};

use crate::{
    ui_components::{icon_button, PaletteColor, SvgIcon},
    util::truncate_text,
    ConnectedState, KeystacheMessage,
};

use super::{container, RouteName};

#[derive(Clone)]
pub struct Page {
    pub connected_state: ConnectedState,
    pub subroute: Subroute,
}

impl Page {
    pub fn view<'a>(&self) -> Column<'a, KeystacheMessage> {
        match &self.subroute {
            Subroute::List(list) => list.view(&self.connected_state),
            Subroute::Add(add) => add.view(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubrouteName {
    List,
    Add,
}

#[derive(Clone)]
pub enum Subroute {
    List(List),
    Add(Add),
}

impl Subroute {
    pub fn to_name(&self) -> SubrouteName {
        match self {
            Self::List(_) => SubrouteName::List,
            Self::Add(_) => SubrouteName::Add,
        }
    }
}

#[derive(Clone)]
pub struct List {}

impl List {
    // TODO: Remove this clippy allow.
    #[allow(clippy::unused_self)]
    fn view<'a>(&self, connected_state: &ConnectedState) -> Column<'a, KeystacheMessage> {
        // TODO: Add pagination.
        let Ok(public_keys) = connected_state.db.list_public_keys(999, 0) else {
            return container("Keys").push("Failed to load keys");
        };

        let mut container = container("Keys");

        for public_key in public_keys {
            container = container.push(row![
                Text::new(truncate_text(&public_key, 12, true))
                    .size(20)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                icon_button("Delete", SvgIcon::Delete, PaletteColor::Danger)
                    .on_press(KeystacheMessage::DeleteKeypair { public_key }),
            ]);
        }

        container = container.push(
            icon_button("Add Keypair", SvgIcon::Add, PaletteColor::Primary).on_press(
                KeystacheMessage::Navigate(RouteName::NostrKeypairs(SubrouteName::Add)),
            ),
        );

        container
    }
}

#[derive(Clone)]
pub struct Add {
    pub nsec: String,
    pub keypair_or: Option<Keypair>, // Parsed from nsec on any update. `Some` if nsec is valid, `None` otherwise.
}

impl Add {
    fn view<'a>(&self) -> Column<'a, KeystacheMessage> {
        container("Add Keypair")
            .push(
                text_input("nSec", &self.nsec)
                    .on_input(KeystacheMessage::SaveKeypairNsecInputChanged)
                    .padding(10)
                    .size(30),
            )
            .push(
                icon_button("Save", SvgIcon::Save, PaletteColor::Primary)
                    .on_press_maybe(self.keypair_or.map(KeystacheMessage::SaveKeypair)),
            )
            .push(
                icon_button(
                    "Generate New Keypair",
                    SvgIcon::Casino,
                    PaletteColor::Primary,
                )
                .on_press(KeystacheMessage::SaveKeypair(Keypair::new_global(
                    &mut thread_rng(),
                ))),
            )
            .push(
                icon_button("Back", SvgIcon::ArrowBack, PaletteColor::Background).on_press(
                    KeystacheMessage::Navigate(RouteName::NostrKeypairs(SubrouteName::List)),
                ),
            )
    }
}
