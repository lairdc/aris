//! Patterns for rewriting equivalences (a specific type of rule).

use crate::rewrite_rules::RewriteRule;

/// Defines literal data used for a rewrite rule.
///
/// ```ignore
/// # use aris::equivalences::define_rewrite_rule;
/// define_rewrite_rule! {
///     NAME_OF_RULE,
///     &[
///         ("pattern", "replacement"),
///     ]
/// }
/// ```
macro_rules! define_rewrite_rule {
    ($name:ident, $rules:expr) => {
        lazy_static! {
            pub static ref $name: RewriteRule = RewriteRule::from_patterns($rules);
        }
    };
}

// Boolean Equivalences
define_rewrite_rule! {
    DOUBLE_NEGATION,
    &[
        ("~~P", "P")
    ]
}
define_rewrite_rule! {
    DISTRIBUTION,
    &[
        ("(P & Q) | (P & R)", "P & (Q | R)"),
        ("(P | Q) & (P | R)", "P | (Q & R)"),
    ]
}
define_rewrite_rule! {
    IDENTITY,
    &[
        ("phi & ^|^", "phi"),
        ("phi | _|_", "phi"),
    ]
}
define_rewrite_rule! {
    ANNIHILATION,
    &[
        ("phi & _|_", "_|_"),
        ("phi | ^|^", "^|^"),
    ]
}
define_rewrite_rule! {
    INVERSE,
    &[
        ("~^|^", "_|_"),
        ("~_|_", "^|^"),
    ]
}

// Conditional Equivalences
define_rewrite_rule! {
    CONDITIONAL_ABSORPTION,
    &[
        ("phi & (~phi -> psi)", "phi"),
        ("psi & (phi -> psi)", "psi"),
    ]
}
define_rewrite_rule! {
    CONDITIONAL_COMPLEMENT,
    &[
        ("phi -> phi", "^|^"),
    ]
}
define_rewrite_rule! {
    CONDITIONAL_IDENTITY,
    &[
        ("phi -> _|_", "~phi"),
        ("^|^ -> phi", "phi"),
    ]
}
define_rewrite_rule! {
    CONDITIONAL_ANNIHILATION,
    &[
        ("phi -> ^|^", "^|^"),
        ("_|_ -> phi", "^|^"),
    ]
}
define_rewrite_rule! {
    CONDITIONAL_IMPLICATION,
    &[
        ("phi -> psi", "~phi | psi"),
        ("~(phi -> psi)", "phi & ~psi"),
    ]
}
// equivalence
define_rewrite_rule! {
    CONDITIONAL_CONTRAPOSITION,
    &[
        ("~phi -> ~psi", "psi -> phi"),
    ]
}
// exportation
define_rewrite_rule! {
    CONDITIONAL_EXPORTATION,
    &[
        ("phi -> (psi -> lambda)", "(phi & psi) -> lambda"),
    ]
}
define_rewrite_rule! {
    CONDITIONAL_DISTRIBUTION,
    &[
        ("phi -> (psi & lambda)", "(phi -> psi) & (phi -> lambda)"),
        ("(phi | psi) -> lambda", "(phi -> lambda) & (psi -> lambda)"),
        ("phi -> (psi | lambda)", "(phi -> psi) | (phi -> lambda)"),
        ("(phi & psi) -> lambda", "(phi -> lambda) | (psi -> lambda)"),
    ]
}
define_rewrite_rule! {
    CONDITIONAL_REDUCTION,
    &[
        ("phi & (phi -> psi)", "phi & psi"),
        ("~psi & (phi -> psi)", "~psi & ~phi"),
    ]
}
define_rewrite_rule! {
    KNIGHTS_AND_KNAVES,
    &[
        ("phi <-> (phi & psi)", "phi -> psi"),
        ("phi <-> (phi | psi)", "psi -> phi"),
    ]
}
define_rewrite_rule! {
    CONDITIONAL_IDEMPOTENCE,
    &[
        ("phi -> ~phi", "~phi"),
        ("~phi -> phi", "phi"),
    ]
}

// Biconditional Equivalences
define_rewrite_rule! {
    BICONDITIONAL_EQUIVALENCE,
    &[
        ("(phi -> psi) & (psi -> phi)", "phi <-> psi"),
        ("(phi & psi) | (~phi & ~psi)", "phi <-> psi"),
    ]
}
define_rewrite_rule! {
    BICONDITIONAL_COMMUTATION,
    &[
        ("phi <-> psi", "psi <-> phi"),
    ]
}
define_rewrite_rule! {
    BICONDITIONAL_ASSOCIATION,
    &[
        ("phi <-> (psi <-> lambda)", "(phi <-> psi) <-> lambda"),
    ]
}
define_rewrite_rule! {
    BICONDITIONAL_REDUCTION,
    &[
        ("phi & (phi <-> psi)", "phi & psi"),
        ("~phi & (phi <-> psi)", "~phi & ~psi"),
    ]
}
define_rewrite_rule! {
    BICONDITIONAL_COMPLEMENT,
    &[
        ("phi <-> phi", "^|^"),
        ("phi <-> ~phi", "_|_"),
    ]
}
define_rewrite_rule! {
    BICONDITIONAL_IDENTITY,
    &[
        ("phi <-> _|_", "~phi"),
        ("phi <-> ^|^", "phi"),
    ]
}
define_rewrite_rule! {
    BICONDITIONAL_NEGATION,
    &[
        ("~phi <-> psi", "~(phi <-> psi)"),
        ("phi <-> ~psi", "~(phi <-> psi)"),
    ]
}
define_rewrite_rule! {
    BICONDITIONAL_SUBSTITUTION,
    &[
        ("(phi <-> psi) & S(phi)", "(phi <-> psi) & S(psi)"),
    ]
}

define_rewrite_rule! {
    CONJUNCTION,
    &[
        ("phi & ^|^", "phi"),
        ("phi & _|_", "_|_"),

    ]
}

define_rewrite_rule! {
    DISJUNCTION,
    &[
        ("phi | _|_", "phi"),
        ("phi | ^|^", "^|^"),
    ]
}

define_rewrite_rule! {
    NEGATION,
    &[
        ("~^|^", "_|_"),
        ("~_|_", "^|^"),
    ]
}

define_rewrite_rule! {
    BICOND_REDUCTION,
    &[
        ("phi <-> _|_", "~phi"),
        ("phi <-> ^|^", "phi"),
    ]
}

define_rewrite_rule! {
    COND_REDUCTION,
    &[
        ("phi -> ^|^", "^|^"),
        ("_|_ -> phi", "^|^"),
        ("phi -> _|_", "~phi"),
        ("^|^ -> phi", "phi"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::expr::free_vars;

    fn for_each_truthtable<F>(n: usize, mut f: F)
    where
        F: FnMut(&[bool]),
    {
        let mut table = vec![false; n];
        for x in 0..(2usize.pow(n as _)) {
            for (i, value) in table.iter_mut().enumerate() {
                *value = (x & (1 << i)) != 0;
            }
            f(&table[..]);
        }
    }

    /// Test function to verify the logical equivalence of rewrite rules using brute-force truth tables.
    #[test]
    fn bruteforce_equivalence_truthtables() {
        use std::collections::HashMap;
        let rules: Vec<&RewriteRule> = vec![&*DOUBLE_NEGATION, &*DISTRIBUTION, &*IDENTITY, &*ANNIHILATION, &*INVERSE, &*CONDITIONAL_ABSORPTION, &*CONDITIONAL_ANNIHILATION, &*CONDITIONAL_IMPLICATION, &*CONDITIONAL_CONTRAPOSITION, &*CONDITIONAL_EXPORTATION, &*CONDITIONAL_COMPLEMENT, &*CONDITIONAL_IDENTITY, &*CONDITIONAL_DISTRIBUTION, &*CONDITIONAL_REDUCTION, &*KNIGHTS_AND_KNAVES, &*CONDITIONAL_IDEMPOTENCE, &*BICONDITIONAL_ASSOCIATION, &*BICONDITIONAL_COMMUTATION, &*BICONDITIONAL_REDUCTION, &*BICONDITIONAL_COMPLEMENT, &*BICONDITIONAL_IDENTITY, &*BICONDITIONAL_EQUIVALENCE, &*BICONDITIONAL_NEGATION, &*BICONDITIONAL_SUBSTITUTION];
        for rule in rules {
            for (lhs, rhs) in rule.reductions.iter() {
                println!("Testing {lhs} -> {rhs}");
                let mut fvs: Vec<String> = free_vars(lhs).union(&free_vars(rhs)).cloned().collect();
                fvs.sort();
                let mut arities = HashMap::new();
                lhs.infer_arities(&mut arities);
                rhs.infer_arities(&mut arities);
                println!("Inferred arities: {arities:?}");
                let total_arity = arities.values().map(|v| 2usize.pow(*v as _)).sum();
                for_each_truthtable(total_arity, |table| {
                    let mut env = HashMap::new();
                    let mut i = 0;
                    for fv in fvs.iter().cloned() {
                        let n = 2usize.pow(arities[&fv] as _);
                        env.insert(fv, table[i..i + n].to_vec());
                        i += n;
                    }
                    println!("{table:?} {env:?}");
                    assert_eq!(lhs.eval(&env), rhs.eval(&env));
                });
                println!("-----");
            }
        }
    }
}
