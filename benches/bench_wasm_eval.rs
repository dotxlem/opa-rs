use criterion::{black_box, criterion_group, criterion_main, Criterion};
use opa::{bundle::Bundle, wasm::Opa};
use serde_json::{json, Value};

fn criterion_benchmark(c: &mut Criterion) {
    let mut bundle =
        Bundle::from_bytes(include_bytes!("../examples/wasm_bundle/example.tar.gz")).unwrap();

    let mut opa = Opa::new()
        .build(bundle.wasm_policies.pop().unwrap().bytes)
        .unwrap();

    let data = json!({
        "users": {
            "test": {
                "projects": {
                    "test": {
                        "roles": ["owner"]
                    }
                }
            }
        },
        "projects": {
            "test": {}
        }
    });

    let input = json!({
        "user_id": "test",
        "project_id": "test",
    });

    opa.set_data(&data).unwrap();

    c.bench_function("eval_project_permissions", |b| {
        b.iter(|| {
            opa.eval::<_, Value>(black_box("example.project_permissions"), black_box(&input))
                .unwrap()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
