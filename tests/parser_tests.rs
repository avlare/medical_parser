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