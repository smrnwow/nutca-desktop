use crate::storage::ProfilesStorage;
use crate::ui::components::profiles::ProfileListingItem;
use crate::ui::components::utils::{Block, Button, Card, Divider, Table, TableCell, Title};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn ProfilesListingPage() -> Element {
    let profiles_storage = consume_context::<Signal<ProfilesStorage>>();

    let profiles = use_memo(move || profiles_storage.read().list());

    rsx! {
        div {
            class: "profiles-listing-page",

            section {
                class: "profiles-listing-page__listing",

                Card {
                    Block {
                        Title {
                            text: "Список профилей питания",
                        }
                    }

                    Divider {}

                    Block {
                        div {
                            class: "profiles-listing-page__controls",

                            Button {
                                style: "primary",
                                on_click: move |_| {
                                    navigator().push(Route::ProfileAddPage {});
                                },

                                "Добавить профиль питания",
                            }
                        }
                    }

                    Block {
                        exclude_padding: "top",

                        Table {
                            header: rsx! {
                                TableCell {
                                    width: "100%",
                                    "Название",
                                }

                                TableCell {
                                    width: "1%",
                                }
                            },
                            body: rsx! {
                                for profile in profiles.read().clone() {
                                    ProfileListingItem {
                                        profile,
                                        on_open: move |profile_id| {
                                            navigator().push(Route::ProfileEditPage {
                                                profile_id,
                                            });
                                        },
                                        on_use: move |profile_id| {
                                            navigator().push(Route::SolutionAddPage {
                                                profile_id,
                                            });
                                        },
                                        on_delete: move |profile_id| {
                                            println!("delete profile {}", profile_id);
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
