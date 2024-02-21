#![no_main]

use hkt_primitives::contract::ContractCode;
use hkt_primitives::runtime::fees::RuntimeFeesConfig;
use hkt_primitives::version::PROTOCOL_VERSION;
use hkt_vm_logic::mocks::mock_external::MockedExternal;
use hkt_vm_logic::VMConfig;
use hkt_vm_runner::internal::VMKind;
use hkt_vm_runner::VMResult;
use hkt_vm_runner_fuzz::{create_context, find_entry_point, ArbitraryModule};

libfuzzer_sys::fuzz_target!(|module: ArbitraryModule| {
    let code = ContractCode::new(module.0.module.to_bytes(), None);
    let _result = run_fuzz(&code, VMKind::for_protocol_version(PROTOCOL_VERSION));
});

fn run_fuzz(code: &ContractCode, vm_kind: VMKind) -> VMResult {
    let mut fake_external = MockedExternal::new();
    let mut context = create_context(vec![]);
    context.prepaid_gas = 10u64.pow(14);
    let mut config = VMConfig::test();
    config.limit_config.wasmer2_stack_limit = i32::MAX; // If we can crash wasmer2 even without the secondary stack limit it's still good to know
    let fees = RuntimeFeesConfig::test();

    let promise_results = vec![];

    let method_name = find_entry_point(code).unwrap_or_else(|| "main".to_string());
    vm_kind.runtime(config).unwrap().run(
        code,
        &method_name,
        &mut fake_external,
        context,
        &fees,
        &promise_results,
        PROTOCOL_VERSION,
        None,
    )
}
