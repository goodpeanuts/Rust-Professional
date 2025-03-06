use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_file(filename: &str) -> BTreeMap<String, HashMap<String, Vec<String>>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut data = BTreeMap::new();
    let mut main_key = String::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.is_empty() || line.starts_with('{') {
            continue;
        }

        if line.contains(": {") {
            let (key, _) = line.split_once(':').unwrap();
            main_key = key.trim().trim_matches('"').to_string();
            data.insert(main_key.clone(), HashMap::new());
            continue;
        }
        if line.starts_with('}') {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim().trim_matches('"');
        let values_str = parts[1]
            .trim()
            .trim_matches(',')
            .trim_matches('[')
            .trim_matches(']')
            .trim_matches('"');
        let values: Vec<String> = if values_str.is_empty() {
            Vec::new()
        } else {
            values_str
                .split(',')
                .map(|v| v.trim().trim_matches('"').to_string())
                .collect()
        };

        let mm: &mut HashMap<String, Vec<String>> = data.get_mut(&main_key).unwrap();
        if mm.contains_key(key) {
            if let Some(map) = mm.get_mut(key) {
                map.extend(values);
            }
        } else {
            mm.insert(key.to_string(), values);
        }
    }
    data
}

pub fn count_provinces() -> String {
    let mut result = Vec::new();
    let data = parse_file("district.json");
    for item in data {
        let mut list: HashMap<String, Vec<String>> = HashMap::new();
        for (sub_item_key, mut sub_item_values) in item.1 {
            let tmp_vec = match list.get_mut(&sub_item_key) {
                Some(v) => v,
                None => &mut Vec::new(),
            };
            if tmp_vec.contains(&sub_item_key) {
                tmp_vec.extend(sub_item_values);
            } else {
                let mut found_on_keys = Vec::new();
                for (list_key, list_values) in list.iter_mut() {
                    if list_values.contains(&sub_item_key) {
                        found_on_keys.push(list_key.clone());
                    } else {
                        for kk in sub_item_values.iter() {
                            if list_values.contains(kk) {
                                found_on_keys.push(list_key.clone());
                                break;
                            }
                        }
                    }
                }

                if list.is_empty() || found_on_keys.is_empty() {
                    sub_item_values.push(sub_item_key.to_string());
                    list.insert(sub_item_key, sub_item_values);
                    continue;
                }

                let mut tmp_v = vec![];
                tmp_v.push(sub_item_key.to_string());
                tmp_v.extend(sub_item_values);
                for key in found_on_keys {
                    let t = list.remove(&key).unwrap();
                    tmp_v.extend(t);
                }
                list.insert(tmp_v[0].clone(), tmp_v);
            }
        }
        result.push(format!("{}", list.len()));
    }
    result.join(",")
}
