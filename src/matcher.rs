use nucleo::{Config, Matcher};
use crate::applications::Application;
use std::collections::HashMap;

pub fn resolve_same_score(app_1: &Application, app_2: &Application, query: &str) -> i32 {
	let app_1_name = app_1.name.to_lowercase();
	let app_2_name = app_2.name.to_lowercase();

	let split_1 = app_1_name.split_whitespace().collect::<Vec<&str>>();
	let split_2 = app_2_name.split_whitespace().collect::<Vec<&str>>();
	let query_lower = query.to_lowercase();

	let app_1_exact = split_1.iter().any(|&s| s == query_lower);
	let app_2_exact = split_2.iter().any(|&s| s == query_lower);

	if app_1_exact && !app_2_exact {
		1
	} else if app_2_exact && !app_1_exact {
		-1
	} else {
		// neither or both are exact matches, prioritise shorter name
		if app_1_name.len() < app_2_name.len() {
			1
		} else if app_2_name.len() < app_1_name.len() {
			-1
		} else {
			0
		}
	}
}

pub fn sort_applications(apps: &mut Vec<Application>, query: &str) -> Vec<(u16, usize)> {
	// TODO: improve sorting algorithm
	// TODO: fuzzy search the application type, and mime types too
	// 
       
        let mut matcher = Matcher::new(Config::DEFAULT);

        // Use a map score -> list of indices so we preserve all results
        let mut results: HashMap<u16, Vec<usize>> = HashMap::new();
        for (index, app) in apps.iter().enumerate() {
            // get score from fuzzy match
            if let Some(score) = matcher.fuzzy_match(
                nucleo::Utf32Str::new(&app.name.to_lowercase(), &mut Vec::new()),
                nucleo::Utf32Str::new(query, &mut Vec::new())
            ) {
                if results.contains_key(&score) {
                    // Compare current app against existing entries in this score bucket.
                    // If current clearly beats any existing entry, promote current to score+1.
                    // If an existing clearly beats current, promote that existing to score+1.
                    // If all comparisons are ties, keep both at the same score.

					// example
					// query = "zen"
					// results = { [88, zen browser], [88, zenity]}
					// in this case, zen browser should beat zenity, as it has a closer substring match
					// so we promote zen browser to 89, and keep zenity at 88
					// 
					// users don't want a "maybe" from multiple results, they want the best match at the top
					// considering language, if i type zen, i want something with exactly "zen" in the name to be at the top
					// even if zenity has the same fuzzy score, it's not as good a match

					// this method still ensures normal matching
					// i.e. if type "browser", multiple browsers with same score will be kept at same score as they are all equally relevant

                    let mut existing_beats_current = None;
                    let mut current_beats_existing = false;

					// get colliding scores
                    let bucket = results.get(&score).unwrap().clone();
                    for &existing_index in bucket.iter() {
                        let res = resolve_same_score(&apps[existing_index], app, query);
                        if res > 0 {
                            // existing is better than current
                            existing_beats_current = Some(existing_index);
                            break;
                        } else if res < 0 {
                            // current is better than at least one existing
                            current_beats_existing = true;
                        }
                    }

                    if current_beats_existing {
                        // promote current to score + 1
                        results.entry(score + 1).or_default().push(index);
                    } else if let Some(best_existing) = existing_beats_current {
                        // promote the existing winner to score + 1, keep current at this score
                        // remove best_existing from this bucket
                        results.get_mut(&score).unwrap().retain(|&i| i != best_existing);
                        results.entry(score + 1).or_default().push(best_existing);
                        results.get_mut(&score).unwrap().push(index);
                    } else {
                        // all ties -> keep both at same score
                        results.get_mut(&score).unwrap().push(index);
                    }
                } else {
                    // no collision, insert normally
                    results.insert(score, vec![index]);
                }
            }
        }

        // flatten into Vec<(score, index)>
        let mut output: Vec<(u16, usize)> = Vec::new();
        for (score, idxs) in results {
            for idx in idxs {
                output.push((score, idx));
            }
        }

        output.sort_by(|a, b| b.0.cmp(&a.0));

       
        output
    }

#[cfg(test)]
mod tests {
	use crate::applications::find_desktop_files;

use super::*;

	#[test]
	fn test_sort_applications() {
		let now = std::time::Instant::now();
		let query = "zen";
		let apps = find_desktop_files();

		let mut apps_clone = apps.clone();
		let sorted = sort_applications(&mut apps_clone, query);
		assert!(!sorted.is_empty());
		println!("Sorted {} applications in {:?}", apps.len(), now.elapsed());


		let mut i = 1;
		for (score, idx) in sorted {
			println!("Score: {}, App: {:?}", score, apps[idx].name);
			if i >= 10 {
				break;
			}
			i += 1;
		}
		
	}
}