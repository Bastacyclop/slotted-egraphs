use crate::*;

// syntactic add:
impl<L: Language> EGraph<L> {
    pub fn add_syn_expr(&mut self, re: RecExpr<L>) -> AppliedId {
        let mut n = re.node;
        let mut refs: Vec<&mut AppliedId> = n.applied_id_occurences_mut();
        if CHECKS {
            assert_eq!(re.children.len(), refs.len());
        }
        for (i, child) in re.children.into_iter().enumerate() {
            *(refs[i]) = self.add_syn_expr(child);
        }
        self.add_syn(n)
    }

    pub fn add_syn(&mut self, enode: L) -> AppliedId {
        self.add(enode.clone());

        if let Some(x) = self.lookup_syn(&enode) {
            return x;
        }

        let old_slots = enode.slots();
        let fresh_to_old = Bijection::bijection_from_fresh_to(&old_slots);
        let old_to_fresh = fresh_to_old.inverse();
        let enode = enode.apply_slotmap(&old_to_fresh);
        let c = self.alloc_eclass(&old_to_fresh.values(), enode.clone());

        let c_a = self.mk_syn_identity_applied_id(c);
        self.handle_congruence(&c_a);

        c_a
    }

    fn lookup_syn(&self, enode: &L) -> Option<AppliedId> {
        let (sh, bij) = enode.weak_shape();
        let i = self.syn_hashcons.get(&sh)?;

        // bij :: SHAPE -> X
        // i :: slots(i.id) -> SHAPE
        let i = i.apply_slotmap(&bij);
        Some(i)
    }
}

// semantic add:
impl<L: Language> EGraph<L> {
    pub fn add_expr(&mut self, re: RecExpr<L>) -> AppliedId {
        let mut n = re.node;
        let mut refs: Vec<&mut AppliedId> = n.applied_id_occurences_mut();
        if CHECKS {
            assert_eq!(re.children.len(), refs.len());
        }
        for (i, child) in re.children.into_iter().enumerate() {
            *(refs[i]) = self.add_expr(child);
        }
        self.add(n)
    }


    pub fn add(&mut self, enode: L) -> AppliedId {
        self.add_internal(self.shape(&enode))
    }

    // self.add(x) = y implies that x.slots() is a superset of y.slots().
    // x.slots() - y.slots() are redundant slots.
    pub fn add_internal(&mut self, t: (L, SlotMap)) -> AppliedId {
        if let Some(x) = self.lookup_internal(&t) {
            return x;
        }

        let enode = t.0.refresh_private().apply_slotmap(&t.1);
        let enode = self.synify_enode(enode);

        self.mk_singleton_class(t, enode)
    }

    pub fn lookup(&self, n: &L) -> Option<AppliedId> {
        self.lookup_internal(&self.shape(n))
    }

    pub(in crate::egraph) fn lookup_internal(&self, (shape, n_bij): &(L, Bijection)) -> Option<AppliedId> {
        let i = self.hashcons.get(&shape)?;
        let c = &self.classes[i];
        let cn_bij = &c.nodes[&shape].0;

        // X = shape.slots()
        // Y = n.slots()
        // Z = c.slots()
        // n_bij :: X -> Y
        // cn_bij :: X -> Z
        // out :: Z -> Y
        let out = cn_bij.inverse().compose(&n_bij);

        // Note that ENodes in an EClass can have redundant slots.
        // They shouldn't come up in the AppliedId.
        let out = out.iter().filter(|(x, _)| c.slots.contains(x)).collect();

        let app_id = self.mk_sem_applied_id(
            *i,
            out,
        );

        if CHECKS {
            assert_eq!(&c.slots, &app_id.m.keys());
        }

        Some(app_id)
    }
}

impl<L: Language> EGraph<L> {
    // returns a sem applied id.
    fn mk_singleton_class(&mut self, (sh, bij): (L, SlotMap), syn_enode: L) -> AppliedId {
        let old_slots = bij.values();

        let fresh_to_old = Bijection::bijection_from_fresh_to(&old_slots);
        let old_to_fresh = fresh_to_old.inverse();

        // allocate new class & slot set.
        let fresh_slots = old_to_fresh.values();
        let syn_enode_real = syn_enode.apply_slotmap_fresh(&old_to_fresh);
        let i = self.alloc_eclass(&fresh_slots, syn_enode_real.clone());

        let m = bij.compose(&old_to_fresh);
        let syn_app_id = AppliedId::new(i, SlotMap::identity(&syn_enode_real.slots()));
        self.raw_add_to_class(i, (sh, m), syn_app_id);
        self.mk_sem_applied_id(i, fresh_to_old)
    }

    // adds (sh, bij) to the eclass `id`.
    pub(in crate::egraph) fn raw_add_to_class(&mut self, id: Id, (sh, bij): (L, Bijection), src_id: AppliedId) {
        let tmp1 = self.classes.get_mut(&id).unwrap().nodes.insert(sh.clone(), (bij, src_id));
        let tmp2 = self.hashcons.insert(sh.clone(), id);
        if CHECKS {
            assert!(tmp1.is_none());
            assert!(tmp2.is_none());
        }
        for ref_id in sh.ids() {
            let usages = &mut self.classes.get_mut(&ref_id).unwrap().usages;
            usages.insert(sh.clone());
        }
    }

    // TODO: the bijection here is probably useless and error-prone. Let's only get Id and Shape as arguments here.
    pub(in crate::egraph) fn raw_remove_from_class(&mut self, id: Id, (sh, _bij): (L, Bijection)) {
        let tmp1 = self.classes.get_mut(&id).unwrap().nodes.remove(&sh);
        let tmp2 = self.hashcons.remove(&sh);
        if CHECKS {
            assert!(tmp1.is_some());
            assert!(tmp2.is_some());
        }
        for ref_id in sh.ids() {
            let usages = &mut self.classes.get_mut(&ref_id).unwrap().usages;
            usages.remove(&sh);
        }
    }

}

impl<L: Language> EGraph<L> {
    // TODO make the public API auto "fresh" slots.
    pub fn alloc_empty_eclass(&mut self, slots: &HashSet<Slot>) -> Id {
        panic!("Can't use alloc_empty_eclass if explanations are enabled!");
        self.alloc_eclass(slots, panic!())
    }

    pub(in crate::egraph) fn alloc_eclass(&mut self, slots: &HashSet<Slot>, syn_enode: L) -> Id {
        let c_id = Id(self.unionfind_len()); // Pick the next unused Id.

        let syn_slots = syn_enode.slots();
        let proven_perm = ProvenPerm::identity(c_id, &slots, &syn_slots);

        let app_id = AppliedId::new(c_id, SlotMap::identity(&syn_slots));
        let redundancy_proof = prove_reflexivity(&app_id);

        let c = EClass {
            nodes: HashMap::default(),
            group: Group::identity(&proven_perm),
            slots: slots.clone(),
            usages: HashSet::default(),
            redundancy_proof,
            syn_enode: syn_enode.clone(),
        };
        self.classes.insert(c_id, c);

        { // add syn_enode to the hashcons.
            let (sh, bij) = syn_enode.weak_shape();

            if CHECKS {
                assert!(!self.syn_hashcons.contains_key(&sh));
            }

            let app_id = self.mk_syn_applied_id(c_id, bij.inverse());
            self.syn_hashcons.insert(sh, app_id);
        }

        let app_id = self.mk_sem_identity_applied_id(c_id);
        let prf = self.prove_reflexivity(&app_id);
        self.unionfind_set(c_id, app_id, prf);

        c_id
    }
}
