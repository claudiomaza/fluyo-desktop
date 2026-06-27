# COMPILATION_LOG - FLUYO

## Fixes Applied
1. **Initialization Fix**: Replaced the basic entry point with the cm2labs Instance Manager logic.
2. **Vault (Bóveda) Requirement**:
   - The app creates a `storage/` directory relative to the executable.
   - Isolated profiles in `storage/instances/`.
3. **UI Update**: Overwrote `ui/index.html` with the Instance Launcher UI.
4. **Tauri Config Update**:
   - Set `csp: null`.
   - Set `windows[0].url: "index.html"`.
5. **NSIS Nesting Fix**: Configured NSIS for `currentUser` install mode.

## Deployment Protocol v1.0 (2026-06-27)
- **Workflow Standardization**: Renamed to `cm2labs-fluyo-deployment.yml`.
- **Standardized Build Logic**: Using `tauri-apps/tauri-action@v0` for stability.
- **Artifact Management**: Explicitly generating and uploading:
  1. `standalone-executable` (Portable EXE)
  2. `installer-setup` (NSIS Installer)
- **Branding**: `productName` standardized to `fluyo-by-cm2labs`.

## Result
- **Initialization**: Fully operational.
- **Vault**: Data stored in `storage/`.
- **URL Verification**: Points to `https://app.fluyo.io`.
