// Modern, minimalistic & standard-compliant cold wallet library.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2020-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2020-2023 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2020-2023 Dr Maxim Orlovsky. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::BTreeSet;
use std::iter;

use derive::{
    CompressedPk, Derive, DeriveXOnly, DerivedScript, InternalPk, KeyOrigin, Keychain, NormalIndex,
    TapDerivation, Terminal, XOnlyPk, XpubDerivable, XpubSpec,
};
use indexmap::IndexMap;

use crate::{Descriptor, SpkClass};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate",))]
#[derive(Clone, Eq, PartialEq, Hash, Debug, From)]
pub struct TrKey<K: DeriveXOnly = XpubDerivable>(K);

impl<K: DeriveXOnly> TrKey<K> {
    pub fn as_internal_key(&self) -> &K { &self.0 }
    pub fn into_internal_key(self) -> K { self.0 }
}

impl<K: DeriveXOnly> Derive<DerivedScript> for TrKey<K> {
    #[inline]
    fn default_keychain(&self) -> Keychain { self.0.default_keychain() }

    #[inline]
    fn keychains(&self) -> BTreeSet<Keychain> { self.0.keychains() }

    fn derive(
        &self,
        keychain: impl Into<Keychain>,
        index: impl Into<NormalIndex>,
    ) -> DerivedScript {
        let internal_key = self.0.derive(keychain, index);
        DerivedScript::TaprootKeyOnly(InternalPk::from_unchecked(internal_key))
    }
}

impl<K: DeriveXOnly> Descriptor<K> for TrKey<K> {
    type KeyIter<'k> = iter::Once<&'k K> where Self: 'k, K: 'k;
    type VarIter<'v> = iter::Empty<&'v ()> where Self: 'v, (): 'v;
    type XpubIter<'x> = iter::Once<&'x XpubSpec> where Self: 'x;

    fn class(&self) -> SpkClass { SpkClass::P2tr }

    fn keys(&self) -> Self::KeyIter<'_> { iter::once(&self.0) }
    fn vars(&self) -> Self::VarIter<'_> { iter::empty() }
    fn xpubs(&self) -> Self::XpubIter<'_> { iter::once(self.0.xpub_spec()) }

    fn compr_keyset(&self, _terminal: Terminal) -> IndexMap<CompressedPk, KeyOrigin> {
        IndexMap::new()
    }

    fn xonly_keyset(&self, terminal: Terminal) -> IndexMap<XOnlyPk, TapDerivation> {
        let mut map = IndexMap::with_capacity(1);
        let key = self.0.derive(terminal.keychain, terminal.index);
        map.insert(
            key,
            TapDerivation::with_internal_pk(self.0.xpub_spec().origin().clone(), terminal),
        );
        map
    }
}

/*
pub struct TrScript<K: DeriveXOnly> {
    internal_key: K,
    tap_tree: TapTree<Policy<K>>,
}
*/
