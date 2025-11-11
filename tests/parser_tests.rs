use anyhow::Result;
use medical_parser::*;
use pest::Parser;

// unit tests for the parser rules
#[test]
fn test_file_rule_parses_patients() {
    let input = "<patient><name>Meow</name><age>10</age></patient>";
    let result = Grammar::parse(Rule::file, input);
    assert!(result.is_ok());
}

#[test]
fn test_patient_rule_valid() {
    let input = "<patient><name>Meow</name><age>23</age></patient>";
    let result = Grammar::parse(Rule::patient, input);
    assert!(result.is_ok());
}

#[test]
fn test_patient_rule_missing_age_fails() {
    let input = "<patient><name>Meow</name></patient>";
    let result = Grammar::parse(Rule::patient, input);
    assert!(result.is_err());
}

#[test]
fn test_visit_with_all_fields() {
    let input = "<visit><diagnosis>Cold</diagnosis><temperature>37.1</temperature><notes>Rest</notes></visit>";
    let result = Grammar::parse(Rule::visit, input);
    assert!(result.is_ok());
}

#[test]
fn test_name_rule() {
    let input = "<name>Big Meow</name>";
    let result = Grammar::parse(Rule::name, input);
    assert!(result.is_ok());
}

#[test]
fn test_age_rule_valid_integer() {
    let input = "<age>45</age>";
    assert!(Grammar::parse(Rule::age, input).is_ok());
}

#[test]
fn test_age_rule_invalid_text_fails() {
    let input = "<age>forty</age>";
    assert!(Grammar::parse(Rule::age, input).is_err());
}

#[test]
fn test_diagnosis_rule() {
    let input = "<diagnosis>Flu</diagnosis>";
    assert!(Grammar::parse(Rule::diagnosis, input).is_ok());
}

#[test]
fn test_temperature_float_and_int() {
    assert!(Grammar::parse(Rule::temperature, "<temperature>36.6</temperature>").is_ok());
    assert!(Grammar::parse(Rule::temperature, "<temperature>39</temperature>").is_ok());
}

#[test]
fn test_notes_rule() {
    let input = "<notes>Feeling fine.</notes>";
    assert!(Grammar::parse(Rule::notes, input).is_ok());
}

#[test]
fn test_text_rule_allows_letters_and_spaces() {
    assert!(Grammar::parse(Rule::text, "Meow meow").is_ok());
    assert!(Grammar::parse(Rule::text, "<little meow").is_err());
}

#[test]
fn test_number_rule_valid_examples() {
    assert!(Grammar::parse(Rule::number, "38").is_ok());
    assert!(Grammar::parse(Rule::number, "36.7").is_ok());
    assert!(Grammar::parse(Rule::number, "-5.3").is_ok());
}

#[test]
fn test_number_rule_invalid_examples() {
    assert!(Grammar::parse(Rule::number, "a4").is_err());
    assert!(Grammar::parse(Rule::number, "abc").is_err());
}


// integration tests for full parsing functionality
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
