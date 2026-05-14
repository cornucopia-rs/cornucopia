use std::{path::PathBuf, str::FromStr};

use cornucopia::{config::Config, conn::cornucopia_conn};
use criterion::Criterion;

fn bench(c: &mut Criterion) {
    cornucopia::container::setup(false, "docker.io/library/postgres:latest", 250).unwrap();
    let client = &cornucopia_conn().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    cornucopia::load_schema(client, &["tests/codegen/schema.sql"]).unwrap();

    let cfg = Config::builder()
        .queries(PathBuf::from_str("tests/codegen/queries").unwrap())
        .destination(tmp.keep())
        .sync(true)
        .r#async(true)
        .derive_traits(vec!["serde::Serialize".to_string()]);

    c.bench_function("codegen_sync", |b| {
        b.iter(|| cornucopia::gen_live(client, cfg.clone().build()).unwrap())
    });

    let cfg = cfg.sync(false).r#async(false);

    c.bench_function("codegen_async", |b| {
        b.iter(|| cornucopia::gen_live(client, cfg.clone().build()).unwrap())
    });

    cornucopia::container::cleanup(false).unwrap();
}
criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
