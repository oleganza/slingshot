extern crate spacesuit;
use spacesuit::{prove, verify, Value, SpacesuitError};

extern crate curve25519_dalek;

extern crate bulletproofs;
use bulletproofs::{BulletproofGens, PedersenGens};

fn spacesuit_helper(
    bp_gens: &BulletproofGens,
    inputs: Vec<Value>,
    outputs: Vec<Value>,
) -> Result<(), SpacesuitError> {
    let m = inputs.len();
    let n = outputs.len();
    let pc_gens = PedersenGens::default();

    let (proof, commitments) = prove(&bp_gens, &pc_gens, &inputs, &outputs)?;
    verify(&bp_gens, &pc_gens, &proof, commitments, m, n)
}

// Helper functions to make the tests easier to read
fn yuan(q: u64) -> Value {
    Value {
        q,
        a: 888u64.into(),
        t: 999u64.into(),
    }
}
fn peso(q: u64) -> Value {
    Value {
        q,
        a: 666u64.into(),
        t: 777u64.into(),
    }
}
fn euro(q: u64) -> Value {
    Value {
        q,
        a: 444u64.into(),
        t: 555u64.into(),
    }
}
fn zero() -> Value {
    Value::zero()
}

// m=1, n=1
#[test]
fn spacesuit_1_1() {
    let bp_gens = BulletproofGens::new(1000, 1);
    assert!(spacesuit_helper(&bp_gens, vec![yuan(1)], vec![yuan(1)]).is_ok());
    assert!(spacesuit_helper(&bp_gens, vec![peso(4)], vec![peso(4)]).is_ok());
    assert!(spacesuit_helper(&bp_gens, vec![yuan(1)], vec![peso(4)]).is_err());
}

// max(m, n) = 2
#[test]
fn spacesuit_uneven_2() {
    let bp_gens = BulletproofGens::new(1000, 1);
    assert!(spacesuit_helper(&bp_gens, vec![yuan(3)], vec![yuan(1), yuan(2)]).is_ok());
    assert!(spacesuit_helper(&bp_gens, vec![yuan(1), yuan(2)], vec![yuan(3)]).is_ok());
}

// m=2, n=2
#[test]
fn spacesuit_2_2() {
    let bp_gens = BulletproofGens::new(1000, 1);
    // Only shuffle (all different flavors)
    assert!(spacesuit_helper(&bp_gens, vec![yuan(1), peso(4)], vec![yuan(1), peso(4)]).is_ok());
    assert!(spacesuit_helper(&bp_gens, vec![yuan(1), peso(4)], vec![peso(4), yuan(1)]).is_ok());

    // Middle shuffle & merge & split (has multiple inputs or outputs of same flavor)
    assert!(spacesuit_helper(&bp_gens, vec![peso(4), peso(4)], vec![peso(4), peso(4)]).is_ok());
    assert!(spacesuit_helper(&bp_gens, vec![peso(5), peso(3)], vec![peso(5), peso(3)]).is_ok());
    assert!(spacesuit_helper(&bp_gens, vec![peso(5), peso(3)], vec![peso(1), peso(7)]).is_ok());
    assert!(spacesuit_helper(&bp_gens, vec![peso(1), peso(8)], vec![peso(0), peso(9)]).is_ok());
    assert!(spacesuit_helper(&bp_gens, vec![yuan(1), yuan(1)], vec![peso(4), yuan(1)]).is_err());
}

// m=3, n=3
#[test]
fn spacesuit_3_3() {
    let bp_gens = BulletproofGens::new(1000, 1);
    // Only shuffle
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), euro(8)],
            vec![yuan(1), peso(4), euro(8)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), euro(8)],
            vec![yuan(1), euro(8), peso(4)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), euro(8)],
            vec![peso(4), yuan(1), euro(8)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), euro(8)],
            vec![peso(4), euro(8), yuan(1)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), euro(8)],
            vec![euro(8), yuan(1), peso(4)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), euro(8)],
            vec![euro(8), peso(4), yuan(1)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), euro(8)],
            vec![yuan(2), peso(4), euro(8)]
        )
        .is_err()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), euro(8)],
            vec![yuan(1), euro(4), euro(8)]
        )
        .is_err()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), euro(8)],
            vec![yuan(1), peso(4), euro(9)]
        )
        .is_err()
    );

    // Middle shuffle & merge & split
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), yuan(1), peso(4)],
            vec![yuan(1), yuan(1), peso(4)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(4), yuan(3), peso(4)],
            vec![yuan(2), yuan(5), peso(4)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(4), yuan(3), peso(4)],
            vec![peso(4), yuan(2), yuan(5)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), yuan(2), yuan(5)],
            vec![yuan(4), yuan(3), yuan(1)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), yuan(2), yuan(5)],
            vec![yuan(4), yuan(3), yuan(10)]
        )
        .is_err()
    );

    // End shuffles & merge & split & middle shuffle
    // (multiple asset types that need to be grouped and merged or split)
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), yuan(1)],
            vec![yuan(1), yuan(1), peso(4)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(4), peso(4), yuan(3)],
            vec![peso(3), yuan(7), peso(1)]
        )
        .is_ok()
    );
}

// max(m, n) = 3
#[test]
fn spacesuit_uneven_3() {
    let bp_gens = BulletproofGens::new(1000, 1);
    assert!(spacesuit_helper(&bp_gens, vec![yuan(4), yuan(4), yuan(3)], vec![yuan(11)]).is_ok());
    assert!(spacesuit_helper(&bp_gens, vec![yuan(11)], vec![yuan(4), yuan(4), yuan(3)],).is_ok());
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(11), peso(4)],
            vec![yuan(4), yuan(7), peso(4)],
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(4), yuan(7), peso(4)],
            vec![yuan(11), peso(4)],
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(5), yuan(6)],
            vec![yuan(4), yuan(4), yuan(3)],
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(4), yuan(4), yuan(3)],
            vec![yuan(5), yuan(6)],
        )
        .is_ok()
    );
}

// m=4, n=4
#[test]
fn spacesuit_4_4() {
    let bp_gens = BulletproofGens::new(1000, 1);
    // Only shuffle
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), euro(7), euro(10)],
            vec![yuan(1), peso(4), euro(7), euro(10)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), euro(7), euro(10)],
            vec![euro(7), yuan(1), euro(10), peso(4),]
        )
        .is_ok()
    );

    // Middle shuffle & merge & split
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), yuan(1), peso(4), peso(4)],
            vec![yuan(1), yuan(1), peso(4), peso(4)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(4), yuan(3), peso(4), peso(4)],
            vec![yuan(2), yuan(5), peso(1), peso(7)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(4), yuan(3), peso(4), peso(4)],
            vec![peso(1), peso(7), yuan(2), yuan(5)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), yuan(1), yuan(5), yuan(2)],
            vec![yuan(1), yuan(1), yuan(5), yuan(2)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), yuan(2), yuan(5), yuan(2)],
            vec![yuan(4), yuan(3), yuan(3), zero()]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), yuan(2), yuan(5), yuan(2)],
            vec![yuan(4), yuan(3), yuan(3), yuan(20)]
        )
        .is_err()
    );

    // End shuffles & merge & split & middle shuffle
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(1), peso(4), yuan(1), peso(4)],
            vec![peso(4), yuan(1), yuan(1), peso(4)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(4), peso(4), peso(4), yuan(3)],
            vec![peso(1), yuan(2), yuan(5), peso(7)]
        )
        .is_ok()
    );
    assert!(
        spacesuit_helper(
            &bp_gens,
            vec![yuan(10), peso(1), peso(2), peso(3)],
            vec![yuan(5), yuan(4), yuan(1), peso(6)]
        )
        .is_ok()
    );
}
