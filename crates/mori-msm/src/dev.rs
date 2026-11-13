// SPDX-License-Identifier: Apache-2.0
//! Device handle: context, stream and the loaded PTX module.
use std::sync::{Arc, OnceLock};

use cudarc::driver::{CudaContext, CudaFunction, CudaModule, CudaStream};
use cudarc::nvrtc::Ptx;

pub const PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/mori_msm_kernel.ptx"));
/// SASS built by ptxas for `MORI_MSM_ARCH` (sm_121 by default) at build time.
#[cfg(mori_msm_cubin)]
pub const CUBIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/mori_msm_kernel.cubin"));

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
        // MORI_MSM_CUBIN=<file> loads another module (register-limit sweeps); else the
        // built-in cubin when it matches this device, else the PTX through the driver JIT.
        if let Some(path) = std::env::var_os("MORI_MSM_CUBIN") {
            let module = ctx
                .load_module(Ptx::from_binary(std::fs::read(path).ok()?))
                .ok()?;
            return Some(Dev {
                ctx,
                stream,
                module,
            });
        }
        #[cfg(mori_msm_cubin)]
        let module = ctx
            .load_module(Ptx::from_binary(CUBIN.to_vec()))
            .or_else(|_| ctx.load_module(Ptx::from_src(PTX)))
            .ok()?;
        #[cfg(not(mori_msm_cubin))]
        let module = ctx.load_module(Ptx::from_src(PTX)).ok()?;
        Some(Dev {
            ctx,
            stream,
            module,
        })
    })
    .as_ref()
}
