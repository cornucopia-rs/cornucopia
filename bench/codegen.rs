use cornucopia::{conn::cornucopia_conn, CodegenSettings};
use criterion::Criterion;

fn bench(c: &mut Criterion) {
    cornucopia::container::cleanup(false).ok();
    cornucopia::container::setup(false).unwrap();
    let client = &mut cornucopia_conn().unwrap();

    cornucopia::load_schema(client, vec!["../codegen_test/schema.sql".into()]).unwrap();
    c.bench_function("codegen_sync", |b| {
        b.iter(|| {
            cornucopia::generate_live(
                client,
                "../codegen_test/queries",
                None,
                CodegenSettings {
                    is_async: false,
                    derive_ser: true,
                    is_recursive: false,
                },
            )
            .unwrap()
        })
    });
    c.bench_function("codegen_async", |b| {
        b.iter(|| {
            cornucopia::generate_live(
                client,
                "../codegen_test/queries",
                None,
                CodegenSettings {
                    is_async: true,
                    derive_ser: true,
                    is_recursive: false,
                },
            )
            .unwrap()
        })
    });
    cornucopia::container::cleanup(false).unwrap();
}
criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
