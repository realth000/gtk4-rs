// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ConstraintAttribute, ConstraintRelation, ConstraintTarget};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GtkConstraint")]
    pub struct Constraint(Object<ffi::GtkConstraint, ffi::GtkConstraintClass>);

    match fn {
        type_ => || ffi::gtk_constraint_get_type(),
    }
}

impl Constraint {
    #[doc(alias = "gtk_constraint_new")]
    pub fn new(
        target: Option<&impl IsA<ConstraintTarget>>,
        target_attribute: ConstraintAttribute,
        relation: ConstraintRelation,
        source: Option<&impl IsA<ConstraintTarget>>,
        source_attribute: ConstraintAttribute,
        multiplier: f64,
        constant: f64,
        strength: i32,
    ) -> Constraint {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_constraint_new(
                target.map(|p| p.as_ref()).to_glib_none().0,
                target_attribute.into_glib(),
                relation.into_glib(),
                source.map(|p| p.as_ref()).to_glib_none().0,
                source_attribute.into_glib(),
                multiplier,
                constant,
                strength,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_new_constant")]
    pub fn new_constant(
        target: Option<&impl IsA<ConstraintTarget>>,
        target_attribute: ConstraintAttribute,
        relation: ConstraintRelation,
        constant: f64,
        strength: i32,
    ) -> Constraint {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_constraint_new_constant(
                target.map(|p| p.as_ref()).to_glib_none().0,
                target_attribute.into_glib(),
                relation.into_glib(),
                constant,
                strength,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Constraint`] objects.
    ///
    /// This method returns an instance of [`ConstraintBuilder`](crate::builders::ConstraintBuilder) which can be used to create [`Constraint`] objects.
    pub fn builder() -> ConstraintBuilder {
        ConstraintBuilder::new()
    }

    #[doc(alias = "gtk_constraint_get_constant")]
    #[doc(alias = "get_constant")]
    pub fn constant(&self) -> f64 {
        unsafe { ffi::gtk_constraint_get_constant(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_constraint_get_multiplier")]
    #[doc(alias = "get_multiplier")]
    pub fn multiplier(&self) -> f64 {
        unsafe { ffi::gtk_constraint_get_multiplier(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_constraint_get_relation")]
    #[doc(alias = "get_relation")]
    pub fn relation(&self) -> ConstraintRelation {
        unsafe { from_glib(ffi::gtk_constraint_get_relation(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_get_source")]
    #[doc(alias = "get_source")]
    pub fn source(&self) -> Option<ConstraintTarget> {
        unsafe { from_glib_none(ffi::gtk_constraint_get_source(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_get_source_attribute")]
    #[doc(alias = "get_source_attribute")]
    #[doc(alias = "source-attribute")]
    pub fn source_attribute(&self) -> ConstraintAttribute {
        unsafe {
            from_glib(ffi::gtk_constraint_get_source_attribute(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_get_strength")]
    #[doc(alias = "get_strength")]
    pub fn strength(&self) -> i32 {
        unsafe { ffi::gtk_constraint_get_strength(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_constraint_get_target")]
    #[doc(alias = "get_target")]
    pub fn target(&self) -> Option<ConstraintTarget> {
        unsafe { from_glib_none(ffi::gtk_constraint_get_target(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_get_target_attribute")]
    #[doc(alias = "get_target_attribute")]
    #[doc(alias = "target-attribute")]
    pub fn target_attribute(&self) -> ConstraintAttribute {
        unsafe {
            from_glib(ffi::gtk_constraint_get_target_attribute(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_is_attached")]
    pub fn is_attached(&self) -> bool {
        unsafe { from_glib(ffi::gtk_constraint_is_attached(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_is_constant")]
    pub fn is_constant(&self) -> bool {
        unsafe { from_glib(ffi::gtk_constraint_is_constant(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_is_required")]
    pub fn is_required(&self) -> bool {
        unsafe { from_glib(ffi::gtk_constraint_is_required(self.to_glib_none().0)) }
    }
}

impl Default for Constraint {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Constraint`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ConstraintBuilder {
    builder: glib::object::ObjectBuilder<'static, Constraint>,
}

impl ConstraintBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn constant(self, constant: f64) -> Self {
        Self {
            builder: self.builder.property("constant", constant),
        }
    }

    pub fn multiplier(self, multiplier: f64) -> Self {
        Self {
            builder: self.builder.property("multiplier", multiplier),
        }
    }

    pub fn relation(self, relation: ConstraintRelation) -> Self {
        Self {
            builder: self.builder.property("relation", relation),
        }
    }

    pub fn source(self, source: &impl IsA<ConstraintTarget>) -> Self {
        Self {
            builder: self.builder.property("source", source.clone().upcast()),
        }
    }

    pub fn source_attribute(self, source_attribute: ConstraintAttribute) -> Self {
        Self {
            builder: self.builder.property("source-attribute", source_attribute),
        }
    }

    pub fn strength(self, strength: i32) -> Self {
        Self {
            builder: self.builder.property("strength", strength),
        }
    }

    pub fn target(self, target: &impl IsA<ConstraintTarget>) -> Self {
        Self {
            builder: self.builder.property("target", target.clone().upcast()),
        }
    }

    pub fn target_attribute(self, target_attribute: ConstraintAttribute) -> Self {
        Self {
            builder: self.builder.property("target-attribute", target_attribute),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Constraint`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Constraint {
        self.builder.build()
    }
}
