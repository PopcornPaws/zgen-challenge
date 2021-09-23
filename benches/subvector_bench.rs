#[macro_use]
extern crate bencher;

use bencher::Bencher;
use subvector::test_inputs::*;

macro_rules! add_bench_functions { 
    ( $( $module:ident ),* ) => {
        mod short {
            use super::*;
        $(
            pub fn $module(bench: &mut Bencher) {
                bench.iter(|| {
                    assert_eq!(
                        Some(EXPECTED_SHORT),
                        subvector::$module::identify_subvector(TEST_STR_SHORT)
                    );
                });
            }
        )*
        }

        mod medium {
            use super::*;
        $(
            pub fn $module(bench: &mut Bencher) {
                bench.iter(|| {
                    assert_eq!(
                        Some(EXPECTED_MEDIUM),
                        subvector::$module::identify_subvector(TEST_STR_MEDIUM)
                    );
                });
            }
        )*
        }

        mod long {
            use super::*;
        $(
            pub fn $module(bench: &mut Bencher) {
                bench.iter(|| {
                    assert_eq!(
                        Some(EXPECTED_LONG),
                        subvector::$module::identify_subvector(TEST_STR_LONG)
                    );
                });
            }
        )*
        }

        // TODO not supported yet
        //benchmark_group!(
        //    benches,
        //    $( $short_idents ),*
        //    $( $medium_idents ),*
        //    $( $long_idents ),*
        //);
    };
}

add_bench_functions!(hashmap, quadratic, recursive, vecs);

benchmark_group!(
    benches,
    short::hashmap,
    short::quadratic,
    //short::recursive,
    short::vecs,
    medium::hashmap,
    medium::quadratic,
    //medium::recursive,
    medium::vecs,
    long::hashmap,
    long::quadratic,
    //long::recursive,
    long::vecs,
);

benchmark_main!(benches);
