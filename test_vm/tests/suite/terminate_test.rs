use fil_actors_integration_tests::tests::terminate_sectors_test;
use fvm_ipld_blockstore::MemoryBlockstore;
use test_vm::TestVM;

#[test]
fn terminate_sectors() {
    let store = MemoryBlockstore::new();
    let v = TestVM::<MemoryBlockstore>::new_with_singletons(&store);
    terminate_sectors_test(&v);
}
