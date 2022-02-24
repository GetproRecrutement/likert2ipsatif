use std::collections::{HashMap, HashSet};

use crate::{Assessment, BaseAssessment};

pub(crate) struct QuestionData {
    domain: String,
    subdomain: String,
}

pub(crate) type QuestionsWithData = HashMap<String, QuestionData>;

pub(crate) fn get_questions_data(assessment: &BaseAssessment) -> QuestionsWithData {
    let mut res: QuestionsWithData = HashMap::new();

    for domain in assessment.domains.iter() {
        for subdomain in domain.subdomains.iter() {
            for question in subdomain.questions.iter() {
                if res
                    .insert(
                        question.to_owned(),
                        QuestionData {
                            domain: domain.name.to_owned(),
                            subdomain: subdomain.name.to_owned(),
                        },
                    )
                    .is_some()
                {
                    panic!("Duplicate question");
                }
            }
        }
    }

    res
}

pub(crate) fn count_duplicated_subdomains<const N: usize>(
    questions_data: &QuestionsWithData,
    assessment: &Assessment<N>,
) -> u64 {
    let mut duplicates = 0;

    let mut existing = HashSet::new();

    for part in assessment.parts.iter() {
        part.questions.iter().for_each(|q| {
            let data = questions_data
                .get(q)
                .expect(&format!("Failed to find question data for {}", q));

            if !existing.insert(data.subdomain.to_owned()) {
                duplicates += 1;
            }
        });
    }

    duplicates
}

pub(crate) fn count_duplicated_domains<const N: usize>(
    questions_data: &QuestionsWithData,
    assessment: &Assessment<N>,
) -> u64 {
    let mut duplicates = 0;

    let mut existing = HashSet::new();

    for part in assessment.parts.iter() {
        part.questions.iter().for_each(|q| {
            let data = questions_data
                .get(q)
                .unwrap_or_else(|| panic!("Failed to find question data for {}", q));

            if !existing.insert(data.domain.to_owned()) {
                duplicates += 1;
            }
        });
    }

    duplicates
}

pub(crate) fn count_duplicated_questions<const N: usize>(assessment: &Assessment<N>) -> u64 {
    let mut duplicates = 0;

    let mut existing = HashSet::new();

    for part in assessment.parts.iter() {
        part.questions.iter().for_each(|q| {
            if !existing.insert(q) {
                duplicates += 1;
            }
        });
    }

    duplicates
}
