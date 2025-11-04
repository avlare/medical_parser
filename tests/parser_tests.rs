use anyhow::Result;
use medical_parser::*;

#[test]
fn test_parse_basic_patient() -> Result<()> {
    let input = r#"
    <patient>
        <name>Big Meow</name>
        <age>110</age>
        <visit>
            <diagnosis>Flu</diagnosis>
            <temperature>38.1</temperature>
            <notes>Feeling weak, advised rest.</notes>
        </visit>
        <visit>
            <diagnosis>Check-up</diagnosis>
            <temperature>36.7</temperature>
            <notes>Condition improved.</notes>
        </visit>
    </patient>
    "#;

    let patients = parse_medical_document(input)?;
    assert_eq!(patients.len(), 1);

    let p = &patients[0];
    assert_eq!(p.name, "Big Meow");
    assert_eq!(p.age, 110);
    assert_eq!(p.visits.len(), 2);

    let v1 = &p.visits[0];
    assert_eq!(v1.diagnosis, "Flu");
    assert!((v1.temperature - 38.1).abs() < 0.01);
    assert!(v1.notes.contains("weak"));

    Ok(())
}

#[test]
fn test_parse_multiple_patients() -> Result<()> {
    let input = r#"
    <patient>
        <name>Lilililalala</name>
        <age>42</age>
        <visit>
            <diagnosis>Check-up</diagnosis>
            <temperature>36.5</temperature>
            <notes>All good.</notes>
        </visit>
    </patient>
    <patient>
        <name>LilililalalaLululuku</name>
        <age>36</age>
    </patient>
    "#;

    let patients = parse_medical_document(input)?;
    assert_eq!(patients.len(), 2);
    assert_eq!(patients[0].name, "Lilililalala");
    assert_eq!(patients[1].visits.len(), 0);

    Ok(())
}

#[test]
fn test_parse_missing_age_is_err() {
    let input = r#"
    <patient>
        <name>No Age</name>
        <visit>
            <diagnosis>Check-up</diagnosis>
            <temperature>36.7</temperature>
            <notes>Missing age should fail.</notes>
        </visit>
    </patient>
    "#;

    assert!(parse_medical_document(input).is_err());
}
