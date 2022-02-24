#[derive(Debug)]
struct BaseAssessment {
    name: String,
    /// Domaine
    domains: Vec<Domain>,
}

#[derive(Debug)]
struct Domain {
    name: String,
    /// Facette
    subdomain: Vec<SubDomain>,
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
    let mut assessment = BaseAssessment {
        name: "qbg".to_string(),
        domains: vec![],
    };

    for char in ['A', 'C', 'E', 'N', 'O'] {
        let mut domain = Domain {
            name: char.to_string(),
            subdomain: vec![],
        };

        for f in 0..6 {
            let mut questions = vec![];

            for q in 0..10 {
                questions.push(format!("{char}{f}.{q}"));
            }

            domain.subdomain.push(SubDomain {
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

    // println!("{:#?}", eval);

    // Build a 4 questions per page assessment
    let res: Assessment<4> = assessment.build();

    // println!("{:#?}", res);
}
