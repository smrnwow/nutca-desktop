use crate::controller::solutions::SolutionEditor;
use crate::controller::Toaster;
use crate::repository::Storage;
use crate::ui::components::solutions::SolutionEditor;
use dioxus::prelude::*;

#[component]
pub fn SolutionAddPage(profile_id: String) -> Element {
    let mut toaster = consume_context::<Signal<Toaster>>();

    let storage = consume_context::<Signal<Storage>>();

    let mut solution_editor = use_signal(|| SolutionEditor::new(storage, profile_id));

    let nutrition_program_browser =
        use_memo(move || solution_editor.read().nutrition_program_browser().clone());

    let fertilizers_picker = use_memo(move || solution_editor.read().fertilizers_picker().clone());

    let solution = use_memo(move || solution_editor.read().solution());

    let profile = use_memo(move || solution.read().profile());

    let validation = use_memo(move || solution_editor.read().validation());

    use_effect(move || toaster.write().render(validation.read().list()));

    rsx! {
        SolutionEditor {
            nutrition_program_browser,
            fertilizers_picker,
            solution,
            validation,
            profile,
            edit_mode: solution_editor.read().edit_mode(),
            on_profile_change: move |profile_id| {
                solution_editor.write().change_profile(profile_id);
            },
            on_fertilizer_select: move |fertilizer_id| {
                solution_editor.write().select_fertilizer(fertilizer_id);
            },
            on_fertilizer_exclude: move |fertilizer_id| {
                solution_editor.write().exclude_fertilizer(fertilizer_id);
            },
            on_fertilizer_amount_update: move |(fertilizer_id, amount)| {
                solution_editor.write().update_fertilizer_amount(fertilizer_id, amount);
            },
            on_name_update: move |name| {
                solution_editor.write().update_name(name);
            },
            on_profile_nutrient_update: move |nutrient_requirement| {
                solution_editor.write().update_nutrient_requirement(nutrient_requirement);
            },
            on_volume_update: move |volume| {
                solution_editor.write().update_volume(volume);
            },
            on_profile_search: move |search_query| {
                solution_editor.write().search_nutrient_program(search_query);
            },
            on_fertilizer_search: move |search_query| {
                solution_editor.write().search_fertilizer(search_query);
            },
            on_fertilizers_paginate: move |page_index| {
                solution_editor.write().paginate_fertilizers_browser(page_index);
            },
            on_selected_set_paginate: move |page_index| {
                solution_editor.write().paginate_selected_set(page_index);
            },
            on_save: move |_| {
                solution_editor.write().create();
            },
        }
    }
}
