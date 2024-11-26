// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

struct Symbol(String);

struct Role(Symbol);

enum Concept {
    Atomic(Symbol),
    Conjunction(Box<Concept>, Box<Concept>),
    Disjunction(Box<Concept>, Box<Concept>),
    Negation(Box<Concept>),
    Subsumption(Box<Concept>, Box<Concept>),
    Equivalence(Box<Concept>, Box<Concept>),
    ExistentialRestriction(Box<Concept>, Role),
    ValueRestriction(Box<Concept>, Role)
}

struct RoleInstance(Role, Concept, Concept);
