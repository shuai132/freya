use dioxus_native_core::{
    exports::shipyard::Component,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;

use crate::CustomAttributeValues;

#[derive(Default, Clone, Debug, Component, PartialEq)]
pub struct Transform {
    pub rotate_degs: Option<f32>,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for Transform {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&["rotate"]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let mut rotate_degs = None;

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "rotate" => {
                        if let Some(value) = attr.value.as_text() {
                            if value.ends_with("deg") {
                                if let Ok(degs) = value.replacen("deg", "", 1).parse::<f32>() {
                                    rotate_degs = Some(degs)
                                }
                            }
                        }
                    }
                    _ => {
                        panic!("Unsupported attribute <{}>, this should not be happening, please report it.", attr.attribute.name);
                    }
                }
            }
        }

        let changed = rotate_degs != self.rotate_degs;
        *self = Self { rotate_degs };
        changed
    }
}
