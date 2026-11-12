// SPDX-License-Identifier: Apache-2.0
//! Device handle: context, stream and the loaded PTX module.
use std::sync::{Arc, OnceLock};

use cudarc::driver::{CudaContext, CudaFunction, CudaModule, CudaStream};
use cudarc::nvrtc::Ptx;

pub const PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/mori_msm_kernel.ptx"));

pub struct Dev {
    pub ctx: Arc<CudaContext>,
    pub stream: Arc<CudaStream>,
    pub module: Arc<CudaModule>,
}

impl Dev {
    pub fn func(&self, name: &str) -> CudaFunction {
        self.module.load_function(name).expect(name)
    }
}

pub fn dev() -> Option<&'static Dev> {
    static D: OnceLock<Option<Dev>> = OnceLock::new();
    D.get_or_init(|| {
        let ctx = CudaContext::new(0).ok()?;
        let stream = ctx.default_stream();
        let module = ctx.load_module(Ptx::from_src(PTX)).ok()?;
        Some(Dev {
            ctx,
            stream,
            module,
        })
    })
    .as_ref()
}
