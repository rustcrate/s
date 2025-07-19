use s;

#[test]
fn test_num_cpu() {
    let cores = s::num_cpu();
    assert!(cores >= 1, "Should return at least 1 core");
}

#[test]
fn test_num_cpu_is_cached() {
    let cores1 = s::num_cpu();
    let cores2 = s::num_cpu();
    assert_eq!(cores1, cores2, "Subsequent calls should return the same value");
}

#[test]
fn test_time() {
    let t1 = s::time();
    let t2 = s::time();
    assert!(t2 >= t1, "Time should move forward");
}
