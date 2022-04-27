pub fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}
