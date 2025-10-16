# Known Issues

## GitHub Actions Clippy Failure Due to `size-of` Crate

### Issue
The GitHub Actions Clippy workflow fails on Linux x86_64 with ABI errors from the `size-of v0.1.5` crate:

```
error[E0570]: "aapcs" is not a supported ABI for the current target
error[E0570]: "stdcall" is not a supported ABI for the current target
error[E0570]: "fastcall" is not a supported ABI for the current target
```

### Root Cause
- `size-of v0.1.5` is a transitive dependency from `starknet-types-core v0.1.9`
- The crate uses platform-specific ABIs (`aapcs`, `stdcall`, `fastcall`) that were previously accepted but are now hard errors in Rust 1.84+
- These ABIs are only valid on specific architectures (ARM for aapcs, x86-32 for stdcall/fastcall)
- See Rust issue: https://github.com/rust-lang/rust/issues/130260

### Impact
- **Local Development**: Works fine (warnings only)
- **macOS/ARM**: Works fine
- **Linux x86_64 CI**: Fails to compile

### Solutions

#### Option 1: Wait for Upstream Fix
The starknet team needs to update `starknet-types-core` to use a newer version of `size-of` or remove the dependency.

#### Option 2: Pin Rust Version (Temporary Workaround)
Modify `.github/workflows/clippy.yml` to use Rust 1.83.0 or earlier:

```yaml
- name: Setup Rust
  uses: actions-rust-lang/setup-rust-toolchain@v1
  with:
    toolchain: 1.83.0  # Before ABI errors became hard errors
    components: clippy
```

#### Option 3: Allow Deprecated ABIs in CI (Not Recommended)
This requires editing the workflow to set RUSTFLAGS, but may mask other issues.

### Current Status
**CI workflows temporarily disabled** due to this issue.

The problem affects both old and new Rust versions:
- Rust 1.83.0 and earlier: Can't compile base64ct v1.8.0 (requires edition 2024)
- Rust 1.84.0 and later: Can't compile size-of v0.1.5 (ABI hard errors)

Even patching to the git version of size-of doesn't resolve the issue - the crate still uses unsupported ABIs.

### Recommended Action
Local development works fine. CI is temporarily disabled until upstream fixes land.

### Tracking
- Rust issue: https://github.com/rust-lang/rust/issues/130260
- size-of repo: https://github.com/Kixiron/size-of
- starknet-rs repo: https://github.com/xJonathanLEI/starknet-rs

Last updated: 2025-10-16
