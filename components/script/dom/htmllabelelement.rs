/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::activation::{synthetic_click_activation, Activatable, ActivationSource};
use crate::dom::attr::Attr;
use crate::dom::bindings::codegen::Bindings::HTMLLabelElementBinding;
use crate::dom::bindings::codegen::Bindings::HTMLLabelElementBinding::HTMLLabelElementMethods;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::document::Document;
use crate::dom::element::{AttributeMutation, Element};
use crate::dom::event::Event;
use crate::dom::eventtarget::EventTarget;
use crate::dom::htmlelement::HTMLElement;
use crate::dom::htmlformelement::{FormControl, FormControlElementHelpers, HTMLFormElement};
use crate::dom::node::{document_from_node, Node, ShadowIncluding};
use crate::dom::virtualmethods::VirtualMethods;
use dom_struct::dom_struct;
use html5ever::{LocalName, Prefix};
use style::attr::AttrValue;

#[dom_struct]
pub struct HTMLLabelElement {
    htmlelement: HTMLElement,
}

impl HTMLLabelElement {
    fn new_inherited(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> HTMLLabelElement {
        HTMLLabelElement {
            htmlelement: HTMLElement::new_inherited(local_name, prefix, document),
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> DomRoot<HTMLLabelElement> {
        Node::reflect_node(
            Box::new(HTMLLabelElement::new_inherited(
                local_name, prefix, document,
            )),
            document,
            HTMLLabelElementBinding::Wrap,
        )
    }
}

impl Activatable for HTMLLabelElement {
    fn as_element(&self) -> &Element {
        self.upcast::<Element>()
    }

    fn is_instance_activatable(&self) -> bool {
        true
    }

    // https://html.spec.whatwg.org/multipage/#run-pre-click-activation-steps
    // https://html.spec.whatwg.org/multipage/#the-button-element:activation-behavior
    fn pre_click_activation(&self) {}

    // https://html.spec.whatwg.org/multipage/#run-canceled-activation-steps
    fn canceled_activation(&self) {}

    // https://html.spec.whatwg.org/multipage/#run-post-click-activation-steps
    fn activation_behavior(&self, _event: &Event, _target: &EventTarget) {
        if let Some(e) = self.GetControl() {
            let elem = e.upcast::<Element>();
            synthetic_click_activation(
                elem,
                false,
                false,
                false,
                false,
                ActivationSource::NotFromClick,
            );
        }
    }
}

impl HTMLLabelElementMethods for HTMLLabelElement {
    // https://html.spec.whatwg.org/multipage/#dom-fae-form
    fn GetForm(&self) -> Option<DomRoot<HTMLFormElement>> {
        self.form_owner()
    }

    // https://html.spec.whatwg.org/multipage/#dom-label-htmlfor
    make_getter!(HtmlFor, "for");

    // https://html.spec.whatwg.org/multipage/#dom-label-htmlfor
    make_atomic_setter!(SetHtmlFor, "for");

    // https://html.spec.whatwg.org/multipage/#dom-label-control
    fn GetControl(&self) -> Option<DomRoot<HTMLElement>> {
        if !self.upcast::<Node>().is_in_doc() {
            return None;
        }

        let for_attr = match self
            .upcast::<Element>()
            .get_attribute(&ns!(), &local_name!("for"))
        {
            Some(for_attr) => for_attr,
            None => return self.first_labelable_descendant(),
        };

        let for_value = for_attr.value();
        document_from_node(self)
            .get_element_by_id(for_value.as_atom())
            .and_then(DomRoot::downcast::<HTMLElement>)
            .into_iter()
            .filter(|e| e.is_labelable_element())
            .next()
    }
}

impl VirtualMethods for HTMLLabelElement {
    fn super_type(&self) -> Option<&dyn VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &dyn VirtualMethods)
    }

    fn parse_plain_attribute(&self, name: &LocalName, value: DOMString) -> AttrValue {
        match name {
            &local_name!("for") => AttrValue::from_atomic(value.into()),
            _ => self
                .super_type()
                .unwrap()
                .parse_plain_attribute(name, value),
        }
    }

    fn attribute_mutated(&self, attr: &Attr, mutation: AttributeMutation) {
        self.super_type().unwrap().attribute_mutated(attr, mutation);
        match attr.local_name() {
            &local_name!("form") => {
                self.form_attribute_mutated(mutation);
            },
            _ => {},
        }
    }
}

impl HTMLLabelElement {
    pub fn first_labelable_descendant(&self) -> Option<DomRoot<HTMLElement>> {
        self.upcast::<Node>()
            .traverse_preorder(ShadowIncluding::No)
            .filter_map(DomRoot::downcast::<HTMLElement>)
            .filter(|elem| elem.is_labelable_element())
            .next()
    }
}

impl FormControl for HTMLLabelElement {
    fn form_owner(&self) -> Option<DomRoot<HTMLFormElement>> {
        self.GetControl()
            .map(DomRoot::upcast::<Element>)
            .and_then(|elem| {
                elem.as_maybe_form_control()
                    .and_then(|control| control.form_owner())
            })
    }

    fn set_form_owner(&self, _: Option<&HTMLFormElement>) {
        // Label is a special case for form owner, it reflects its control's
        // form owner. Therefore it doesn't hold form owner itself.
    }

    fn to_element<'a>(&'a self) -> &'a Element {
        self.upcast::<Element>()
    }
}
