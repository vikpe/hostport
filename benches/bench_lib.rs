use criterion::{Criterion, criterion_group, criterion_main};

fn lib_benchmark(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("validate::is_valid_host");
        let f = hostport::validate::is_valid_host;
        group.bench_function("invalid alias/domain", |b| b.iter(|| f("foo.c%m:28000")));
        group.bench_function("invalid ip", |b| b.iter(|| f("1000.0.0.0:28000")));

        group.bench_function("alias", |b| b.iter(|| f("localhost:28000")));
        group.bench_function("domain", |b| b.iter(|| f("quake-world.com:28000")));
        group.bench_function("ip", |b| b.iter(|| f("10.10.10.10:28000")));
        group.finish();
    }
}

criterion_group!(benches, lib_benchmark);
criterion_main!(benches);
