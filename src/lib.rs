use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

#[derive(Debug)]
pub struct Patient {
    pub name: String,
    pub age: u8,
    pub visits: Vec<Visit>,
}

#[derive(Debug)]
pub struct Visit {
    pub diagnosis: String,
    pub temperature: f32,
    pub notes: String,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    PestError(#[from] Box<pest::error::Error<Rule>>),

    #[error("Invalid data format: {0}")]
    DataError(String),
}

pub fn parse_medical_document(input: &str) -> Result<Vec<Patient>, ParseError> {
    let pairs =
        Grammar::parse(Rule::file, input).map_err(|e| ParseError::PestError(Box::new(e)))?;
    let mut patients = Vec::new();

    for p in pairs {
        for patient in p.into_inner() {
            if patient.as_rule() == Rule::patient {
                patients.push(parse_patient(patient)?);
            }
        }
    }

    Ok(patients)
}

fn parse_patient(pair: pest::iterators::Pair<Rule>) -> Result<Patient, ParseError> {
    let mut name = String::new();
    let mut age = 0;
    let mut visits = Vec::new();

    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::name => name = part.into_inner().as_str().trim().to_string(),
            Rule::age => {
                age = part
                    .into_inner()
                    .as_str()
                    .parse()
                    .map_err(|_| ParseError::DataError("Invalid age value".into()))?;
            }
            Rule::visit => visits.push(parse_visit(part)?),
            _ => {}
        }
    }

    Ok(Patient { name, age, visits })
}

fn parse_visit(pair: pest::iterators::Pair<Rule>) -> Result<Visit, ParseError> {
    let mut diagnosis = String::new();
    let mut temperature = 0.0;
    let mut notes = String::new();

    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::diagnosis => diagnosis = part.into_inner().as_str().trim().to_string(),
            Rule::temperature => {
                temperature = part
                    .into_inner()
                    .as_str()
                    .parse()
                    .map_err(|_| ParseError::DataError("Invalid temperature".into()))?;
            }
            Rule::notes => notes = part.into_inner().as_str().trim().to_string(),
            _ => {}
        }
    }

    Ok(Visit {
        diagnosis,
        temperature,
        notes,
    })
}

pub fn print_patients(patients: &[Patient]) {
    for p in patients {
        println!("{} (age {})", p.name, p.age);
        for v in &p.visits {
            println!("  {}, {} °C — {}", v.diagnosis, v.temperature, v.notes);
        }
        println!();
    }
}

/// Additional analysis functions :)
pub fn count_patients(patients: &[Patient]) -> usize {
    patients.len()
}

pub fn count_total_visits(patients: &[Patient]) -> usize {
    patients.iter().map(|p| p.visits.len()).sum()
}
