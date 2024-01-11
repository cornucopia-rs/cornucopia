use cornucopia::{conn::cornucopia_conn, container::ContainerOpts, CodegenSettings};
use criterion::Criterion;

fn bench(c: &mut Criterion) {
    let container_opts = ContainerOpts::default();
    cornucopia::container::cleanup(&container_opts).ok();
    cornucopia::container::setup(&container_opts).unwrap();
    let client = &mut cornucopia_conn(&container_opts).unwrap();

    cornucopia::load_schema(client, &["../codegen_test/schema.sql"]).unwrap();
    c.bench_function("codegen_sync", |b| {
        b.iter(|| {
            cornucopia::generate_live(
                client,
                "../test_codegen/queries",
                None,
                CodegenSettings {
                    gen_sync: true,
                    gen_async: false,
                    derive_ser: true,
                },
            )
            .unwrap()
        })
    });
    c.bench_function("codegen_async", |b| {
        b.iter(|| {
            cornucopia::generate_live(
                client,
                "../test_codegen/queries",
                None,
                CodegenSettings {
                    gen_sync: true,
                    gen_async: false,
                    derive_ser: true,
                },
            )
            .unwrap()
        })
    });
    cornucopia::container::cleanup(&container_opts).unwrap();
}
criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
