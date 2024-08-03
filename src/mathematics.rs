use crate::series::Series;

fn change(new: f64, old: f64) -> f64 {
    (new - old) / old
}

pub fn changes(series: &Series) -> Vec<f64> {
    let mut vector: Vec<f64> = Vec::new();
    for index in 1..series.get_series().len() {
        vector.push(change(
            series.get_series()[index].get_close(),
            series.get_series()[index - 1].get_close(),
        ));
    }
    vector
}

fn mean(vector: &[f64]) -> f64 {
    vector.iter().sum::<f64>() / vector.len() as f64
}

pub fn mean_standard_deviation_ratio(vector: &[f64]) -> f64 {
    let mean = mean(vector);
    mean / standard_deviation(vector, mean)
}

fn standard_deviation(vector: &[f64], _mean: f64) -> f64 {
    let vector: Vec<f64> = vector.iter().map(|x| (x - _mean).powi(2)).collect();
    mean(&vector).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn made_change() {
        assert_eq!(change(1.0, 2.0), -0.5)
    }

    #[test]
    fn made_mean() {
        let vector: Vec<f64> = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert_eq!(mean(&vector), 5.0);
    }

    #[test]
    fn made_ratio() {
        let vector: Vec<f64> = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert_eq!(mean_standard_deviation_ratio(&vector), 2.5);
    }

    #[test]
    fn made_standard_deviation() {
        let vector: Vec<f64> = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert_eq!(standard_deviation(&vector, mean(&vector)), 2.0);
    }
}
