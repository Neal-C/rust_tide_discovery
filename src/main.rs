use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize)]
struct GetNameQueryParams {
    pub name: String,
}

async fn handle_get_name_with_path_param(request: tide::Request<State>) -> tide::Result<String> {
    let name = request
        .param("name")
        .unwrap_or("lazy ass who can't pass a param");
    Ok(format!("Hi {name}"))
}
async fn handle_get_name(request: tide::Request<State>) -> tide::Result<String> {
    let name: String = request.query::<GetNameQueryParams>()?.name;
    Ok(format!(
        "Hey {name} You should hire me because I know how to reverse a string and do fizzbuzz"
    ))
}

#[derive(Deserialize, Serialize)]
struct Candidate {
    name: String,
    salary_expectations: u128,
}

async fn handle_create_candidate(mut request: tide::Request<State>) -> tide::Result<String> {
    let Candidate {
        name,
        salary_expectations,
    } = request.body_json().await?;
    let app_state_like_a_db = request.state();
    {
        let mut repository = app_state_like_a_db.write().await;
        repository.candidates.insert(
            name.clone(),
            Candidate {
                name: name.clone(),
                salary_expectations,
            },
        );
    }
    Ok(format!("Succesfully created {name} who has the shocking expectation of {salary_expectations}K per year! We can't hire such spoiled kids !!!"))
}

async fn handle_read_all_candidates(
    _request: tide::Request<State>,
) -> tide::Result<tide::Response> {
    let candidates = vec![
        Candidate {
            name: String::from("Sanji"),
            salary_expectations: 140_000,
        },
        Candidate {
            name: String::from("Zoro"),
            salary_expectations: 160_000,
        },
        Candidate {
            name: String::from("Luffy"),
            salary_expectations: 500_000_000,
        },
    ];

    let Ok(candidates) = tide::Body::from_json(&candidates) else {
        return Ok(tide::Response::new(tide::StatusCode::InternalServerError));
    };
    let response = tide::Response::builder(tide::StatusCode::Ok)
        .body(candidates)
        .build();
    // Can't send a response && interact with cookies
    // it's one or the other
    Ok(response)
}

struct SharedStateLikeADB {
    candidates: HashMap<String, Candidate>,
}

impl SharedStateLikeADB {
    fn new() -> Self {
        Self {
            // Strong typing lets the compiler know the types of Key & Value
            candidates: HashMap::new(),
        }
    }
}

type State = Arc<async_std::sync::RwLock<SharedStateLikeADB>>;

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut tide_app = tide::new();

    tide_app.with(tide::log::LogMiddleware::new());

    tide_app.at("/api/v1").nest({
        let app_state_like_a_db = Arc::new(async_std::sync::RwLock::new(SharedStateLikeADB::new()));
        let mut candidate_api = tide::with_state(app_state_like_a_db);

        candidate_api.at("/candidate").get(handle_get_name);

        candidate_api
            .at("/:name")
            .get(handle_get_name_with_path_param);

        candidate_api.at("/candidate").post(handle_create_candidate);

        candidate_api
            .at("/canditate")
            .get(handle_read_all_candidates);

        candidate_api
    });

    tide_app.listen("127.0.0.1:8000").await?;

    println!("Hire me or just send me a technical test straight away");
    Ok(())
}
