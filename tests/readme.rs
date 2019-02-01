use alloc_counter::*;

#[global_allocator]
static A: AllocCounterSystem = ALLOC_COUNTER_SYSTEM;

#[test]
#[should_panic]
fn type_check() {
    let ((_allocs, _reallocs, _deallocs), _value) = count_alloc(|| Box::new(0));

    // allowed
    allow_alloc(|| {
        Box::new(0);
    });

    // denied
    deny_alloc(|| {
        Box::new(0);
    });

    // allowed
    deny_alloc(|| {
        allow_alloc(|| {
            Box::new(0);
        });
    });

    // forbidden
    forbid_alloc(|| {
        allow_alloc(|| {
            Box::new(0);
        });
    });
}
