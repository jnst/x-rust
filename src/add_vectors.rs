fn add_vectors(vector1: &[i32], vector2: &[i32]) -> Result<Vec<i32>, String> {
    if vector1.len() != vector2.len() {
        return Err("invalid argument".to_string());
    }

    let result: Vec<i32> = vector1
        .iter()
        .zip(vector2)
        .map(|(&x, &y)| x + y)
        .collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vectors() {
        let vector1 = &[1, 2, 3, 4];
        let vector2 = &[5, 6, 7, 8];
        assert_eq!(add_vectors(vector1, vector2), Ok(vec![6, 8, 10, 12]));
    }
}
