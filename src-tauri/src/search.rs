use glob::Pattern;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Mutex};
use tauri::State;

use crate::AppState;

#[derive(Serialize, Deserialize)]
pub enum SearchType {
    Fuzzy,
    Regex,
    Prefix,
    Glob,
}

#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    query: String,
    search_type: SearchType,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    matches: Vec<String>,
    scores: Vec<f64>, // Optional: scores for ranking (useful for fuzzy matching)
}

impl SearchResult {
    pub fn merge(&mut self, other: SearchResult) {
        self.matches.extend(other.matches);
        self.scores.extend(other.scores);
    }

    pub fn merge_sorted(&mut self, other: SearchResult) {
        // Combine matches and scores into a vector of tuples
        let mut combined: Vec<(String, f64)> = self
            .matches
            .drain(..)
            .zip(self.scores.drain(..))
            .chain(other.matches.into_iter().zip(other.scores.into_iter()))
            .collect();

        // Sort by score in descending order
        combined.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Unzip back into matches and scores
        let (matches, scores): (Vec<String>, Vec<f64>) = combined.into_iter().unzip();
        self.matches = matches;
        self.scores = scores;
    }
}

#[tauri::command]
pub fn autodetect_search(state: State<Mutex<AppState>>, query: String) -> SearchResult {
    let state = state.lock().unwrap();
    let items: Vec<String> = state.std_in.clone();

    if query.starts_with("/") && query.ends_with("/") && query.len() > 1 {
        let regex_query = &query[1..query.len() - 1]; // Extract the pattern between the slashes
        if is_valid_regex(regex_query) {
            regex_search(regex_query, &items)
        } else {
            SearchResult {
                matches: Vec::new(),
                scores: Vec::new(),
            }
        }
    } else if is_glob_like(&query) {
        glob_search(&query, &items)
    } else {
        let mut pref = prefix_search(&query, &items);
        let threshold = state.config.clone().fuzzy_threshold;
        let fuzz = fuzzy_search(&query.to_string(), &items, threshold);

        pref.merge_sorted(fuzz);
        return pref;
    }
}

pub fn is_valid_regex(pattern: &str) -> bool {
    Regex::new(pattern).is_ok()
}

fn is_glob_like(query: &str) -> bool {
    query.contains("*") || query.contains("?") || query.contains("[")
}

// pub fn search(request: SearchRequest) -> SearchResult {
//     match request.search_type {
//         SearchType::Fuzzy => fuzzy_search(&request.query, &request.items, 0.25),
//         SearchType::Regex => regex_search(&request.query, &request.items),
//         SearchType::Prefix => prefix_search(&request.query, &request.items),
//         SearchType::Glob => glob_search(&request.query, &request.items),
//     }
// }

fn convert_string_slice_to_str_slice(input: &[String]) -> Vec<&str> {
    let mut result = Vec::new();
    for string_ref in input {
        result.push(string_ref.as_str());
    }
    result
}

fn fuzzy_search(query: &str, items: &[String], threshold: f32) -> SearchResult {
    use rust_fuzzy_search::fuzzy_search_threshold;

    let str_items: Vec<&str> = convert_string_slice_to_str_slice(items);

    let res = fuzzy_search_threshold(query, &str_items, threshold);

    let mut matches = Vec::new();
    let mut scores = Vec::new();

    for (word, score) in res {
        matches.push(word.to_string());
        scores.push(score as f64);
    }

    SearchResult { matches, scores }
}

fn regex_search(query: &str, items: &[String]) -> SearchResult {
    let regex = match Regex::new(query) {
        Ok(regex) => regex,
        Err(_) => {
            return SearchResult {
                matches: Vec::new(),
                scores: Vec::new(),
            }
        }
    };

    let matches: Vec<String> = items
        .iter()
        .filter(|item| regex.is_match(item))
        .cloned()
        .collect();
    let scores = vec![1.0; matches.len()]; // No scoring for regex

    SearchResult { matches, scores }
}

fn prefix_search(query: &str, items: &[String]) -> SearchResult {
    let matches: Vec<String> = items
        .iter()
        .filter(|item| item.starts_with(query))
        .cloned()
        .collect();
    let scores = vec![1.0; matches.len()]; // No scoring for prefix

    SearchResult { matches, scores }
}

fn glob_search(query: &str, items: &[String]) -> SearchResult {
    let pattern = match Pattern::new(query) {
        Ok(pattern) => pattern,
        Err(_) => {
            return SearchResult {
                matches: Vec::new(),
                scores: Vec::new(),
            }
        }
    };

    let matches: Vec<String> = items
        .iter()
        .filter(|item| pattern.matches(item))
        .cloned()
        .collect();
    let scores = vec![1.0; matches.len()]; // No scoring for glob

    SearchResult { matches, scores }
}
