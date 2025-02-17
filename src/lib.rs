//! Sancta is the plural of the Latin _sanctum_, meaning a holy place.
//!
//! It is a library for simulating religious dynamics and spiritual
//! phenomena.
//!
//! The crate is inspired in part by the work of Terry Davis on TempleOS,
//! creating a project in code specifically pursued as a work of religious
//! devotion. The work in not carried out with a particular tradition as a
//! guidepost. Perhaps it will be useful in the development of new systems of
//! religious practice or in the synthesis or revival of old forms.

#[allow(dead_code)] // haha
/// Lack, emptiness, formlessness. Across traditions, the role and general
/// valence attached to emptiness varies widely. Whether viewed as the realm of
/// chaotic evil, or the generative space that births all existence, void is a
/// central and foundational concept to human transcental history.
struct Void {}

#[allow(dead_code)]
/// A group (in whole or part) joined in common or collaborative practice
struct Assembly {
    people: Vec<Person>,
    practices: Vec<Practice>,
}

#[allow(dead_code)]
/// A human being
struct Person {
    name: String,
    practices: Vec<Practice>,
}

#[allow(dead_code)]
struct Practice {}
