use bunny_cdn_wrapper::{add, BunnyStorage};
use tokio::runtime::{Builder, Runtime};

fn rt() -> Runtime {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create runtime")
}

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn test_list() {
    let rt = rt();

    let bunny_storage = BunnyStorage::new(
        "collegekingsstorage",
        "ceb70a44-9da3-43c8-acbd327c632a-c24d-4ee7",
        "de",
    )
    .unwrap();

    let files = rt
        .block_on(bunny_storage.list(&format!(
            "__bcdn_perma_cache__/pullzone__collegekings__22373407/wp-content/uploads/secured/{}/",
            "college_kings_2"
        )))
        .unwrap();
    println!("{:?}", files);
}
