use std::collections::HashMap;

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() { return 0.0; }
    let idx = ((p.clamp(0.0, 100.0) / 100.0) * (sorted.len() as f64 - 1.0)).round() as usize;
    sorted[idx]
}

pub fn centrality_summary(values: &[f64]) -> HashMap<String, f64> {
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if v.is_empty() { return HashMap::new(); }
    let min = *v.first().unwrap();
    let max = *v.last().unwrap();
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    let mut out = HashMap::new();
    out.insert("min".to_string(), min);
    out.insert("mean".to_string(), mean);
    out.insert("max".to_string(), max);
    out.insert("p50".to_string(), percentile(&v, 50.0));
    out.insert("p75".to_string(), percentile(&v, 75.0));
    out.insert("p90".to_string(), percentile(&v, 90.0));
    out.insert("p95".to_string(), percentile(&v, 95.0));
    out.insert("p99".to_string(), percentile(&v, 99.0));
    out.insert("p999".to_string(), percentile(&v, 99.9));
    out
}

pub fn similarity_summary(values: &[f64]) -> HashMap<String, f64> {
    // includes stdDev and p100 like Java util
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if v.is_empty() { return HashMap::new(); }
    let min = *v.first().unwrap();
    let max = *v.last().unwrap();
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    let var = v.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / v.len() as f64;
    let std_dev = var.sqrt();
    let mut out = HashMap::new();
    out.insert("min".to_string(), min);
    out.insert("max".to_string(), max);
    out.insert("mean".to_string(), mean);
    out.insert("stdDev".to_string(), std_dev);
    fn key(p: f64) -> String { if (p - p.floor()).abs() < f64::EPSILON { format!("p{}", p as i32) } else { format!("p{}", p) } }
    for p in [1.0,5.0,10.0,25.0,50.0,75.0,90.0,95.0,99.0,100.0] { out.insert(key(p), percentile(&v, p)); }
    out
}

pub fn community_summary(values: &[u64]) -> HashMap<String, f64> {
    // Convert u64 histogram values into f64 stats akin to AbstractHistogram
    if values.is_empty() { return HashMap::new(); }
    let mut v: Vec<f64> = values.iter().map(|x| *x as f64).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min = *v.first().unwrap();
    let max = *v.last().unwrap();
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    let mut out = HashMap::new();
    out.insert("min".to_string(), min);
    out.insert("max".to_string(), max);
    out.insert("mean".to_string(), mean);
    fn key(p: f64) -> String { if (p - p.floor()).abs() < f64::EPSILON { format!("p{}", p as i32) } else { format!("p{}", p) } }
    for p in [1.0,5.0,10.0,25.0,50.0,75.0,90.0,95.0,99.0,99.9] { out.insert(key(p), percentile(&v, p)); }
    out
}


