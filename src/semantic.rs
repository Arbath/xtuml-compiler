use crate::parser::{Model, ClassDef};
use anyhow::{Result, anyhow};
use std::collections::{HashSet, HashMap};

pub fn validate(model: &Model) -> Result<()> {
    // VALIDASI DASAR CLASS (Unique Name & Attributes)
    let mut class_names = HashSet::new();
    let mut class_map: HashMap<String, &ClassDef> = HashMap::new();

    for c in &model.classes {
        // Cek Duplikat Nama Class
        if !class_names.insert(c.name.clone()) {
            return Err(anyhow!("Fatal: Duplicate class name found: '{}'", c.name));
        }
        class_map.insert(c.name.clone(), c);

        // Cek Atribut dalam Class
        let mut attrs = HashSet::new();
        for a in &c.attributes {
            if !attrs.insert(a.name.clone()) {
                return Err(anyhow!("Fatal: Duplicate attribute '{}' in class '{}'", a.name, c.name));
            }
            if a.ty.trim().is_empty() {
                return Err(anyhow!("Fatal: Attribute '{}' in class '{}' has empty type", a.name, c.name));
            }
        }
    }

    // VALIDASI ASOSIASI (Referential Integrity)
    let mut rel_ids = HashSet::new();

    for rel in &model.associations {
        // Cek Duplikat ID Relasi (R1, R2, dst harus unik)
        if !rel_ids.insert(rel.rel_id.clone()) {
            return Err(anyhow!("Fatal: Duplicate Relationship ID found: '{}'", rel.rel_id));
        }

        match rel.ty.as_str() {
            "binary" | "linked" => {
                // Pastikan side_a dan side_b ADA
                let side_a = rel.side_a.as_ref()
                    .ok_or_else(|| anyhow!("Rel '{}' is {}, but missing 'side_a'", rel.rel_id, rel.ty))?;
                let side_b = rel.side_b.as_ref()
                    .ok_or_else(|| anyhow!("Rel '{}' is {}, but missing 'side_b'", rel.rel_id, rel.ty))?;

                // Pastikan Class yang direferensikan ADA di model
                if !class_names.contains(&side_a.class) {
                    return Err(anyhow!("Rel '{}' references unknown class '{}' in side_a", rel.rel_id, side_a.class));
                }
                if !class_names.contains(&side_b.class) {
                    return Err(anyhow!("Rel '{}' references unknown class '{}' in side_b", rel.rel_id, side_b.class));
                }

                // Khusus Linked: Pastikan Link Class ada dan valid
                if rel.ty == "linked" {
                    let link_cls = rel.link_class.as_ref()
                        .ok_or_else(|| anyhow!("Rel '{}' is linked but missing 'link_class'", rel.rel_id))?;
                    
                    if !class_names.contains(link_cls) {
                        return Err(anyhow!("Rel '{}' link_class references unknown class '{}'", rel.rel_id, link_cls));
                    }
                    
                    // Validasi Logika: Link Class tidak boleh sama dengan Side A atau Side B (biasanya)
                    if link_cls == &side_a.class || link_cls == &side_b.class {
                         return Err(anyhow!("Rel '{}': Link class cannot be the same as one of the associated classes", rel.rel_id));
                    }
                }
            },
            "generalization" => {
                // Pastikan Superclass dan Subclasses ada
                let superclass = rel.superclass.as_ref()
                    .ok_or_else(|| anyhow!("Rel '{}' is generalization but missing 'superclass'", rel.rel_id))?;
                
                if !class_names.contains(superclass) {
                    return Err(anyhow!("Rel '{}' references unknown superclass '{}'", rel.rel_id, superclass));
                }

                if rel.subclasses.is_empty() {
                    return Err(anyhow!("Rel '{}' is generalization but has no subclasses defined", rel.rel_id));
                }

                for sub in &rel.subclasses {
                    if !class_names.contains(sub) {
                        return Err(anyhow!("Rel '{}' references unknown subclass '{}'", rel.rel_id, sub));
                    }
                    // Validasi: Circular Inheritance (Sederhana)
                    if sub == superclass {
                        return Err(anyhow!("Rel '{}': Class '{}' cannot be both superclass and subclass", rel.rel_id, sub));
                    }
                }
            },
            unknown => {
                return Err(anyhow!("Rel '{}' has unknown type: '{}'", rel.rel_id, unknown));
            }
        }
    }

    // VALIDASI STATE MACHINE (Flow Integrity)
    for c in &model.classes {
        if let Some(sm_list) = &c.state_model {
            for sm in sm_list {
                // Kumpulkan semua nama state yang valid
                let valid_states: HashSet<String> = sm.states.iter().map(|s| s.name.clone()).collect();

                // Cek Initial State
                if !valid_states.contains(&sm.initial_state) {
                    return Err(anyhow!("Class '{}': Initial state '{}' is not defined in 'states' list", c.name, sm.initial_state));
                }

                // Cek Transisi
                for t in &sm.transitions {
                    if !valid_states.contains(&t.from) {
                        return Err(anyhow!("Class '{}': Transition from unknown state '{}'", c.name, t.from));
                    }
                    if !valid_states.contains(&t.to) {
                        return Err(anyhow!("Class '{}': Transition to unknown state '{}'", c.name, t.to));
                    }
                    
                }
            }
        }
    }

    // VALIDASI EVENTS
    for ev in &model.events {
        if let Some(trigger) = &ev.trigger {
            if !class_names.contains(trigger) {
                return Err(anyhow!("Event '{}' triggers unknown class '{}'", ev.name, trigger));
            }
        }
    }

    Ok(())
}