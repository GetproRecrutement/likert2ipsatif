use crate::utils::{
    count_duplicated_domains, count_duplicated_questions, count_duplicated_subdomains,
    get_questions_data,
};

mod utils;

#[derive(Debug, Default)]
struct BaseAssessment {
    /// Domaine
    domains: Vec<Domain>,
}

#[derive(Debug)]
struct Domain {
    name: String,
    /// Facette
    subdomains: Vec<SubDomain>,
}

#[derive(Debug)]
struct SubDomain {
    name: String,
    /// Items
    questions: Vec<String>,
}

#[derive(Debug)]
struct AssessmentPart<const N: usize> {
    questions: [String; N],
}

#[derive(Debug)]
struct Assessment<const N: usize> {
    pub parts: Vec<AssessmentPart<N>>,
}

trait Buildable<const N: usize> {
    fn build(self) -> Assessment<N>;
}

fn base_assessment() -> BaseAssessment {
    let mut assessment = BaseAssessment::default();

    for char in ['A', 'C', 'E', 'N', 'O'] {
        let mut domain = Domain {
            name: char.to_string(),
            subdomains: vec![],
        };

        for f in 0..6 {
            let mut questions = vec![];

            for q in 0..10 {
                questions.push(format!("{char}{f}.{q}"));
            }

            domain.subdomains.push(SubDomain {
                name: format!("{char}{f}"),
                questions,
            })
        }

        assessment.domains.push(domain);
    }

    assessment
}

// 4 questions per page
impl Buildable<4> for BaseAssessment {
    fn build(self) -> Assessment<4> {
        todo!()
    }
}

// 3 questions per page
impl Buildable<3> for BaseAssessment {
    fn build(self) -> Assessment<3> {
        todo!()
    }
}

// 3 questions per page
impl Buildable<2> for BaseAssessment {
    fn build(self) -> Assessment<2> {
        todo!()
    }
}

fn main() {
    let assessment = base_assessment();
    let questions_data = get_questions_data(&assessment);

    // println!("{:#?}", eval);

    // Build a 4 questions per page assessment
    let generated: Assessment<4> = assessment.build();

    assert_eq!(count_duplicated_questions(&generated), 0);
    assert_eq!(count_duplicated_subdomains(&questions_data, &generated), 0);
    assert!(count_duplicated_domains(&questions_data, &generated) < 70);
}
