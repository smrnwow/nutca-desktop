use super::SolutionCompositionNutrient;
use crate::controller::solutions::EditMode;
use crate::model::chemistry::Nutrient;
use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::Text;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionCompositionProps {
    solution: Memo<Solution>,
    edit_mode: Signal<EditMode>,
}

#[component]
pub fn SolutionComposition(props: SolutionCompositionProps) -> Element {
    let solution = props.solution.read();

    let badge = props.edit_mode.read().is_automatic() && !props.solution.read().is_empty();

    rsx! {
        Column {
            gap: "medium",

            Column {
                gap: "x-small",

                Text {
                    size: "x-small",
                    "Макроэлементы",
                }

                Row {
                    gap: "small",

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Nitrogen),
                        nutrient_result: solution.nutrient_diff(Nutrient::Nitrogen),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Phosphorus),
                        nutrient_result: solution.nutrient_diff(Nutrient::Phosphorus),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Potassium),
                        nutrient_result: solution.nutrient_diff(Nutrient::Potassium),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Calcium),
                        nutrient_result: solution.nutrient_diff(Nutrient::Calcium),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Magnesium),
                        nutrient_result: solution.nutrient_diff(Nutrient::Magnesium),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Sulfur),
                        nutrient_result: solution.nutrient_diff(Nutrient::Sulfur),
                    }
                }
            }

            Column {
                gap: "x-small",

                Text {
                    size: "x-small",
                    "Формы азота"
                }

                Row {
                    gap: "small",

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::NitrogenNitrate),
                        nutrient_result: solution.nutrient_diff(Nutrient::NitrogenNitrate),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::NitrogenAmmonium),
                        nutrient_result: solution.nutrient_diff(Nutrient::NitrogenAmmonium),
                    }
                }
            }

            Column {
                gap: "x-small",

                Text {
                    size: "x-small",
                    "Микроэлементы",
                }

                Row {
                    gap: "small",

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Iron),
                        nutrient_result: solution.nutrient_diff(Nutrient::Iron),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Manganese),
                        nutrient_result: solution.nutrient_diff(Nutrient::Manganese),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Copper),
                        nutrient_result: solution.nutrient_diff(Nutrient::Copper),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Zinc),
                        nutrient_result: solution.nutrient_diff(Nutrient::Zinc),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Boron),
                        nutrient_result: solution.nutrient_diff(Nutrient::Boron),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: solution.nutrient_value(Nutrient::Molybdenum),
                        nutrient_result: solution.nutrient_diff(Nutrient::Molybdenum),
                    }
                }
            }
        }
    }
}
