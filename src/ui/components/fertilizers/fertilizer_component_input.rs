use crate::model::fertilizers::LabelComponent;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerComponentInputProps {
    component: LabelComponent,
    on_update: EventHandler<LabelComponent>,
}

#[component]
pub fn FertilizerComponentInput(props: FertilizerComponentInputProps) -> Element {
    rsx! {
        NutrientValue {
            symbol: props.component.symbol(),
            value: Signal::new(props.component.value()),
            on_change: move |value| props.on_update.call(props.component.new(value)),
        }
    }
}
