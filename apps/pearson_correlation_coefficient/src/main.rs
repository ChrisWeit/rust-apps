/**
 * Example input: 
 * 10 8  20 3  2  20 32 20 18
 * 7  19 18 21 15 16 22 27 19
 * 
 * expected: r=0.36757
 */ 

fn main() {
    let a = string_to_int(get_input());
    let b = string_to_int(get_input());
    
    let pcc = compute_pcc(&a,&b);
    println!("Pearson's correlation coefficient r={:.5}", pcc); 
}

fn get_input() -> String {
    let mut buffer = String::new();
    println!("Enter a set of scores (divided by \" \"):");
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    return buffer;
}

fn string_to_int(string: String) -> Vec<i32> {
    return string
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
}

fn compute_pcc(phy_scores: &Vec<i32>, hist_scores: &Vec<i32>) -> f64 {
    let centered_phy: Vec<f64> = center_values(&phy_scores);
    let centered_hist: Vec<f64> = center_values(&hist_scores);
    
    let numerator: f64 = sum_product(&centered_phy, &centered_hist);
    
    let sqrt_phy: f64 = sum_squares(&centered_phy).sqrt();
    let sqrt_hist: f64 = sum_squares(&centered_hist).sqrt();
    
    return numerator / (sqrt_phy*sqrt_hist);
}

fn sum_product(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    return a.iter()
        .zip(b.iter())
        .map(|(a,b)| a * b)
        .sum::<f64>();
}

fn sum_squares(list: &Vec<f64>) -> f64 {
    return list.iter()
        .map(|x| x*x)
        .sum::<f64>();
} 

fn center_values(list: &Vec<i32>) -> Vec<f64> {
    let mean: f64 = mean(list);
    return list.iter()
        .map(|x| f64::from(*x) - mean)
        .collect();
}

fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = Iterator::sum(list.iter());
    return f64::from(sum) / (list.len() as f64);
}
