use clap::Parser;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use scraper::Html;
use serde::Deserialize;
use serde_json::json;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

/// A tool to fetch a LeetCode problem and create a Rust stub in src/solution.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
  /// LeetCode problem link (e.g. https://leetcode.com/problems/count-total-number-of-colored-cells/)
  #[arg(long)]
  link: String,
}

fn extract_slug_from_link(link: &str) -> Result<String, String> {
  // Example: https://leetcode.com/problems/count-total-number-of-colored-cells/
  let re = Regex::new(r"leetcode\.com/problems/([^/]+)/?").unwrap();
  if let Some(caps) = re.captures(link) {
    Ok(caps.get(1).unwrap().as_str().to_string())
  } else {
    Err("Invalid LeetCode problem URL.".to_string())
  }
}

#[derive(Deserialize)]
struct GraphqlResponse {
  data: Option<QuestionData>,
  errors: Option<Vec<GraphqlError>>,
}

#[derive(Deserialize)]
struct QuestionData {
  question: Option<Question>,
}

#[derive(Deserialize)]
struct GraphqlError {
  message: String,
}

#[derive(Deserialize)]
struct Question {
  #[serde(rename = "questionId")]
  question_id: String,
  title: String,
  content: String,
  difficulty: String,
  #[serde(rename = "exampleTestcasesx")]
  example_test_cases: Option<String>,
}

fn fetch_problem_details(slug: &str) -> Result<Question, String> {
  let query = r#"
    query questionData($titleSlug: String!) {
      question(titleSlug: $titleSlug) {
        questionId

        title
        content
        difficulty
        exampleTestcases
      }
    }
    "#;
  let variables = json!({ "titleSlug": slug });
  let req_body = json!({
      "query": query,
      "variables": variables
  });

  let client = Client::new();
  let res = client
    .post("https://leetcode.com/graphql")
    .header(CONTENT_TYPE, "application/json")
    .header(USER_AGENT, "Mozilla/5.0")
    .json(&req_body)
    .send()
    .map_err(|e| format!("HTTP request failed: {}", e))?;
  let graphql_resp: GraphqlResponse = res
    .json()
    .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

  if let Some(data) = graphql_resp.data {
    if let Some(question) = data.question {
      Ok(question)
    } else {
      Err("Question data not found in GraphQL response.".to_string())
    }
  } else {
    // print error message
    let mut error_messages: String = "".to_string();
    if let Some(errors) = graphql_resp.errors {
      for error in &errors {
        error_messages += &error.message.to_string();
      }
    }

    Err(error_messages)
  }
}

fn html_to_text(html: &str) -> String {
  // Use scraper crate to parse the HTML and extract text.
  let fragment = Html::parse_fragment(html);
  // The Selector for all tags is not necessary since we simply want
  // to join all text nodes together.
  fragment.root_element().text().collect::<Vec<_>>().join(" ")
}

fn slug_to_filename(slug: &str) -> String {
  format!("{}.rs", slug.replace("-", "_"))
}

fn slug_to_modulename(slug: &str) -> String {
  slug.replace("-", "_").to_string()
}

fn slug_to_function_name(slug: &str) -> String {
  // Use the same transformation as for file name.
  slug.replace("-", "_")
}

fn main() {
  let args = Args::parse();

  let slug = match extract_slug_from_link(&args.link) {
    Ok(s) => s,
    Err(err) => {
      eprintln!("Error extracting slug: {}", err);
      return;
    }
  };

  let question = match fetch_problem_details(&slug) {
    Ok(q) => q,
    Err(err) => {
      eprintln!("Error fetching problem details: {}", err);
      return;
    }
  };

  // Convert HTML description to plain text and replace (Unicode NBSP) with normal space.
  let _description = html_to_text(&question.content).replace('\u{a0}', " ");
  let _examples = question.example_test_cases.unwrap_or_default();

  let header = format!(
    "/**
LeetCode Problem {}: {}
Link: {}
Difficulty: {}
**/
",
    question.question_id, question.title, args.link, question.difficulty);

  let fn_name = slug_to_function_name(&slug);
  let function_stub = format!(
    "pub fn {}() {{
    // TODO: Implement the solution.
}}
",
    fn_name
  );

  let file_content = format!("{}\n{}", header, function_stub);

  // Determine the output file path: src/solution/<filename>
  let output_dir = Path::new("src").join("solution");
  if let Err(e) = create_dir_all(&output_dir) {
    eprintln!("Error creating directory {:?}: {}", output_dir, e);
    return;
  }
  let filename = slug_to_filename(&slug);
  let file_path = output_dir.join(filename);

  if file_path.exists() {
    eprintln!(
      "File {:?} already exists. Aborting to prevent overwrite.",
      file_path
    );
    return;
  }

  match File::create(&file_path) {
    Ok(mut file) => {
      if let Err(e) = file.write_all(file_content.as_bytes()) {
        eprintln!("Error writing to file {:?}: {}", file_path, e);
      } else {
        println!("Generated file {:?}", file_path);
      }
    }
    Err(e) => eprintln!("Error creating file {:?}: {}", file_path, e),
  }

  // append the file name to mod.rs
  let mod_path = Path::new("src").join("solution").join("mod.rs");
  let mut mod_file = match File::options().append(true).open(&mod_path) {
    Ok(f) => f,
    Err(e) => {
      eprintln!("Error opening mod.rs: {}", e);
      return;
    }
  };

  // Append the new file name to mod.rs
  if let Err(e) = writeln!(mod_file, "pub mod {};", slug_to_modulename(&slug)) {
    eprintln!("Error writing to mod.rs: {}", e);
  }
}
