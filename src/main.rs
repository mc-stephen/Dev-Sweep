slint::include_modules!();

pub mod modules;
pub mod utils;

use modules::window_management::WindowManagement;

use crate::modules::dashboard::dashboard::DashboardModule;

fn main() -> Result<(), slint::PlatformError> {
    let window: AppWindow = AppWindow::new()?;
    let ui_handle: slint::Weak<AppWindow> = window.as_weak();

    //==========================
    //
    //==========================
    WindowManagement {
        ui_handle: ui_handle.clone(),
        window: window.clone_strong(),
    }
    .setup_callbacks();

    //==========================
    //
    //==========================
    let dashboard_module: DashboardModule = DashboardModule::new(window.clone_strong());
    dashboard_module.update_stats();

    window.run()
}
