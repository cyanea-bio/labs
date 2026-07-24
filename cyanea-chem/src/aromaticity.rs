//! Aromaticity perception.
//!
//! After a molecule is parsed, Kekulé-form aromatics (written with explicit
//! alternating single/double bonds) carry ring topology but no aromatic flags.
//! [`perceive_aromaticity`] runs a Hückel-style ring analysis over the
//! molecular graph and marks the atoms and bonds of aromatic rings, so that a
//! Kekulé SMILES and the equivalent lower-case aromatic SMILES normalize to the
//! same aromatic graph.

use crate::molecule::{BondOrder, Molecule};
use crate::ring::find_sssr;

/// Detect aromatic rings and mark their atoms/bonds aromatic.
///
/// Uses the smallest set of smallest rings; a ring is aromatic when every
/// member atom is sp²-participating and the π-electron count satisfies Hückel's
/// 4n+2 rule. Existing aromatic flags (e.g. from lower-case SMILES) are left
/// untouched — the pass only turns aromaticity on.
pub fn perceive_aromaticity(mol: &mut Molecule) {
    let rings = find_sssr(mol);
    if rings.is_empty() {
        return;
    }

    // Atoms that belong to any ring — used to tell a π bond that stays inside
    // the fused ring system (counts) from an exocyclic double bond (does not).
    let mut ring_atom = vec![false; mol.atoms.len()];
    for ring in &rings {
        for &a in ring {
            ring_atom[a] = true;
        }
    }

    // Decide aromaticity from the *original* bond orders before mutating.
    let mut aromatic_atom = vec![false; mol.atoms.len()];
    let mut aromatic_bond = vec![false; mol.bonds.len()];

    for ring in &rings {
        if !ring_is_aromatic(mol, ring, &ring_atom) {
            continue;
        }
        let k = ring.len();
        for i in 0..k {
            let a = ring[i];
            let b = ring[(i + 1) % k];
            aromatic_atom[a] = true;
            if let Some(&(_, bond_idx)) = mol.adjacency[a].iter().find(|&&(n, _)| n == b) {
                aromatic_bond[bond_idx] = true;
            }
        }
    }

    for (i, &arom) in aromatic_atom.iter().enumerate() {
        if arom {
            mol.atoms[i].is_aromatic = true;
        }
    }
    for (i, &arom) in aromatic_bond.iter().enumerate() {
        if arom {
            mol.bonds[i].is_aromatic = true;
            mol.bonds[i].order = BondOrder::Aromatic;
        }
    }
}

/// A ring is aromatic when every atom contributes to a continuous π system and
/// the total π-electron count is 4n+2.
fn ring_is_aromatic(mol: &Molecule, ring: &[usize], ring_atom: &[bool]) -> bool {
    let mut pi = 0u32;
    for &a in ring {
        match pi_contribution(mol, a, ring_atom) {
            Some(e) => pi += e,
            None => return false,
        }
    }
    pi >= 2 && (pi - 2).is_multiple_of(4)
}

/// π electrons a ring atom contributes to the ring system, or `None` if the
/// atom cannot be part of an aromatic ring (e.g. an sp³ centre).
fn pi_contribution(mol: &Molecule, atom_idx: usize, ring_atom: &[bool]) -> Option<u32> {
    let atom = &mol.atoms[atom_idx];
    let charge = atom.formal_charge;

    // Classify this atom's multiple bonds.
    let mut double_to_ring = false; // π bond staying inside the ring system
    let mut double_to_exo = false; // exocyclic double bond (e.g. C=O)
    for &(n, bond_idx) in &mol.adjacency[atom_idx] {
        match mol.bonds[bond_idx].order {
            BondOrder::Double => {
                if ring_atom[n] {
                    double_to_ring = true;
                } else {
                    double_to_exo = true;
                }
            }
            // An sp centre (triple bond) cannot be aromatic.
            BondOrder::Triple => return None,
            _ => {}
        }
    }

    match atom.atomic_number {
        // Carbon
        6 => {
            if double_to_ring {
                Some(1)
            } else if double_to_exo {
                Some(0) // sp² but the π density points out of the ring
            } else if charge < 0 {
                Some(2) // carbanion (e.g. cyclopentadienyl)
            } else if charge > 0 {
                Some(0) // carbocation (e.g. tropylium)
            } else {
                None // sp³ carbon
            }
        }
        // Nitrogen, phosphorus
        7 | 15 => {
            if double_to_ring {
                Some(1) // pyridine-type =N-
            } else if double_to_exo {
                Some(0)
            } else {
                Some(2) // pyrrole-type: lone pair into the ring
            }
        }
        // Oxygen, sulfur, selenium
        8 | 16 | 34 => {
            if double_to_ring {
                Some(1) // e.g. pyrylium O+
            } else if double_to_exo {
                Some(0)
            } else {
                Some(2) // furan/thiophene lone pair
            }
        }
        // Boron: empty p orbital
        5 => Some(if double_to_ring { 1 } else { 0 }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::canon::canonical_smiles;
    use crate::smiles::parse_smiles;

    fn aromatic_atoms(smiles: &str) -> usize {
        parse_smiles(smiles)
            .unwrap()
            .atoms
            .iter()
            .filter(|a| a.is_aromatic)
            .count()
    }

    #[test]
    fn benzene_kekule_is_aromatized() {
        assert_eq!(aromatic_atoms("C1=CC=CC=C1"), 6);
        assert_eq!(aromatic_atoms("c1ccccc1"), 6);
    }

    #[test]
    fn naphthalene_kekule_matches_aromatic_form() {
        let aromatic = parse_smiles("c1ccc2ccccc2c1").unwrap();
        let kekule = parse_smiles("C1=CC=C2C=CC=CC2=C1").unwrap();

        let count_a = aromatic.atoms.iter().filter(|a| a.is_aromatic).count();
        let count_k = kekule.atoms.iter().filter(|a| a.is_aromatic).count();
        assert_eq!(
            count_a, 10,
            "aromatic-form naphthalene has 10 aromatic atoms"
        );
        assert_eq!(
            count_k, 10,
            "kekulé-form naphthalene should aromatize to 10"
        );

        // The two forms should normalize to the same canonical SMILES.
        assert_eq!(canonical_smiles(&aromatic), canonical_smiles(&kekule));
    }

    #[test]
    fn indole_kekule_aromatizes_including_ring_nitrogen() {
        // Indole: fused benzene (6) + pyrrole (5), 9 ring atoms incl. the N.
        let indole = parse_smiles("C12=C(C=CN2)C=CC=C1").unwrap();
        assert_eq!(
            indole.atoms.iter().filter(|a| a.is_aromatic).count(),
            9,
            "all 9 fused-ring atoms (including N) should be aromatic"
        );
        // The ring nitrogen specifically must be aromatic.
        let n_aromatic = indole
            .atoms
            .iter()
            .any(|a| a.atomic_number == 7 && a.is_aromatic);
        assert!(n_aromatic, "the ring nitrogen should be aromatic");
    }

    #[test]
    fn pyridine_and_pyrrole_kekule_are_aromatic() {
        assert_eq!(aromatic_atoms("C1=CC=NC=C1"), 6); // pyridine
        assert_eq!(aromatic_atoms("C1=CC=CN1"), 5); // pyrrole (N-H)
    }

    #[test]
    fn non_aromatic_rings_stay_non_aromatic() {
        assert_eq!(aromatic_atoms("C1CCCCC1"), 0); // cyclohexane
        assert_eq!(aromatic_atoms("C1=CCCCC1"), 0); // cyclohexene
        assert_eq!(aromatic_atoms("O=C1CCCCC1"), 0); // cyclohexanone
    }
}
