fn legacy_counter_abi_and_payment_effects_match_existing_frozen_snapshots() {
    let counter = mori_core::parse(include_str!("programs/counter.mori")).unwrap();
    let a = mori_backend::compile(&counter, &Target::ledger9("local", "counter-anchor")).unwrap();
    let actual = a
        .circuits
        .iter()
        .map(|c| {
            (
                &c.action,
                c.public_inputs
                    .iter()
                    .map(|p| (&p.role, &p.name))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let snapshot = include_str!("snapshots/compile__authored_counter_abi.snap");
    let expected = snapshot.split_once("\n---\n").unwrap().1.trim_end();
    assert_eq!(format!("{actual:#?}"), expected);

    let payment = mori_core::parse(include_str!("programs/payment.mori")).unwrap();
    let a = mori_backend::compile(&payment, &Target::ledger9("local", "payment-anchor")).unwrap();
    let snapshot = include_str!("snapshots/compile__authored_payment_effects.snap");
    let expected = snapshot.split_once("\n---\n").unwrap().1.trim_end();
    assert_eq!(format!("{:#?}", a.circuits[0].effect_frame), expected);
}
