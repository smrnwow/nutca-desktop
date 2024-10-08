use super::{FertilizersBrowser, FertilizersSet, SolutionProfile};
use crate::controller::solutions::{
    FertilizersPicker, FertilizersUsed, ProfilesBrowser, SolutionValidator,
};
use crate::model::chemistry::{NutrientAmount, Volume};
use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{Block, Button, Divider, TextField, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionEditorProps {
    solution: Memo<Solution>,
    validator: Memo<SolutionValidator>,
    profiles_browser: Memo<ProfilesBrowser>,
    fertilizers_picker: Memo<FertilizersPicker>,
    fertilizers_used: Memo<FertilizersUsed>,
    on_name_update: EventHandler<String>,
    on_volume_update: EventHandler<Volume>,
    on_profile_change: EventHandler<String>,
    on_profile_stage_change: EventHandler<String>,
    on_profile_search: EventHandler<String>,
    on_profile_nutrient_update: EventHandler<NutrientAmount>,
    on_fertilizer_select: EventHandler<String>,
    on_fertilizer_exclude: EventHandler<String>,
    on_fertilizer_amount_update: EventHandler<(String, f64)>,
    on_fertilizer_search: EventHandler<String>,
    on_fertilizers_paginate: EventHandler<usize>,
    on_selected_set_paginate: EventHandler<usize>,
    on_save: EventHandler<()>,
}

#[component]
pub fn SolutionEditor(props: SolutionEditorProps) -> Element {
    rsx! {
        Block {
            Row {
                Title {
                    "Редактор питательного раствора",
                }
            }
        }

        Block {
            exclude_padding: "top",

            Column {
                Divider {}

                TextField {
                    label: "Название",
                    value: props.solution.read().name(),
                    error: props.validator.read().name(),
                    on_input: props.on_name_update,
                }

                Divider {}

                SolutionProfile {
                    solution: props.solution,
                    profiles_browser: props.profiles_browser,
                    on_profile_change: props.on_profile_change,
                    on_profile_stage_change: props.on_profile_stage_change,
                    on_profile_search: props.on_profile_search,
                    on_profile_nutrient_update: props.on_profile_nutrient_update,
                }

                Divider {}

                Row {
                    FertilizersBrowser {
                        fertilizers_picker: props.fertilizers_picker,
                        on_select: props.on_fertilizer_select,
                        on_search: props.on_fertilizer_search,
                        on_paginate: props.on_fertilizers_paginate,
                    }

                    FertilizersSet {
                        solution: props.solution,
                        fertilizers_used: props.fertilizers_used,
                        on_fertilizer_exclude: props.on_fertilizer_exclude,
                        on_fertilizer_amount_update: props.on_fertilizer_amount_update,
                        on_paginate: props.on_selected_set_paginate,
                        on_volume_update: props.on_volume_update,
                    }
                }

                Divider {}

                Row {
                    horizontal: "end",

                    Button {
                        style: "primary",
                        on_click: props.on_save,
                        "Сохранить",
                    }
                }
            }
        }
    }
}
