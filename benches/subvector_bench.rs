#[macro_use]
extern crate bencher;

use bencher::Bencher;
use subvector::test_inputs::*;
use subvector::*;

fn vecs_short_input(bench: &mut Bencher) {
    bench.iter(|| {
        assert_eq!(
            Some(EXPECTED_SHORT),
            identify_subvector_vecs(TEST_STR_SHORT)
        );
    });
}

fn vecs_medium_input(bench: &mut Bencher) {
    bench.iter(|| {
        assert_eq!(
            Some(EXPECTED_SHORT),
            identify_subvector_vecs(TEST_STR_SHORT)
        );
    });
}

fn vecs_long_input(bench: &mut Bencher) {
    bench.iter(|| {
        assert_eq!(
            Some(EXPECTED_SHORT),
            identify_subvector_vecs(TEST_STR_SHORT)
        );
    });
}

fn slow_short_input(bench: &mut Bencher) {
    bench.iter(|| {
        assert_eq!(
            Some(EXPECTED_SHORT),
            identify_subvector_slow(TEST_STR_SHORT)
        );
    });
}

fn fast_short_input(bench: &mut Bencher) {
    bench.iter(|| {
        assert_eq!(
            Some(EXPECTED_SHORT),
            identify_subvector_fast(TEST_STR_SHORT)
        );
    });
}

fn slow_medium_input(bench: &mut Bencher) {
    bench.iter(|| {
        assert_eq!(
            Some(EXPECTED_MEDIUM),
            identify_subvector_slow(TEST_STR_MEDIUM)
        );
    });
}

fn fast_medium_input(bench: &mut Bencher) {
    bench.iter(|| {
        assert_eq!(
            Some(EXPECTED_MEDIUM),
            identify_subvector_fast(TEST_STR_MEDIUM)
        );
    });
}

fn slow_long_input(bench: &mut Bencher) {
    bench.iter(|| {
        assert_eq!(Some(EXPECTED_LONG), identify_subvector_slow(TEST_STR_LONG));
    });
}

fn fast_long_input(bench: &mut Bencher) {
    bench.iter(|| {
        assert_eq!(Some(EXPECTED_LONG), identify_subvector_fast(TEST_STR_LONG));
    });
}

benchmark_group!(
    benches,
    slow_short_input,
    fast_short_input,
    slow_medium_input,
    fast_medium_input,
    slow_long_input,
    fast_long_input,
    vecs_short_input,
    vecs_medium_input,
    vecs_long_input,
);
benchmark_main!(benches);
